use mini_markdown::lexer::Token as MdToken;
use std::io::Write;
use std::io::{self, Stdout};
use termion::event::Key::Char;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct State {
    current_page: usize,

    pages: Vec<Vec<MdToken>>,

    stdin: termion::input::Keys<termion::AsyncReader>,
    stdout: termion::raw::RawTerminal<Stdout>,
}

impl State {
    pub fn new(_file: &str) -> io::Result<State> {
        Ok(State {
            current_page: 0,
            pages: Vec::new(),
            stdin: termion::async_stdin().keys(),
            stdout: io::stdout().into_raw_mode()?,
        })
    }

    fn next_page(&mut self) {
        if self.current_page == self.pages.len() {
            return;
        }
        self.current_page += 1
    }

    fn prev_page(&mut self) {
        if self.current_page == 0 {
            return;
        }
        self.current_page -= 1
    }

    fn draw(&mut self) -> io::Result<()> {
        write!(
            self.stdout,
            "{}{}Key pressed: {:?}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            "hello" //key
        )?;

        self.stdout.lock().flush()
    }

    pub fn term_loop(&mut self) -> io::Result<()> {
        self.draw()?;

        loop {
            if let Some(Ok(key)) = self.stdin.next() {
                match key {
                    Char('q') => break,
                    Char('j') => {
                        self.next_page();
                        self.draw()?;
                    }
                    Char('k') => {
                        self.prev_page();
                        self.draw()?;
                    }
                    _ => {}
                }
            }

            std::thread::sleep(std::time::Duration::from_millis(50));
        }

        Ok(())
    }
}
