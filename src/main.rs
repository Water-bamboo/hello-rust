
use std::sync::{Arc, RwLock, Mutex};
use std::thread;
use std::borrow::Cow::{self, Borrowed, Owned};
use std::sync::mpsc;

use rustyline::completion::{Completer, FilenameCompleter, Pair};
use rustyline::config::OutputStreamType;
use rustyline::error::ReadlineError;
use rustyline::highlight::{Highlighter, MatchingBracketHighlighter};
use rustyline::hint::Hinter;
use rustyline::{CompletionType, Config, EditMode, Editor, Helper};

mod lib;
use lib::do_command;

pub const COLORED_PROMPT: &'static str = "\x1b[36mwallet713>\x1b[0m ";
pub const PROMPT: &'static str = "wallet713> ";
//static commandLineStatic: &'static str = "";
//hello = String::from("Hello, ");




struct EditorHelper(FilenameCompleter, MatchingBracketHighlighter);

impl Completer for EditorHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
    ) -> std::result::Result<(usize, Vec<Pair>), ReadlineError> {
        self.0.complete(line, pos)
    }
}

impl Hinter for EditorHelper {
    fn hint(&self, _line: &str, _pos: usize) -> Option<String> {
        None
    }
}

impl Highlighter for EditorHelper {
    fn highlight_prompt<'p>(&self, prompt: &'p str) -> Cow<'p, str> {
        if prompt == PROMPT {
            Borrowed(COLORED_PROMPT)
        } else {
            Borrowed(prompt)
        }
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Owned("\x1b[1m".to_owned() + hint + "\x1b[m")
    }

    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        self.1.highlight(line, pos)
    }

    fn highlight_char(&self, line: &str, pos: usize) -> bool {
        self.1.highlight_char(line, pos)
    }
}

impl Helper for EditorHelper {}

fn main() {
    println!("main>>>>!");

    println!("<<<< loop <<<<");

    let editor_config = Config::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::List)
        .edit_mode(EditMode::Emacs)
        .output_stream(OutputStreamType::Stdout)
        .build();
    let mut rl = Editor::with_config(editor_config);
    rl.set_helper(Some(EditorHelper(
        FilenameCompleter::new(),
        MatchingBracketHighlighter::new(),
    )));

    loop {
        println!("main>>>will readline>>>");

        let command = rl.readline(PROMPT);
        println!("main>>>readline: {:?}>>>", command);
        match command {
            Ok(command) => {
                let mut commandValue = command.trim();
                if (commandValue == "") {
                    println!("<<continue>>");
                    continue;
                }

                if commandValue == "exit" {
                    break;
                }

                let mut out_is_safe = false;

//                thread::spawn(move || {
                    let result = do_command(
                        &commandValue,
                        &mut out_is_safe,
                    );
//                }).join().unwrap();


//                if out_is_safe {
//                    rl.add_history_entry(command);
//                }
            }
            Err(_) => {
                println!("Err>>>");
                break;
            }
        }
    }


    println!("<<<< exit <<<<");
}
