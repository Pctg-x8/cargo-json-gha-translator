use std::convert::TryFrom;

mod gha;

fn main() {
    let stdin = std::io::stdin();
    let stream = cargo_metadata::Message::parse_stream(stdin.lock())
        .filter_map(|msg| gha::WorkflowCommand::try_from(msg.expect("Failed to read stdin")).ok());
    for cmd in stream {
        println!("{}", cmd);
    }
}
