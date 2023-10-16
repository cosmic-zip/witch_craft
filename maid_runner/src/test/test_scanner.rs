use super::*;
use crate::*;

#[test]
fn test_scanner_web() {
    // suported types:
    //      ip
    //      whois
    //      routes
    //      dns
    //      nmap_tcp
    //      nmap_udp
    //      sub_domains
    //      sub_directory
    //      build_with
    //      c_api_url
    //      c_web_url

    let output_ip = scanner_web(
        ScannerWebGenericInput {
            target: "1.1.1.1",
            op_type: "ip",
            list_path: "none",
        },
        true,
    );
    assert_eq!(output_ip, true);

    let output_routes = scanner_web(
        ScannerWebGenericInput {
            target: "1.1.1.1",
            op_type: "routes",
            list_path: "none",
        },
        true,
    );
    assert_eq!(output_routes, true);
}

#[test]
fn test_scanner_auto_nmap() {
    let output_syn = scanner_auto_nmap(
        ScannerWebAutoNmap {
            target: "localhost",
            delay: "fast",
            ports: "auto",
            scan_type: "tcp_syn",
        },
        true,
    );

    let output_ack = scanner_auto_nmap(
        ScannerWebAutoNmap {
            target: "localhost",
            delay: "fast",
            ports: "auto",
            scan_type: "tcp_ack",
        },
        true,
    );

    let output_udp = scanner_auto_nmap(
        ScannerWebAutoNmap {
            target: "localhost",
            delay: "fast",
            ports: "auto",
            scan_type: "udp",
        },
        true,
    );

    assert_eq!(output_syn, true);
    assert_eq!(output_ack, true);
    assert_eq!(output_udp, true);
}
