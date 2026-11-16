//! Speaks to VectorCraft: starts its MCP server and calls its tools.
//!
//! This module hides how the server is started and how a call travels: `vectorcraft-cli mcp`
//! on the PATH, one line of JSON-RPC each way over its standard input and output. The server
//! is confined to reading one folder and may write nowhere. Nothing here is pinned to a
//! version: [`Server::version`] is what the installed server says of itself when it starts.

use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

use serde_json::{Value, json};

use crate::{Error, Result};

/// The program that is started. It is looked for on the PATH.
pub const PROGRAM: &str = "vectorcraft-cli";

/// What to tell a person when the program is not there.
pub const NOT_INSTALLED: &str = "vectorcraft-cli is not installed: it is not on the PATH. The check was not run. That is neither a pass nor a failure of the armature; run it where VectorCraft is installed.";

/// A running VectorCraft server.
pub struct Server {
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
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
            .stderr(Stdio::null())
            .spawn();
        let mut child = match spawned {
            Ok(child) => child,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(error) => {
                return Err(Error::new(format!("cannot start {PROGRAM}: {error}")));
            }
        };
        let (Some(stdin), Some(stdout)) = (child.stdin.take(), child.stdout.take()) else {
            return Err(Error::new(format!("cannot speak to {PROGRAM}")));
        };
        let mut server = Server {
            child,
            stdin,
            stdout: BufReader::new(stdout),
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
        )?;
        server.version = greeting
            .get("serverInfo")
            .and_then(|info| info.get("version"))
            .and_then(Value::as_str)
            .ok_or_else(|| Error::new(format!("{PROGRAM} did not say which version it is")))?
            .to_string();
        server.send(&json!({ "jsonrpc": "2.0", "method": "notifications/initialized" }))?;
        Ok(Some(server))
    }

    /// The version the server gave when it started.
    pub fn version(&self) -> &str {
        &self.version
    }

    fn send(&mut self, message: &Value) -> Result<()> {
        writeln!(self.stdin, "{message}")
            .and_then(|()| self.stdin.flush())
            .map_err(|error| Error::new(format!("cannot write to {PROGRAM}: {error}")))
    }

    fn request(&mut self, method: &str, params: Value) -> Result<Value> {
        let id = self.next_id;
        self.next_id += 1;
        self.send(&json!({ "jsonrpc": "2.0", "id": id, "method": method, "params": params }))?;
        loop {
            let mut line = String::new();
            let read = self
                .stdout
                .read_line(&mut line)
                .map_err(|error| Error::new(format!("cannot read from {PROGRAM}: {error}")))?;
            if read == 0 {
                return Err(Error::new(format!(
                    "{PROGRAM} stopped before it answered `{method}`"
                )));
            }
            let Ok(message) = serde_json::from_str::<Value>(&line) else {
                continue;
            };
            // Anything that is not the answer to this request is a notification.
            if message.get("id").and_then(Value::as_u64) != Some(id) {
                continue;
            }
            if let Some(error) = message.get("error") {
                return Err(Error::new(format!("{PROGRAM} refused `{method}`: {error}")));
            }
            return message
                .get("result")
                .cloned()
                .ok_or_else(|| Error::new(format!("{PROGRAM} gave `{method}` no result")));
        }
    }

    /// Calls one of the server's tools and gives its reply, which is JSON.
    pub fn call(&mut self, tool: &str, arguments: Value) -> Result<Value> {
        let result = self.request(
            "tools/call",
            json!({ "name": tool, "arguments": arguments }),
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
