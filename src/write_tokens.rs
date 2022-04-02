use mini_markdown::lexer::Token::{self, *};
use std::io;

use termion::color::{self, Bg};
use termion::cursor::Goto;
use termion::style::{Bold as StyleBold, CrossedOut, Italic as StyleItalic, Reset};

pub enum Alignment {
    Center,
    Right,
}

pub fn write_aligned(text: &str, stdout: &mut impl io::Write, align: Alignment) -> io::Result<()> {
    let width = termion::terminal_size()?.0 as usize;

    match align {
        Alignment::Center => write!(stdout, "{:^1$}", text, width),
        Alignment::Right => write!(stdout, "{:>1$}", text, width)
    }
}

fn write_token(token: &Token, stdout: &mut impl io::Write) -> io::Result<()> {
    // corner case: in WSL terminal we get \n but need \r\n, can't use writeln due to that.
    let newline: &str = "\r\n";

    match token {
        Newline => write!(stdout, "{}", newline)?,

        Header(level, text, _) => {
            if level == &1 {
                write_aligned(text, stdout, Alignment::Center)?;
            } else {
                write!(stdout, "{}", text)?;
            }
        }

        Plaintext(text) => write!(stdout, "{}", text.replace("\n", "\r\n"))?,

        Italic(text) => write!(stdout, "{}{}{}", StyleItalic, text, Reset)?,
        Bold(text) => write!(stdout, "{}{}{}", StyleBold, text, Reset)?,
        BoldItalic(text) => write!(stdout, "{}{}{}{}", StyleBold, StyleItalic, text, Reset)?,
        Strikethrough(text) => write!(stdout, "{}{}{}", CrossedOut, text, Reset)?,

        UnorderedListEntry(text) | OrderedListEntry(text) => {
            write!(stdout, "* {}{}", text, newline)?
        }

        Code(text) => write!(stdout, "{}{}{}", Bg(color::Green), text, Bg(color::Reset))?,
        //CodeBlock(text, lang) => todo!(),

        BlockQuote(.., text) => write!(stdout, "> {}{}", text, newline)?,

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

pub fn write(tokens: &[Token], stdout: &mut impl io::Write) -> io::Result<()> {
    write!(stdout, "{}{}", termion::clear::All, Goto(1, 1))?;

    for token in tokens {
        write_token(token, stdout)?;
    }

    Ok(())
}
