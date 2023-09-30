# Maid Runner syntax

### Main Keys

    key          value
    
    ?            help
    info         information getting
    scan         scanning and enumeration
    attk         attack  target
    defe         defense host
    keep         web content downloader
    fuze         fuzzer
    lokp         lookup
    serv         local server
    host         host configuration
    pgpk         PGP messages
    free         undefined
    nuke         secure remove file
    
# Function Map

### CORE

    @core

        core_local_logger
        core_local_pgp
        core_local_safe_remove_files
        core_local_safe_remove_metadata
        core_local_file_server_ftp
        core_local_file_server_http
        core_local_web_server
        core_local_cdn_server
        core_local_adapter_router
        core_local_adapter_bluetooth
    
        core_local_scp
        core_local_downloader_web_page
        core_local_downloader_web_file_http
        core_local_downloader_web_file_ftp

    @http
    
        core_http_requests

    @config

        core_external_config_ngrok
        core_external_config_ddns
        core_external_config_ap_router

        core_local_config_docker
        core_local_config_qemu

    @optional

        core_local_config_th3maid_config
            set:
                flatpak, snap, htop, nvtop, bpytop, neovim, podman, ufw,
            config:
                set flathub
                install flatpaks
                host file with scan/ads blocking
                close all ports with ufw 
                setup podman 

        core_local_anon_config
            set:
                null
            config:
                host file with scan/ads blocking
                close all ports with ufw 
                setup docker

    
### LOOKUP

    @general
        lookup_mac_address
        lookup_ipv4
        lookup_ipv6
        lookup_geo_location
        lookup_exif_metadata

    @reverse_engineering    
        lookup_strings
        lookup_binary_hexadecimal
        lookup_binary_octal
        lookup_binary_binary
        lookup_lib_dynamic_link


    @osint

        ? no clue

### SCAN

    @web
 
        scanner_ip
        scanner_whois
        scanner_routes
        scanner_dns
        scanner_network_map //nmap
        scanner_sub_dns
        scanner_sub_directory
        scanner_build_with
        scanner_common_api_url
        scanner_common_web_urls

    @wireless

        scanner_wifi_ssid
        scanner_wifi_ussid
        scanner_wifi_devices
        scanner_bluetooth_devices_ssid
        scanner_bluetooth_devices_name

        #to the future
        scanner_wifi_devices_distance
        scanner_wifi_devices_distance_3d

### ATTACK  

    @device

        attack_device_remote_shell_windows
        attack_device_remote_shell_linux
        attack_device_remote_shell_bsd
        attack_device_remote_shell_mac

    @web 

        attack_web_dos
        attack_web_wordpress_brute_force
        attack_web_wordpress_exec_exploit
        attack_web_common_xss
        attack_web_api_end_points

    @browser

        attack_browser_grap_cookies
        attack_browser_harvest_data

    @wireless

        attack_wifi_de_auth_broadcast
        attack_wifi_poisoned_ussid
        attack_wifi_dns_redirect
        attack_wifi_sefl_arp_spoof
        attack_wifi_poisoned_hotspot
        attack_wifi_crack_password
        attack_wifi_dos

        attack_bluetooth_dos_speaker
        attack_bluetooth_dos_device
        attack_bluetooth_de_auth

### POST_ATTACK

    @rootkit
        attack_device_remote_shell_rootkit_windows
        attack_device_remote_shell_rootkit_linux
        attack_device_remote_shell_rootkit_bsd
        attack_device_remote_shell_rootkit_mac    

### BOTNET & POST_ATTACK (External code and binaries)

All botnet malware must be self and remove controlled
with scripts from an domain/url

    @botnet_backend_only

        botnet_backend_windows
        botnet_backend_linux
        botnet_backend_android
        botnet_backend_ios
        botnet_backend_mac
        botnet_backend_bsd
        botnet_backend_LoT

    @botnet_embedded_with_app

        botnet_trojan_windows
        botnet_trojan_linux
        botnet_trojan_android

    @botnet_proxy

        botnet_set_proxy_http
        botnet_set_proxy_https
        botnet_set_proxy_vps
        botnet_set_proxy_tor_bridge
        

### Explanation

Each key are a module, each module execute and handle they own commands, tasks, or actions. And
also Logs and documentation.

maid_runner will create a workspace folder called
maid_docs, this folder will be used to store all
docs and logs with name opsec_[number].opsec (json file), opsec_[number].tree (file tree from web site), and more. All file from a session with recive the same number.

maid_runner session attach number
maid_runner session detach number

### Example

```
$ maidrunner info www.example.com
:: ---------------------------- ::

    dns :: www.example.com
    ip4 :: 172.16.0.1
    ip4 :: 172.18.10.1
    ip6 :: 2204:5f34:1201:2d67:d47e:4ze9:a184:6163/64

    traceroute to localhost (127.0.0.1), 64 hops max
    1   127.0.0.1  0.002ms  0.000ms  0.000ms
    2   127.0.0.2  0.002ms  0.000ms  0.000ms
    3   127.0.0.3  0.002ms  0.000ms  0.000ms

:: ---------------------------- ::

    port_scan :: true
    port_type :: default

    PORT    STATE SERVICE
    22/tcp  open  ssh
    80/tcp  open  http
    631/tcp open  ipp


:: ---------------------------- ::

    sub_dns :: host.example.com
    sub_dns :: login.example.com
    sub_dns :: cdn.example.com

:: ---------------------------- ::

    files_tree :: true 
    file_path  :: /home/anon/maid_docs/opsec_001.tree

:: ---------------------------- ::

    json_output :: true
    json_path   :: /home/anon/maid_docs/opsec_001.opsec

```

### json → opsec_001.opsec

    {
        "opsec": "opsec_name",
        "info": {
            "target": "www.example.com",
            "scan_type": "default",
            "data": {
                "dns": [
                    "www.example.com",
                    "www.examplebackup.com",
                ],
                ip: [
                    "172.16.0.1",
                    "172.18.10.1",
                    "2204:5f34:1201:2d67:d47e:4ze9:a184:6163/64",
                ],
                "traceroute": [
                    "127.0.0.1",
                    "127.0.0.2",
                    "127.0.0.3",
                ],
                "nmap": [
                    {
                        "port": "22/tcp",
                        "state": "open",
                        "service": "ssh",
                    },
                    {
                        "port": "80/tcp",
                        "state": "open",
                        "service": "http",
                    },
                    {
                        "port": "663/tcp",
                        "state": "open",
                        "service": "ipp",
                    },
                ]
                "sub_dns": [
                    "host.example.com",
                    "login.example.com",
                    "cdn.example.com",
                ],
                "files_tree": true,
                "file_path": "/home/anon/maid_docs/opsec_001.tree",

                "json_output": true,
                "json_path": "/home/anon/maid_docs/opsec_001.opsec",

            }

        },       
    }


## FINAL

@core
	--remove_metadata           Remove metadata from a picture
		
		--debug [optional]
			true
			none
		--path                  Image file path
			./file/image
	--web_downloader            Download an web page an all relatable files 
		
		--debug [optional]      Turn debug on, was an optional flag isn't needed
			true
			none
		--url                   Target web page URL with www
			www.example.com
	
@lookup
	
    --mac_address key value     Lookup mac vendor based on first 3 pairs
		
		--debug [optional]      Turn debug on, was an optional flag isn't needed
			true
			none
		--mac                   MAC string with 00:00 or 00:00:00
			00:00:00
			01:23
		--path                  MAC lookup file (see file/general/macaddr_lockup.ascii)
			./file/mac_list

	--lookup_reverse_engineering	Lookup basic reverse engineering
		--debug [optional]      Turn debug on, was an optional flag isn't needed
			true
			none
		--sample                File to be analyzed
			./file/sample
		--type                  Type of analysis
			s                   	search for string
			h                   	search for hexadecimals
			b                   	search for binary
			r                   	todo
			l                   	search for linked library
	
@scanner 
	
    --web_scanner key value     Scanning domain and ip's 
		--target                Set target ip or dns
			172.16.0.1
			example.com

        --type                  Select and scanning type:
            ip                      basic ping
            whois                   basic whois
            routes                  basic traceroute
            dns                     basic dns enumeration
            nmap_tcp                basic nmap TCP scanner
            nmap_udp                basic nmap UDP scanner
            sub_domains             DNS sub domains scanner
            sub_directory           Web page sub directory scanner
            build_with              Scan common frameworks on a page
            c_api_url               Scan common api urls
            c_web_url               Scan common web page urls

        --path [optional]       File path need by:
                                    sub_domains
                                    sub_directory
                                    build_with
                                    c_api_url
                                    c_web_url
            ./file/generic_list File path

        --debug [optional]      Turn debug on, was an optional flag isn't needed
            true
		    none

## semples

    use crate::core::utils::*;

    pub fn sample(generic_list: Vec<String>, debug: bool) -> bool {
        let cmd: String = format!("{}", "variable");
        if debug == true {
            system_text(&cmd, "yellow");
        }
        system_text("[VOID] :: Menssage", "green");
        system_command_exec(&cmd, debug)
    }
