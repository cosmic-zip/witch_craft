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
    pub id: i32,
    pub session: &'session str,
    pub session_description: &'session_description str,
    pub source_from: &'source_from str,
    pub source_command: &'source_command str,
    pub source_detail: &'source_detail str,
    pub source_description: &'source_description str,
    pub timestemp: &'timestemp str,
    pub returned_status: &'returned_status str,
    pub formatted_stdout: Vec<String>,
    pub formatted_stderr: Vec<String>,
    pub debug: i32,
}

impl ProcessResult<'_, '_, '_, '_, '_, '_, '_, '_> {
    // Function to serialize the struct to a string
    pub fn to_string(&self) -> String {
        format!(
            "id: {}, session: {}, session_description: {}, source_from: {}, source_command: {}, \
            source_detail: {}, source_description: {}, timestemp: {}, returned_status: {}, debug: {}",
            self.id,
            self.session,
            self.session_description,
            self.source_from,
            self.source_command,
            self.source_detail,
            self.source_description,
            self.timestemp,
            self.returned_status,
            self.debug
        )
    }
}