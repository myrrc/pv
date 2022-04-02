use mini_markdown::lex;
use mini_markdown::lexer::Token as MdToken;
use std::io::Write;
use std::io::{self, Stdout};
use termion::cursor::Goto;
use termion::event::Key::Char;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::style::{Bold, NoBold, Italic, NoItalic, CrossedOut, NoCrossedOut};

pub struct State {
    current_page: usize,

    contents: Vec<MdToken>,
    page_bounds: Vec<usize>,

    stdout: termion::raw::RawTerminal<Stdout>,
}

impl State {
    pub fn new(file: &str) -> io::Result<State> {
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

    fn pages(&self) -> usize {
        self.page_bounds.len()
    }

    fn page_bounds(&self) -> (usize, usize) {
        let start = self.page_bounds[self.current_page];

        let end = if self.current_page + 1 == self.pages() {
            self.contents.len()
        } else {
            self.page_bounds[self.current_page + 1]
        };

        (start, end)
    }

    fn next_page(&mut self) {
        if self.current_page + 1 == self.pages() {
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

    fn draw_footer(&mut self) -> io::Result<()> {
        let (width, height) = termion::terminal_size()?;
        let page_str = format!("{}/{}", self.current_page + 1, self.pages());

        write!(
            self.stdout,
            "{}{}",
            Goto(width - page_str.len() as u16, height),
            page_str
        )
    }

    fn draw(&mut self) -> io::Result<()> {
        let (start, end) = self.page_bounds();
        let page = &self.contents[start..end];

        write!(self.stdout, "{}{}", termion::clear::All, Goto(1, 1))?;

        for token in page {
            match token {
                MdToken::Header(_, text, _) => write!(self.stdout, "{}", text)?,
                MdToken::Plaintext(text) => write!(self.stdout, "{}", text)?,
                MdToken::UnorderedListEntry(_) => todo!(),
                MdToken::OrderedListEntry(_) => todo!(),
                MdToken::Italic(text) => write!(self.stdout, "{}{}{}", Italic, text, NoItalic)?,
                MdToken::Bold(text) => write!(self.stdout, "{}{}{}", Bold, text, NoBold)?,
                MdToken::BoldItalic(_) => todo!(),
                MdToken::Strikethrough(text) => write!(self.stdout, "{}{}{}", CrossedOut, text, NoCrossedOut)?,
                MdToken::LineBreak => todo!(),
                MdToken::Newline => todo!(),
                MdToken::HorizontalRule => todo!(),
                MdToken::Tab => todo!(),
                MdToken::DoubleTab => todo!(),
                MdToken::Code(_) => todo!(),
                MdToken::CodeBlock(_, _) => todo!(),
                MdToken::BlockQuote(_, _) => todo!(),
                MdToken::Image(_, _) => todo!(),
                MdToken::Link(_, _, _) => todo!(),
                MdToken::Detail(_, _) => todo!(),
                MdToken::Table(_, _) => todo!(),
                MdToken::TaskListItem(_, _) => todo!(),
                MdToken::Footnote(_, _) => todo!(),
            }
        }

        self.draw_footer()?;
    
        self.stdout.lock().flush()
    }

    pub fn term_loop(&mut self) -> io::Result<()> {
        let mut stdin = termion::async_stdin().keys();

        self.draw()?;

        loop {
            if let Some(Ok(key)) = stdin.next() {
                match key {
                    Char('q') => return Ok(()),
                    Char('j') => {
                        self.next_page();
                        self.draw()?;
                    }
                    Char('k') => {
                        self.prev_page();
                        self.draw()?;
                    }
                    _ => (),
                }
            }

            std::thread::sleep(std::time::Duration::from_millis(50));
        }
    }
}
