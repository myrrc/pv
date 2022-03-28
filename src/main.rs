use std::io;
use crate::State;

//fn read_and_parse_file(file_name: &str) {
//    fs::read_to_string(file_name).map(|contents| lex(contents))?;
//
//    let tokens: Vec<MdToken> = lex(markdown);
//}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    assert!(
        args.len() == 2,
        "Expected one argument: path to markdown presentation"
    );

    let mut state = State::new(&args[1])?;

    state.term_loop();

    Ok(())
}
