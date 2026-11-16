//! Speaks to VectorCraft: starts its MCP server and calls its tools.
//!
//! This module hides how the server is started and how a call travels: `vectorcraft-cli mcp`
//! on the PATH, one line of JSON-RPC each way over its standard input and output. The server
//! is confined to reading one folder and may write nowhere. Nothing here is pinned to a
//! version: [`Server::version`] is what the installed server says of itself when it starts.
//!
//! Every request has [`TIME_LIMIT`] to be answered. A server that does not answer in time, or
//! stops, ends the call with an error that says which request it was and passes on what the
//! server wrote to its error output.

use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use serde_json::{Value, json};

use crate::{Error, Result};

/// The program that is started. It is looked for on the PATH.
pub const PROGRAM: &str = "vectorcraft-cli";

/// What to tell a person when the program is not there.
pub const NOT_INSTALLED: &str = "vectorcraft-cli is not installed: it is not on the PATH. The check was not run. That is neither a pass nor a failure of the armature; run it where VectorCraft is installed.";

/// How long the server may take to answer one request. Opening and reading an armature takes
/// it well under a second; this is generous so that only a server that has hung reaches it.
pub const TIME_LIMIT: Duration = Duration::from_secs(60);

/// How much of the server's error output is kept to be passed on: its last characters.
const ERRORS_KEPT: usize = 4000;

/// How long a server that has closed its output is given to exit and finish writing why.
const STOPPING: Duration = Duration::from_secs(2);

/// A running VectorCraft server.
pub struct Server {
    child: Child,
    stdin: ChildStdin,
    /// The lines of its standard output, as a reader in another thread receives them.
    replies: Receiver<String>,
    /// The end of its error output, as a reader in another thread receives it.
    errors: Arc<Mutex<String>>,
    errors_reader: JoinHandle<()>,
    next_id: u64,
    version: String,
}

impl Server {
    /// Starts the server, allowed to read `read_root` and nothing else, and to write nothing.
    /// Gives `None` when VectorCraft is not installed.
    pub fn start(read_root: &Path) -> Result<Option<Server>> {
        let spawned = Command::new(PROGRAM)
            .arg("mcp")
            .arg("--automation-read-root")
            .arg(read_root)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn();
        let mut child = match spawned {
            Ok(child) => child,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(error) => {
                return Err(Error::new(format!("cannot start {PROGRAM}: {error}")));
            }
        };
        let (Some(stdin), Some(stdout), Some(mut stderr)) =
            (child.stdin.take(), child.stdout.take(), child.stderr.take())
        else {
            return Err(Error::new(format!("cannot speak to {PROGRAM}")));
        };

        let (sender, replies) = mpsc::channel();
        thread::spawn(move || {
            for line in BufReader::new(stdout).lines() {
                let Ok(line) = line else { break };
                if sender.send(line).is_err() {
                    break;
                }
            }
        });
        let errors = Arc::new(Mutex::new(String::new()));
        let kept = Arc::clone(&errors);
        let errors_reader = thread::spawn(move || {
            let mut chunk = [0_u8; 1024];
            while let Ok(read) = stderr.read(&mut chunk) {
                if read == 0 {
                    break;
                }
                if let Ok(mut text) = kept.lock() {
                    text.push_str(&String::from_utf8_lossy(&chunk[..read]));
                    if text.len() > 2 * ERRORS_KEPT {
                        let mut cut = text.len() - ERRORS_KEPT;
                        while !text.is_char_boundary(cut) {
                            cut += 1;
                        }
                        text.drain(..cut);
                    }
                }
            }
        });

        let mut server = Server {
            child,
            stdin,
            replies,
            errors,
            errors_reader,
            next_id: 1,
            version: String::new(),
        };
        let greeting = server.request(
            "initialize",
            json!({
                "protocolVersion": "2025-06-18",
                "capabilities": {},
                "clientInfo": { "name": "maquette", "version": env!("CARGO_PKG_VERSION") },
            }),
            "`initialize`",
        )?;
        server.version = greeting
            .get("serverInfo")
            .and_then(|info| info.get("version"))
            .and_then(Value::as_str)
            .ok_or_else(|| Error::new(format!("{PROGRAM} did not say which version it is")))?
            .to_string();
        server.send(
            &json!({ "jsonrpc": "2.0", "method": "notifications/initialized" }),
            "`notifications/initialized`",
        )?;
        Ok(Some(server))
    }

    /// The version the server gave when it started.
    pub fn version(&self) -> &str {
        &self.version
    }

    /// An error that says what happened to the server, how it ended if it did, and what it
    /// wrote to its error output. The command that meets it exits as a failure: whatever it
    /// was checking was not checked to the end.
    fn failure(&mut self, happened: String, stopped: bool) -> Error {
        let mut ended = None;
        if stopped {
            // A server that closed its output is given a moment to exit and to say why.
            let until = Instant::now() + STOPPING;
            while Instant::now() < until {
                if ended.is_none() {
                    ended = self.child.try_wait().ok().flatten();
                }
                if ended.is_some() && self.errors_reader.is_finished() {
                    break;
                }
                thread::sleep(Duration::from_millis(20));
            }
        }
        let errors = self
            .errors
            .lock()
            .map(|text| text.trim().to_string())
            .unwrap_or_default();
        let mut message = happened;
        if let Some(status) = ended {
            message.push_str(&format!(" ({status})"));
        }
        message.push_str(". The check was not finished, so it neither passed nor was judged.");
        if errors.is_empty() {
            message.push_str(&format!(" {PROGRAM} wrote nothing to its error output."));
        } else {
            message.push_str(&format!(
                " What {PROGRAM} wrote to its error output:\n{errors}"
            ));
        }
        Error::new(message)
    }

    fn send(&mut self, message: &Value, what: &str) -> Result<()> {
        let written = writeln!(self.stdin, "{message}").and_then(|()| self.stdin.flush());
        written.map_err(|error| {
            self.failure(
                format!("{PROGRAM} stopped before {what} could be sent to it ({error})"),
                true,
            )
        })
    }

    /// Sends a request and waits, for at most [`TIME_LIMIT`], for its answer. `what` names
    /// the request for a person.
    fn request(&mut self, method: &str, params: Value, what: &str) -> Result<Value> {
        let id = self.next_id;
        self.next_id += 1;
        self.send(
            &json!({ "jsonrpc": "2.0", "id": id, "method": method, "params": params }),
            what,
        )?;
        let deadline = Instant::now() + TIME_LIMIT;
        loop {
            let left = deadline.saturating_duration_since(Instant::now());
            let line = match self.replies.recv_timeout(left) {
                Ok(line) => line,
                Err(RecvTimeoutError::Timeout) => {
                    return Err(self.failure(
                        format!(
                            "{PROGRAM} gave no answer to {what} within {} seconds",
                            TIME_LIMIT.as_secs()
                        ),
                        false,
                    ));
                }
                Err(RecvTimeoutError::Disconnected) => {
                    return Err(
                        self.failure(format!("{PROGRAM} stopped before it answered {what}"), true)
                    );
                }
            };
            let Ok(message) = serde_json::from_str::<Value>(&line) else {
                continue;
            };
            // Anything that is not the answer to this request is a notification.
            if message.get("id").and_then(Value::as_u64) != Some(id) {
                continue;
            }
            if let Some(error) = message.get("error") {
                return Err(Error::new(format!("{PROGRAM} refused {what}: {error}")));
            }
            return message
                .get("result")
                .cloned()
                .ok_or_else(|| Error::new(format!("{PROGRAM} gave {what} no result")));
        }
    }

    /// Calls one of the server's tools and gives its reply, which is JSON.
    pub fn call(&mut self, tool: &str, arguments: Value) -> Result<Value> {
        let result = self.request(
            "tools/call",
            json!({ "name": tool, "arguments": arguments }),
            &format!("the tool `{tool}`"),
        )?;
        let text = result
            .get("content")
            .and_then(|content| content.get(0))
            .and_then(|first| first.get("text"))
            .and_then(Value::as_str)
            .ok_or_else(|| Error::new(format!("VectorCraft's `{tool}` gave no text")))?;
        if result.get("isError").and_then(Value::as_bool) == Some(true) {
            return Err(Error::new(format!("VectorCraft's `{tool}` failed: {text}")));
        }
        serde_json::from_str(text)
            .map_err(|error| Error::new(format!("VectorCraft's `{tool}` gave no JSON: {error}")))
    }

    /// Runs one of VectorCraft's commands.
    pub fn command(&mut self, command: &str, params: Value) -> Result<Value> {
        self.call(
            "run_command",
            json!({ "command": command, "params": params }),
        )
        .map_err(|error| Error::new(format!("{command}: {error}")))
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        // Nothing is saved, so the server has nothing to finish.
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}
