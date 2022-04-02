use mini_markdown::lexer::Token::{self, *};
use std::io;
use termion::color::{self, Bg};
use termion::style::{Bold as StyleBold, CrossedOut, Italic as StyleItalic, Reset};

pub enum Alignment {
    Left,
    Center,
    Right,
}

pub fn write_str(text: &str, stdout: &mut impl io::Write, align: Alignment) -> io::Result<()> {
    let width = termion::terminal_size()?.0 as usize;

    match align {
        Alignment::Left => write!(stdout, "{}", text),
        Alignment::Center => write!(stdout, "{:^1$}", text, width),
        Alignment::Right => write!(stdout, "{:>1$}", text, width),
    }
}

// corner case: in WSL terminal we get \n but need \r\n, can't use writeln due to that.
const NEWLINE: &str = "\r\n";

pub fn write_token(token: &Token, stdout: &mut impl io::Write) -> io::Result<()> {
    match token {
        Newline => write!(stdout, "{}", NEWLINE)?,

        Header(level, text, _) => {
            if level == &1 {
                write_str(text, stdout, Alignment::Center)?;
            } else {
                write!(stdout, "{}", text)?;
            }

            write!(stdout, "{}", NEWLINE)?;
        }

        Plaintext(text) => write!(stdout, "{}", text.replace('\n', NEWLINE))?,

        Italic(text) => write!(stdout, "{}{}{}", StyleItalic, text, Reset)?,
        Bold(text) => write!(stdout, "{}{}{}", StyleBold, text, Reset)?,
        BoldItalic(text) => write!(stdout, "{}{}{}{}", StyleBold, StyleItalic, text, Reset)?,
        Strikethrough(text) => write!(stdout, "{}{}{}", CrossedOut, text, Reset)?,

        UnorderedListEntry(text) | OrderedListEntry(text) => {
            write!(stdout, "* {}{}", text, NEWLINE)?
        }

        Code(text) => write!(stdout, "{}{}{}", Bg(color::Green), text, Bg(color::Reset))?,
        //CodeBlock(text, lang) => todo!(),
        BlockQuote(.., text) => write!(stdout, "> {}{}", text, NEWLINE)?,

        //Image(_, _) => todo!(),
        //Link(_, _, _) => todo!(),
        //Detail(_, _) => todo!(),
        //Table(_, _) => todo!(),
        //TaskListItem(_, _) => todo!(),
        //Footnote(_, _) => todo!(),
        _ => (),
    }

    Ok(())
}
