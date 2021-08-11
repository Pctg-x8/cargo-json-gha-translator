#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "kebab-case", tag = "reason")]
pub enum BuildEngineMessage {
    CompilerMessage { message: CompilerMessage },
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum MessageLevel {
    Warning,
    Error
}

#[derive(serde::Deserialize, Debug)]
pub struct CompilerMessage {
    pub level: MessageLevel,
    pub message: String,
    pub spans: Vec<Span>
}

#[derive(serde::Deserialize, Debug)]
pub struct Span {
    pub file_name: String,
    pub line_start: usize,
    pub column_start: usize,
}
