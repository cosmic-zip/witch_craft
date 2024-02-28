pub struct CommandCall<'a, 'b> {
    pub command: &'a str,
    pub args: &'b [String],
}

pub struct CommandResult {
    pub status: String,
    pub stdout: String,
    pub stderr: String,
}

pub struct ProcessInit<'source, 'source_from, 'source_description> {
    pub source: &'source str,
    pub source_from: &'source_from str,
    pub source_description: &'source_description str,
    pub debug: bool,
}
