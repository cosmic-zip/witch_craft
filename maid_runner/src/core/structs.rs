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

pub struct Logger {
    pub source: String,
    pub source_from: String,
    pub source_description: String,
    pub status: String,
    pub stdout: String,
    pub stderr: String,
    pub debug: bool,
}

pub struct ProcessInit<'source, 'source_from, 'source_description> {
    pub source: &'source str,
    pub source_from: &'source_from str,
    pub source_description: &'source_description str,
    pub debug: bool,
}

pub struct ProcessResult<
    'session,
    'session_description,
    'source_from,
    'source_command,
    'source_detail,
    'source_description,
    'timestemp,
    'returned_status,
> {
    pub session: &'session str,
    pub session_description: &'session_description str,
    pub source_from: &'source_from str,
    pub source_command: &'source_command str,
    pub source_detail: &'source_detail str,
    pub source_description: &'source_description str,
    pub timestemp: &'timestemp str,
    pub returned_status: &'returned_status str,
    pub formated_stdout: Vec<String>,
    pub formated_stderr: Vec<String>,
    pub debug: bool,
}

pub struct Backup<'from, 'to, 'technic> {
    pub from: &'from str,
    pub to: &'to str,
    pub technic: &'technic str,
}
