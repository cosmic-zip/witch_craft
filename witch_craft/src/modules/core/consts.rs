use std::collections::HashMap;

const MAGIC_DOCS: &[(&str, &str)]  = &[
        ("dns", "set domain name"),
        ("target", "set IP or domain name"),
        ("ip", "set IP address"),
        ("path", "set file path"),
        ("file", "set file location"),
        ("wordlist", "path to wordlist"),
        ("port", "set port number"),
        ("output", "set output file"),
        ("protocol", "set communication protocol"),
        ("timeout", "set timeout duration"),
        ("wait", "set delay duration"),
        ("verbose", "enable verbose mode"),
        ("overwrite", "overwrite existing files"),
        ("recursive", "enable recursive mode"),
        ("url", "target complete URL path with http/https"),
        ("username", "specify username; if it contains spaces, use quotes like: \"user name with\""),
        ("password", "specify password; it will be shown in plaintext"),
];
