## MaidRunnerCLI

**NAME**

    MaidRunner - Maid Runner is a versatile task automation software
    designed to serve as the foundation for various cyber security modules.

**SYNOPSIS**

    maidrunner module_name [--key value] [-k value] [--debug true|false]
    [--path file]

**DESCRIPTION**

    This  manual  page  describes the  MaidRunner,  a versatile task
    automation software designed to serve as the foundation for various
    cyber security modules. It provides capabilities for tasks such as
    forensic research, OSINT (Open Source Intelligence), scanning,
    backup and copying, intrusion testing of applications and APIs, and more.

**OPTIONS**

**CORE**

    core Core describes advanced base functions for one or more
    secondary function, the core module permit access and use of the this
    functions by the user

    --remove_metadata           Remove metadata from a picture
    --path                      Image file path ./folder/image

    --web_downloader            Download an web page an all relatable
    files
    --url                       Target web page URL with www.example.com
    --debug                     Optional value for debug can be true
    or false

**LOOKUP**

    lookup  Lookup contains automations for the lookup proccess in cyber
    security

    --mac_address key value     Lookup mac vendor based on first 3 pairs
    --mac                       MAC string like: 00:00 or 00:00:00
    --path                      MAC lookup file or use --path default
    --debug                     Optional value for debug can be true
    or false

    --lookup_reverse_engineering    Lookup basic reverse engineering
    --sample                        File to be analyzed
    --type                          Type of analysis:

        ┌────────────────┬──────────────────────────────────────────┐
        |     TYPE       |       DESCRIPTION                        |
        ├────────────────┼──────────────────────────────────────────┤
        |   s            | search for string                        |
        |   h            | search for hexadecimals                  |
        |   b            | search for binary                        |
        |   r            | todo                                     |
        |   l            | search for linked library                |
        └────────────────┴──────────────────────────────────────────┘

    --debug                     Optional value for debug can be true
    or false

**WEB SCANNER**

    scanner Scanner contains automations for the web scanning proccess
    thats include namp, dirbuster, dnsenum, etc.

    --web_scanner               Scanning domain and ip's
    --target                    Set target ip or dns can be 172.16.0.1
                                or example.com
    --type                      Select and scanning type:

        ┌──────────────────┬────────────────────────────────────────┐
        |       TYPE       |       DESCRIPTION                      |
        ├──────────────────┼────────────────────────────────────────┤
        │   ip             │ Basic ping                             |
        │   whois          │ Basic whois                            |
        │   routes         │ Basic traceroute                       |
        │   dns            │ Basic dns enumeration                  |
        │   nmap_tcp       │ Basic nmap TCP scanner                 |
        │   nmap_udp       │ Basic nmap UDP scanner                 |
        │   sub_domains    │ DNS sub domains scanner                |
        │   sub_directory  │ Web page sub directory scanner         |
        │   build_with     │ Scan common frameworks on a page       |
        │   c_api_url      │ Scan common api urls                   |
        │   c_web_url      │ Scan common web page urls              |
        └──────────────────┴────────────────────────────────────────┘

    --path                      File path need by: sub_domains,
    sub_directory,
    build_with,
                                c_api_url, c_web_url, must be:
                                ./folder/list.bin, check
                                maid_lists.
    --debug                     Optional value for debug can be true
    or false.

    --scanner_auto_nmap         Advanced NMAP automation and custom binds
    --target                    Setup target ip or dns can be 172.16.0.1
                                or example.com
    --delay                     Setup an delay interval to avoid detection
                                by IDS
    --ports                     Setup ports to be scanned

        ┌────────────────┬──────────────────────────────────────────┐
        |     PORT       |       DESCRIPTION                        |
        ├────────────────┼──────────────────────────────────────────┤
        |   all          | Search for possible open ports           |
        |   auto         | Use an list of the most common           |
        |   -p 80,443    | Search for 80 and 443                    |
        |   -p 80-1337   | Search from 80 to 1337                   |
        └────────────────┴──────────────────────────────────────────┘

    --type                      Setup the nmap scanner technic

        ┌────────────────┬──────────────────────────────────────────┐
        |     TECHNIC    |       DESCRIPTION                        |
        ├────────────────┼──────────────────────────────────────────┤
        |   tcp_syn      | TCP SYN                                  |
        |   tcp_ack      | TCP ACK                                  |
        |   tcp_null     | TCP packets with no flags                |
        |   udp          | UDP Scan                                 |
        |   connect      | TCP connect                              |
        |   window       | Window scan                              |
        |   maimon       | Maimon scan CVE-2012-0507                |
        |   fin          | TCP with the FIN (Finish)                |
        |   xmas         | TCP with the FIN, URG, and PSH           |
        └────────────────┴──────────────────────────────────────────┘

    --debug                     Optional value for debug can be true
    or false.

**BCURL**

    bcurl                       Rust binding for the curl command.

    --curl_bind                 Binds for curl command.
    --method                    Allow http methods: get, post, put, patch,
                                delete, head,
                                options, connect, trace.
    --auth_type                 Setup the authentication type: basic,
                                bearer, api_key, ntlm.
    --auth_token                Setup the authentication token or
                                user:password.
    --url                       Setup target url, they must be:
                                www.example.com.
    --ctn_type                  Setup content type, they need to be one
                                of this:
                                json, xml, form_data, text,
                                multi_part_form_data.
    --data                      Setup data, if needed, but ony in the
                                fallow formats:
                                json/formData/xml/text/multi_part_form_data.
    --header                    Show http response header.
    --status_code               Show status code from a GET request
    (useful
                                for is_alive tests).

**MAID ANTIVIRUS**

    maid_av                     Maid_av is a basic malware detection tool
    based on hashes.

    --hash                      Hash sha256, return the hash if an malware
    are dectec
    --pattern                   Search on a database for hash md5, sha256,
    name, extencion
    --scanner                   Automaticly scanner all files and folders
                                inside an base path  like: /path/
                                or ./path

**FIREWALL**

    firewall

    --preset                    Use iptables to setup pre-build rule sets 

        ┌────────────────┬──────────────────────────────────────────┐
        |    OPTION      |       DESCRIPTION                        |
        ├────────────────┼──────────────────────────────────────────┤
        |   reset        | Remove all firewall rules                |
        |   kill         | Drop all in/out connection and allow     |
        |                | ports for outgoing: 80,443,24.           |
        |   hardned      | Same kill rules, but allow more outging  |
        |                | ports: 80, 8080, 443, 20, 21, 22, 25,    |
        |                | 110, 143, 53, 123.                       |
        └────────────────┴──────────────────────────────────────────┘ 
    
    --backup                    Backup and restore firewall rules
    --option                    backup to Backup and restore to restore
    --path                      Backup destination folder or file to restore
    
    --rule                      Setup an custom firewall rule
    --table                     ACCEPT, DROP, REJECT and LOG
    --chain                     INPUT, OUTPUT and FORWARD
    --protocol                  tcp and udp
    --port                      any




**WARNINGS**

    WARNING: This Tool Is For Cybersecurity Use May Have Legal
    Implications As you use this cybersecurity tool, it is important to
    be aware of the potential legal implications. Depending on how the tool 
    is used, there may be consequences under various laws and regulations.

**ENVIRONMENT**

    To ensure a smooth installation process, make sure your Linux system
    has all necessary dependencies, including packages and files.

**FILES**

    Attention! The 'maid_list' folder must be located within '/var/maid/' 
    directory, and the current user must have read/write permissions to access 
    it. Failure to meet these requirements may result in unexpected behavior or 
    errors during the cleaning process.
