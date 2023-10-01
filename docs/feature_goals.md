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
