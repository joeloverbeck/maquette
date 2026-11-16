//! The read-back against a stand-in for VectorCraft's server that fails, through the command
//! line.
//!
//! This test file is its own stand-in: it has no test harness, and when it is started under
//! the name `vectorcraft-cli` it behaves as the server it is told to be by
//! `MAQUETTE_STAND_IN`, which the program passes on to it. Otherwise it copies itself into a
//! scratch folder under that name, puts that folder alone on the PATH and runs the read-back.
//!
//! - `stops`: the server writes why to its error output and exits before it answers.
//! - `silent`: the server answers `initialize` and then never answers again. The read-back
//!   waits out its time limit, a minute, so this case runs only with `-- --ignored`.

mod support;

use std::io::{BufRead, Write};
use std::path::PathBuf;
use std::process::Output;

const STAND_IN: &str = "MAQUETTE_STAND_IN";
const WHY: &str = "the stand-in cannot open a display";

fn main() {
    let started_as = std::env::args().next().unwrap_or_default();
    if started_as.ends_with("vectorcraft-cli") {
        stand_in(&std::env::var(STAND_IN).unwrap_or_default());
        return;
    }
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let slow = arguments
        .iter()
        .any(|argument| argument == "--ignored" || argument == "--include-ignored");
    let filters: Vec<&String> = arguments
        .iter()
        .filter(|argument| !argument.starts_with('-'))
        .collect();
    let tests: [(&str, bool, fn()); 2] = [
        (
            "a_server_that_stops_before_it_answers_says_why_and_is_not_a_pass",
            false,
            a_server_that_stops_before_it_answers_says_why_and_is_not_a_pass,
        ),
        (
            "a_server_that_does_not_answer_in_time_is_given_up_on_with_a_message",
            true,
            a_server_that_does_not_answer_in_time_is_given_up_on_with_a_message,
        ),
    ];
    for (name, takes_a_minute, test) in tests {
        let chosen =
            filters.is_empty() || filters.iter().any(|filter| name.contains(filter.as_str()));
        if !chosen {
            continue;
        }
        if takes_a_minute && !slow {
            println!("test {name} ... ignored, it waits out the time limit");
            continue;
        }
        test();
        println!("test {name} ... ok");
    }
}

/// Behaves as a server that fails in the way it is told.
fn stand_in(behaviour: &str) {
    match behaviour {
        "stops" => {
            eprintln!("{WHY}");
            std::process::exit(3);
        }
        "silent" => {
            let stdin = std::io::stdin();
            for line in stdin.lock().lines() {
                let Ok(line) = line else { break };
                if line.contains("\"initialize\"") {
                    let answer = r#"{"jsonrpc":"2.0","id":1,"result":{"serverInfo":{"name":"stand-in","version":"0.0.0"}}}"#;
                    let mut stdout = std::io::stdout();
                    let _ = writeln!(stdout, "{answer}");
                    let _ = stdout.flush();
                    eprintln!("the stand-in will answer nothing more");
                }
            }
            // Its input is closed, and it still answers nothing.
            loop {
                std::thread::park();
            }
        }
        other => {
            eprintln!("no stand-in behaviour `{other}`");
            std::process::exit(4);
        }
    }
}

/// A folder holding only the stand-in, under the name the program looks for.
fn stand_in_folder(name: &str) -> PathBuf {
    let folder = support::scratch(name).join("bin");
    std::fs::create_dir_all(&folder).expect("the folder can be made");
    let me = std::env::current_exe().expect("this program's path");
    std::fs::copy(me, folder.join("vectorcraft-cli")).expect("the stand-in is copied");
    folder
}

/// Runs the read-back on a freshly drawn view against a stand-in that behaves as told.
fn read_back_against(behaviour: &str) -> Output {
    let path = stand_in_folder(&format!("stand-in-{behaviour}"));
    let drawn = support::draw(&format!("stand-in-{behaviour}-view"), support::MADE_UP, &[]);
    let landmarks = drawn.join("three-quarter.landmarks.toml");
    // SAFETY: this program is single-threaded here; the variable passes to the child.
    unsafe { std::env::set_var(STAND_IN, behaviour) };
    support::maquette_with_path(
        &["read-back", "--landmarks", support::path_text(&landmarks)],
        Some(path.as_os_str()),
    )
}

fn a_server_that_stops_before_it_answers_says_why_and_is_not_a_pass() {
    let ran = read_back_against("stops");
    let message = String::from_utf8_lossy(&ran.stderr);
    // Not a pass, and not "not installed": the server was there and failed.
    assert_eq!(ran.status.code(), Some(1), "{message}");
    assert!(!message.contains("not installed"), "{message}");
    // Whether it stopped before the request reached it or before it answered, it names it.
    assert!(
        message.contains("vectorcraft-cli stopped before"),
        "{message}"
    );
    assert!(message.contains("`initialize`"), "{message}");
    // What the server wrote to its error output is passed on, with how it ended.
    assert!(message.contains(WHY), "{message}");
    assert!(message.contains("exit status: 3"), "{message}");
    assert!(ran.stdout.is_empty(), "nothing is printed as if read back");
}

fn a_server_that_does_not_answer_in_time_is_given_up_on_with_a_message() {
    let started = std::time::Instant::now();
    let ran = read_back_against("silent");
    let message = String::from_utf8_lossy(&ran.stderr);
    assert_eq!(ran.status.code(), Some(1), "{message}");
    assert!(
        message.contains("gave no answer to the tool `open_file` within 60 seconds"),
        "{message}"
    );
    assert!(message.contains("will answer nothing more"), "{message}");
    assert!(started.elapsed().as_secs() < 90, "it waited too long");
}
