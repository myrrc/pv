mod write_tokens;
mod state;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    assert!(args.len() == 2, "Usage: pv my_presentation.md");

    let mut state = state::State::new(&args[1])?;

    state.term_loop()
}
