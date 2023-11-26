#[derive(Debug)]
struct IptablesRule {
    table: String,
    chain: String,
    protocol: Option<String>,
    destination_port: Option<u16>,
    source_ip: Option<String>,
    target: String,
}

impl IptablesRule {
    fn new(table: &str, chain: &str, target: &str) -> Self {
        IptablesRule {
            table: table.to_string(),
            chain: chain.to_string(),
            protocol: None,
            destination_port: None,
            source_ip: None,
            target: target.to_string(),
        }
    }

    fn with_protocol(mut self, protocol: &str) -> Self {
        self.protocol = Some(protocol.to_string());
        self
    }

    fn with_destination_port(mut self, port: u16) -> Self {
        self.destination_port = Some(port);
        self
    }

    fn with_source_ip(mut self, source_ip: &str) -> Self {
        self.source_ip = Some(source_ip.to_string());
        self
    }

}
