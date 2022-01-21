use anyhow::Result;
use std::{env, io::stdin};
fn main() -> Result<()> {
    let stdin = stdin();
    let program_name = if let Some(name) = env::args().nth(1) {
        format!(" [{}] ", name)
    } else {
        String::from(" ")
    };
    let mut line = String::new();
    loop {
        line.clear();
        if stdin.read_line(&mut line)? == 0 {
            break;
        }
        print!(
            "[{timestamp}]{program_name}{input_str}",
            timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S"),
            program_name = program_name,
            input_str = line
        );
    }
    Ok(())
}
