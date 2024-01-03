use super::*;
use crate::modules::firewall::firewall_structs::*;
use crate::*;

// #[test]
// fn test_firewall_preset() {
//     let debug = true;
//     let option = "hardened";
//     let output = firewall_preset(option, debug);
//     assert_eq!(output, true);
// }

// #[test]
fn test_firewall_rules_ssh() {
    let debug = true;
    let command = NfTableRule {
        protocol: "tcp",
        action: "accept",
        port: "22",
    };

    let output = firewall(command, debug);

    assert_eq!(output, true);
}

// #[test]
fn test_firewall_rules_port_8000_tcp() {
    let debug = true;
    let command = NfTableRule {
        protocol: "tcp",
        action: "accept",
        port: "8000",
    };

    let output = firewall(command, debug);

    assert_eq!(output, true);
}

// #[test]
fn test_firewall_rules_port_8000_udp() {
    let debug = true;
    let command = NfTableRule {
        protocol: "udp",
        action: "accept",
        port: "8000",
    };

    let output = firewall(command, debug);

    assert_eq!(output, true);
}
