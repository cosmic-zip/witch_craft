use super::*;
use crate::*;

// #[test]
// fn test_scanner_web() {
//     // suported types:
//     //      ip
//     //      whois
//     //      routes
//     //      dns
//     //      nmap_tcp
//     //      nmap_udp
//     //      sub_domains
//     //      sub_directory
//     //      build_with
//     //      c_api_url
//     //      c_web_url

//     let output_ip = scanner_web(
//         ScannerWebGenericInput {
//             target: "1.1.1.1",
//             op_type: "ip",
//             list_path: "none",
//         },
//         true,
//     );
//     assert_eq!(output_ip, true);

//     let output_routes = scanner_web(
//         ScannerWebGenericInput {
//             target: "1.1.1.1",
//             op_type: "routes",
//             list_path: "none",
//         },
//         true,
//     );
//     assert_eq!(output_routes, true);
// }

#[test]
fn test_scanner_autonmap_tcp_syn() {
    let output = scanner_auto_nmap(
        ScannerWebAutoNmap {
            target: "localhost",
            delay: "fast",
            ports: "auto",
            scan_type: "tcp_syn",
        },
        true,
    );
    assert_eq!(output, true);
}

#[test]
fn test_scanner_autonmap_tcp_ack() {
    let output = scanner_auto_nmap(
        ScannerWebAutoNmap {
            target: "localhost",
            delay: "fast",
            ports: "auto",
            scan_type: "tcp_ack",
        },
        true,
    );
    assert_eq!(output, true);
}

#[test]
fn test_scanner_autonmap_udp() {
    let output = scanner_auto_nmap(
        ScannerWebAutoNmap {
            target: "localhost",
            delay: "fast",
            ports: "auto",
            scan_type: "udp",
        },
        true,
    );
    assert_eq!(output, true);
}

//**Test 1: IP Address**
#[test]
fn test_scanner_web_ip() {
    let output = scanner_web(
        ScannerWebGenericInput {
            target: "1.1.1.1",
            op_type: "ip",
            list_path: "none",
        },
        true,
    );
    assert_eq!(output, true);
}

//**Test 2: WHOIS Information**

#[test]
fn test_scanner_web_whois() {
    let output = scanner_web(
        ScannerWebGenericInput {
            target: "example.com",
            op_type: "whois",
            list_path: "none",
        },
        true,
    );
    assert_eq!(output, true);
}

// **Test 3: Routes Information**

#[test]
fn test_scanner_web_routes() {
    let output = scanner_web(
        ScannerWebGenericInput {
            target: "1.1.1.1",
            op_type: "routes",
            list_path: "none",
        },
        true,
    );
    assert_eq!(output, true);
}

// **Test 4: DNS Information**

#[test]
fn test_scanner_web_dns() {
    let output = scanner_web(
        ScannerWebGenericInput {
            target: "example.com",
            op_type: "dns",
            list_path: "none",
        },
        true,
    );
    assert_eq!(output, true);
}

// **Test 5: Nmap TCP Syn**

#[test]
fn test_scanner_web_nmap_tcp() {
    let output = scanner_web(
        ScannerWebGenericInput {
            target: "localhost",
            op_type: "nmap_tcp",
            list_path: "none",
        },
        true,
    );
    assert_eq!(output, true);
}

// **Test 6: Nmap UDP**

#[test]
fn test_scanner_web_nmap_udp() {
    let output = scanner_web(
        ScannerWebGenericInput {
            target: "localhost",
            op_type: "nmap_udp",
            list_path: "none",
        },
        true,
    );
    assert_eq!(output, true);
}

// **Test 7: Sub-Domains**

#[test]
fn test_scanner_web_sub_domains() {
    let path = read_meow("/var/witch_craft/witch_spells/embedded/config.meow", false);
    let final_path = format!("{}{}", path["TEST_BASE_PATH"], path["TEST_SDNS"]);

    let output = scanner_web(
        ScannerWebGenericInput {
            target: "example.com",
            op_type: "sub_domains",
            list_path: &final_path,
        },
        true,
    );
    assert_eq!(output, true);
}

// **Test 8: Sub-Directories**

#[test]
fn test_scanner_web_sub_directories() {
    let path = read_meow("/var/witch_craft/witch_spells/embedded/config.meow", false);
    let final_path = format!("{}{}", path["TEST_BASE_PATH"], path["TEST_DIRECTORIES"]);

    let output = scanner_web(
        ScannerWebGenericInput {
            target: "example.com",
            op_type: "sub_directories",
            list_path: &final_path,
        },
        true,
    );
    assert_eq!(output, true);
}

// **Test 9: Build with**

#[test]
fn test_scanner_web_build() {
    let output = scanner_web(
        ScannerWebGenericInput {
            target: "example.com",
            op_type: "build_with",
            list_path: "none",
        },
        true,
    );
    assert_eq!(output, true);
}

// **Test 10: C API URL**

#[test]
fn test_scanner_web_c_api() {
    let output = scanner_web(
        ScannerWebGenericInput {
            target: "example.com",
            op_type: "c_api_url",
            list_path: "none",
        },
        true,
    );
    assert_eq!(output, true);
}

// **Test 11: C WEB URL**

#[test]
fn test_scanner_web_c_url() {
    let output = scanner_web(
        ScannerWebGenericInput {
            target: "example.com",
            op_type: "c_web_url",
            list_path: "none",
        },
        true,
    );
    assert_eq!(output, true);
}
