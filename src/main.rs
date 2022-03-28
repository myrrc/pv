use std::fs;
use std::io;
use std::io::Write;
use std::thread;
use std::time;

use termion;
use termion::event::Key::Char;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use mini_markdown::lex;
use mini_markdown::lexer::Token as MdToken;

fn read_and_parse_file(file_name: &str) {
    fs::read_to_string(file_name).map(|contents| lex(contents))?;

    let tokens: Vec<MdToken> = lex(markdown);
}

//    write!(
//                        stdout,
//                        "{}{}Key pressed: {:?}",
//                        termion::clear::All,
//                        termion::cursor::Goto(1, 1),
//                        key
//                    )
//                    .unwrap();
//
//                    stdout.lock().flush().unwrap();

fn main() -> io::Result<()> {
    let mut stdout = io::stdout().into_raw_mode()?;
    let mut stdin = termion::async_stdin().keys();

    loop {
        if let Some(Ok(key)) = stdin.next() {
            match key {
                Char('q') => break,
                Char('j') => break,
                Char('k') => break,
                _ => { }
            }
        }

        thread::sleep(time::Duration::from_millis(50));
    }

    Ok(())
}
