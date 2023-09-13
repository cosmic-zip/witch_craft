use super::*;

#[test]
fn test_maid_runner_command_chain() {
    let _exp_shell_env = "target/debug/maid_runner";
    let _exp_shell_code = "0";
    let _exp_shell_out = r###" 
    linux-vdso.so.1 (0x00007fffffba3000)
	libc.so.6 => /lib/x86_64-linux-gnu/libc.so.6 (0x00007f53e6a13000)
	/lib64/ld-linux-x86-64.so.2 (0x00007f53e6c17000)
    "###;

    let _current_shell_env = system_exec_shell(true);
    let current_cmd = "ldd /bin/cat";

    let res = system_command_exec(current_cmd, true);

    assert_eq!(res, true);
}

#[test]
fn test_system_command_deep_exec() {
    let current_cmd = "ls -lha";

    match system_command_deep_exec(current_cmd, true) {
        Ok(res) => {
            if res.stdout != "none".to_string() {
                system_text(&res.stdout, "green");
            }
            system_text(&res.stdout, "red");
        }
        Err(_e) => eprintln!("Fail system_command_deep_exec toolchain"),
    }
}

#[test]
fn test_lookup_mac_addr() {
    let lk_mac = LookupMacAddress {
        vendor_mac: "01:23",
        list_path: "/home/anon/workspace/MaidRunner/files/general/macaddr_lockup.ascii",
    };

    let output = lookup_mac_address(lk_mac, true);
    assert_eq!(output, true);
}

#[test]
fn test_lookup_reverse_engineering() {
    let opt = vec!["s".to_string(), "h".to_string(), "l".to_string()];

    for item in opt {
        let lk_re = LookupGenericPathOpType {
            sample_path: "/bin/cat",
            op_type: &item,
        };

        let output = lookup_reverse_engineering(lk_re, true);
        assert_eq!(output, true);
    }
}

#[test]
fn test_lookup_exif_metadata() {
    let photo = LookupGenericPathOpType {
        sample_path: "../docs/banner.png",
        op_type: "none",
    };
    let output = lookup_exif_metadata(photo, true);
    assert_eq!(output, true);
}

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

#[test]
fn test_bind_curl() {
    let debug = true;
    let instance = CurlBind {
        method: "get",
        auth_type: "",
        auth_token: "",
        url: "http://example.com/",
        ctn_type: "",
        data: "",
    };
    let output = curl_request(instance, debug);

    assert_eq!(output, true);
}
