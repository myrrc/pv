use pv::State;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    assert!(
        args.len() == 2,
        "Expected one argument: path to markdown presentation"
    );

    let mut state = State::new(&args[1])?;

    state.term_loop()
}
