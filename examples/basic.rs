use std::fs::read_to_string;

use markdown_split::split;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let markdown_file = read_to_string("tests/fixtures/ch01-01-installation.en.md")?;

    split(&markdown_file, None)?
        .iter()
        .enumerate()
        .for_each(|(i, s)| println!("Section {i}\n---------\n{s}\n---------"));

    Ok(())
}
