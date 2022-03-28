use mini_markdown::lexer::Token as MdToken;
use std::io::Write;
use std::io::{self, Stdout};
use termion::event::Key::Char;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use mini_markdown::lex;

struct PageIter<I> {
    iter: I
}

impl<I: Iterator> Iterator for PageIter<I> {
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Vec<I::Item>> {
        let not_p = |x: &I::Item| !p(x);

        let mut iter = self.iter.by_ref();

        iter.take_while(

        match iter.next() {
            None    => None,
            Some(x) => {
                let mut v = Vec::new();
                v.push(x);
                v.extend(i);
                Some(v)
            }
        }
    }
}

pub struct State {
    current_page: usize,
    pages: Vec<Vec<MdToken>>,

    stdout: termion::raw::RawTerminal<Stdout>,
}

impl State {
    pub fn new(file: &str) -> io::Result<State> {
        let parsed_contents = std::fs::read_to_string(file)
            .map(|contents| lex(contents))?;

        let contents_iter = PageIter { iter: parsed_contents.into_iter() };

        Ok(State {
            current_page: 0,
            pages: contents_iter.collect(),
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
        let stdin = termion::async_stdin().keys();

        self.draw()?;

        loop {
            if let Some(Ok(key)) = stdin.next() {
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
