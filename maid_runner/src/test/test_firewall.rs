use super::*;
use crate::*;
use crate::modules::firewall::firewall_structs::*;

#[test]
fn test_firewall_preset() {
    let debug = true;
    let option = "hardned";
    let output = firewall_preset(option, debug); 
    assert_eq!(output, true);
}


#[test]
fn test_firewall_rules_ssh() {
    let debug = true;
    let command = SimpleRule {
        table: "ACCEPT",
        chain: "INPUT",
        protocol: "tcp",
        destination_port: "22",
    };

   let  output = firewall(command, debug);

    assert_eq!(output, true);
}

#[test]
fn test_firewall_rules_port_8000() {
    let debug = true;
    let command = SimpleRule {
        table: "ACCEPT",
        chain: "INPUT",
        protocol: "tcp",
        destination_port: "8000",
    };

    let output = firewall(command, debug);

    assert_eq!(output, true);
}
