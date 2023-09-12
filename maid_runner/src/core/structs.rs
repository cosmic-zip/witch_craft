pub struct CommandCall<'a, 'b> {
    pub command: &'a str,
    pub args: &'b [String],
}

pub struct CommandResult {
    pub status: String,
    pub stdout: String,
    pub stderr: String,
}

pub struct CoreGenericPathOpType<'s, 'o> {
    pub sample_path: &'s str,
    pub op_type: &'o str,
}

pub struct CoreGenericUrl<'u> {
    pub url: &'u str,
}
