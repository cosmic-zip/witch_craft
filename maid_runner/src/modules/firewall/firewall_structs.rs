pub struct SimpleRule<
    'table,
    'chain,
    'protocol,
    'destination_port,
> {
    pub table: &'table  str,
    pub chain: &'chain  str,
    pub protocol: &'protocol  str,
    pub destination_port: &'destination_port  str,
}