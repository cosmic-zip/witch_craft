-- Your SQL goes here
CREATE TABLE logs(
    id INTEGER PRIMARY KEY NOT NULL,
    session TEXT NOT NULL,
    session_description TEXT NOT NULL,
    source_from TEXT NOT NULL,
    source_command TEXT NOT NULL,
    source_detail TEXT NOT NULL,
    source_description TEXT NOT NULL,
    timestemp TEXT NOT NULL,
    returned_status TEXT NOT NULL,
    formated_stdout TEXT NOT NULL,
    formated_stderr TEXT NOT NULL,
    debug TEXT NOT NULL
);
