use mini_markdown::lexer::Token as MdToken;
use std::io::{self, Stdout, Write};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

mod write_tokens;

pub struct State {
    current_page: usize,
    contents: Vec<MdToken>,
    page_bounds: Vec<usize>,
    stdout: termion::raw::RawTerminal<Stdout>,
}

impl State {
    pub fn new(file: &str) -> io::Result<State> {
        use mini_markdown::lex;

        let contents = std::fs::read_to_string(file).map(move |contents| lex(contents.as_str()))?;

        let page_bounds = contents
            .iter()
            .enumerate()
            .filter_map(|(i, item)| match item {
                MdToken::Header(..) => Some(i),
                _ => None,
            })
            .collect();

        Ok(State {
            current_page: 0,
            contents,
            page_bounds,
            stdout: io::stdout().into_raw_mode()?,
        })
    }

    fn next_page(&mut self) -> bool {
        if self.current_page + 1 == self.page_bounds.len() {
            return false;
        }

        self.current_page += 1;
        true
    }

    fn prev_page(&mut self) -> bool {
        if self.current_page == 0 {
            return false;
        }

        self.current_page -= 1;
        true
    }

    fn redraw(&mut self) -> io::Result<()> {
        let page_end = if self.current_page + 1 == self.page_bounds.len() {
            self.contents.len()
        } else {
            self.page_bounds[self.current_page + 1]
        };

        let page = &self.contents[self.page_bounds[self.current_page]..page_end];

        write_tokens::write(page, &mut self.stdout)?;

        write_tokens::write_aligned(
            &format!("{}/{}", self.current_page + 1, self.page_bounds.len()),
            &mut self.stdout,
            write_tokens::Alignment::Right,
        )?;

        self.stdout.lock().flush()
    }

    pub fn term_loop(&mut self) -> io::Result<()> {
        use termion::event::Key::Char;

        let mut stdin = termion::async_stdin().keys();

        self.redraw()?;

        loop {
            if let Some(Ok(key)) = stdin.next() {
                match key {
                    Char('q') => return Ok(()),
                    Char('j') if self.next_page() => self.redraw()?,
                    Char('k') if self.prev_page() => self.redraw()?,
                    _ => (),
                }
            }

            std::thread::sleep(std::time::Duration::from_millis(50));
        }
    }
}
