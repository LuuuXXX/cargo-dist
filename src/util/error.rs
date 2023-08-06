pub type CliResult = Result<(), CliError>;

pub struct CliError {
    pub error: Option<anyhow::Error>,
    pub exit_code: i32,
}