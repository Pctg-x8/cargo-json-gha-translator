use std::{convert::TryFrom, io::BufRead};

mod cargo;
mod gha;

fn main() {
    let input = std::io::stdin();
    let input_lines = input.lock().lines().filter_map(|l| {
        l.map(|l| {
            serde_json::from_str::<cargo::jsonfmt::BuildEngineMessage>(&l)
                .ok()
                .and_then(|e| gha::annotation::WorkflowCommand::try_from(e).ok())
        })
        .transpose()
    });

    let mut found_error = false;
    for cmd in input_lines {
        let cmd = cmd.expect("Failed to read stdin");
        found_error = found_error || cmd.is_error();
        println!("{}", cmd);
    }

    std::process::exit(if found_error { 101 } else { 0 });
}
