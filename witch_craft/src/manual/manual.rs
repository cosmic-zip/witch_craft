use crate::core::utils::system_text;
use crate::manual::hints::*;

pub const WITCH_CRAFT_HEADER: &str = r#"

        ██╗    ██╗██╗████████╗ ██████╗██╗  ██╗         ██████╗██████╗  █████╗ ███████╗████████╗
        ██║    ██║██║╚══██╔══╝██╔════╝██║  ██║        ██╔════╝██╔══██╗██╔══██╗██╔════╝╚══██╔══╝
        ██║ █╗ ██║██║   ██║   ██║     ███████║        ██║     ██████╔╝███████║█████╗     ██║   
        ██║███╗██║██║   ██║   ██║     ██╔══██║        ██║     ██╔══██╗██╔══██║██╔══╝     ██║   
        ╚███╔███╔╝██║   ██║   ╚██████╗██║  ██║███████╗╚██████╗██║  ██║██║  ██║██║        ██║   
         ╚══╝╚══╝ ╚═╝   ╚═╝    ╚═════╝╚═╝  ╚═╝╚══════╝ ╚═════╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝        ╚═╝   

"#;

pub const MAID_BANNER: &str = r#"

                                                                                                                                                      
                                                                                                                                                      
                                                                                                                              ░░██                    
                                                        ░░░░                                                                  ░░██                    
                                              ░░▒▒░░░░░░░░░░                                                              ░░░░░░▒▒                    
                                ░░░░  ░░░░▓▓▓▓▓▓▓▓▓▓▓▓▓▓░░░░                                                      ▒▒▓▓▓▓▓▓▒▒  ░░▒▒                    
                                    ░░░░▒▒▒▒▓▓▓▓▓▓▒▒▓▓▓▓▓▓▒▒▒▒                                                ░░▒▒▓▓▓▓▒▒▓▓▓▓▓▓░░░░▒▒                  
                                    ░░▓▓▓▓▓▓▓▓▓▓▒▒░░▓▓▓▓▓▓▓▓▒▒                                                ░░▒▒▓▓▒▒░░▓▓▓▓▓▓▓▓  ▓▓                  
                                      ▓▓▓▓▓▓▓▓▓▓▒▒░░▒▒▓▓▓▓▓▓▓▓░░                                              ░░▓▓▓▓░░░░▓▓▓▓▓▓▓▓▓▓▓▓░░                
                                      ▓▓▓▓▓▓▓▓▓▓▒▒░░░░▓▓▓▓▓▓▓▓▓▓                                              ▒▒▓▓▓▓░░░░▓▓▓▓▓▓▓▓▓▓▓▓▒▒                
                                    ░░▓▓▓▓▓▓▓▓▓▓▒▒▒▒░░▓▓▓▓▓▓▓▓▓▓                                              ▓▓▓▓▓▓▒▒░░▓▓▓▓▓▓▓▓▓▓██▓▓                
                                      ▓▓▓▓▒▒▒▒▒▒░░▒▒▓▓▒▒▓▓▒▒▓▓▓▓                                              ▓▓░░░░▒▒▓▓▒▒▓▓▒▒▓▓▓▓▓▓▓▓                
                                      ▒▒▓▓▒▒▓▓░░░░░░░░░░▒▒▒▒▓▓▓▓                                              ▒▒▒▒░░░░░░░░▒▒▒▒▓▓██▓▓▓▓                
                                      ▒▒▒▒░░░░░░░░░░░░░░░░▒▒▓▓▓▓░░                                            ▒▒░░░░░░░░░░░░▓▓▓▓██▓▓▓▓                
                                      ▓▓▒▒░░░░░░░░░░░░░░░░░░▓▓▓▓▒▒                                            ░░░░░░░░░░░░░░▒▒▓▓██▓▓▓▓                
                                      ▓▓▓▓░░░░░░░░░░░░░░░░▒▒▓▓▓▓▓▓                                              ░░░░░░░░░░░░▒▒▓▓▓▓▓▓▓▓░░              
                                      ▒▒████░░░░░░░░▒▒░░░░▒▒▓▓▓▓▓▓░░                                              ░░▒▒▒▒░░░░▒▒▓▓▓▓▓▓▓▓░░              
                    ░░░░░░            ▒▒▓▓████▓▓░░░░░░░░▒▒▒▒▓▓▓▓▓▓▓▓                                              ░░░░░░░░▒▒▓▓▓▓▓▓▓▓▓▓▒▒              
                  ░░░░░░░░░░          ▒▒▓▓██████▓▓░░░░▒▒▓▓▓▓████▓▓▓▓▓▓                                    ░░░░      ░░▒▒▓▓██▓▓▓▓▓▓▓▓██▓▓              
                  ░░░░░░░░░░░░░░██████▓▓████████████▓▓▓▓▓▓██████▓▓▓▓▓▓    ░░▒▒░░                      ░░░░░░░░░░  ▒▒▓▓▓▓▓▓██▓▓▓▓▓▓▓▓▓▓▓▓              
                    ░░░░░░░░  ▒▒████████████████████████████▓▓██▒▒██████▓▓▓▓▓▓██                        ░░░░░░▒▒▓▓██░░░░░░░░░░▓▓▓▓▓▓▓▓▓▓              
                    ░░░░░░    ▓▓████████████████████▒▒░░████████░░████▓▓████▒▒░░▒▒                    ░░░░░░░░▓▓▓▓▒▒          ▒▒▓▓████▓▓              
                    ░░░░░░    ▒▒████▓▓▓▓▓▓▒▒░░▓▓████▒▒░░░░▒▒▒▒▓▓░░▓▓▓▓██▓▓▓▓▓▓░░░░                    ░░░░░░▒▒████            ░░▒▒▓▓██▓▓░░            
                    ░░░░░░  ░░░░▓▓██▓▓▓▓░░    ░░▒▒░░░░░░░░░░░░░░░░██▓▓██████░░                        ░░░░▒▒▒▒██▒▒          ░░░░░░▓▓██▓▓▒▒            
                    ░░░░░░░░░░░░████▓▓▒▒                          ██▓▓██████                          ░░░░▒▒▓▓▓▓          ░░░░  ░░▓▓██▓▓▒▒            
                    ░░░░░░  ░░░░▓▓██▒▒              ░░            ██▓▓████▒▒░░                        ░░░░░░▒▒▓▓              ░░▒▒▓▓██▓▓▒▒            
                    ░░░░░░  ░░░░▒▒██                              ▓▓▓▓████  ░░                          ░░░░░░▒▒        ░░      ██▓▓████▒▒            
                  ░░░░░░░░░░░░░░░░▒▒                              ▒▒████▓▓                              ░░░░  ░░    ░░    ░░    ██▓▓████▓▓            
                      ░░░░░░░░░░▓▓▒▒                              ▒▒▓▓▒▒░░                              ░░░░    ░░      ░░    ▒▒██▓▓████▒▒            
                  ░░░░░░░░░░░░░░▓▓▓▓░░                            ██▓▓▒▒░░                              ░░░░  ░░░░░░░░        ████████▓▓▒▒            
                  ░░░░░░░░░░░░░░██▓▓▓▓▒▒                        ▓▓▓▓██░░░░░░░░                          ░░░░  ░░░░░░░░░░  ▒▒▓▓████████▒▒              
                  ░░░░░░░░░░░░▒▒████▓▓▓▓██▒▒              ░░████▓▓▓▓██▒▒░░░░░░                        ░░░░░░  ░░░░░░░░░░░░██████████▓▓▒▒              
                  ░░░░░░░░░░░░░░████▓▓▓▓▓▓▓▓▓▓▒▒░░▒▒▓▓████▓▓██▓▓▓▓▓▓▒▒▒▒░░░░░░                        ░░░░░░░░░░░░░░░░▓▓████████████▓▓▒▒              
                  ░░░░░░░░░░    ██▓▓████▓▓▓▓▓▓████▓▓▓▓▓▓████████▓▓██  ▒▒░░░░░░                        ░░░░░░░░░░░░░░▓▓████████████▓▓▓▓▒▒              
                  ░░░░░░░░                  ░░▒▒██████████▓▓██▓▓██▓▓  ▒▒░░░░░░                        ░░░░░░░░░░░░▓▓████████████▓▓▓▓▓▓▒▒              
                                                    ░░▒▒██████▓▓▓▓    ▒▒░░░░░░                        ░░░░░░░░░░░░██████████████▓▓██░░                
                                                            ▒▒▓▓▓▓    ▒▒░░░░░░                          ░░░░░░░░▓▓████████████▓▓██▒▒                  
                                                                ▒▒    ░░░░░░░░                          ░░░░▒▒▒▒░░░░  ░░▒▒▒▒██░░░░                    
                                            ░░                        ░░░░░░░░                          ░░░░░░░░░░░░░░░░░░░░░░  ░░                    
                                                              ░░      ░░░░░░░░░░                            ██████████████▒▒                          
                                                                        ░░░░░░░░                          ▓▓██▓▓▓▓██▓▓██▓▓░░                          
                                                  ░░                    ░░░░░░░░                        ░░██▓▓▓▓████▓▓▓▓▓▓▓▓░░                        
                ▒▒                                ░░                    ░░░░░░░░░░                      ██▓▓▓▓████████▓▓▓▓████▒▒░░                    
              ▒▒▓▓                                                        ░░░░░░░░                    ▒▒██▓▓████████████▓▓▓▓██▓▓██▓▓░░                
              ██░░                                  ░░                    ▒▒░░░░░░                    ██▓▓▓▓▓▓██████████▓▓▓▓▓▓▓▓▓▓▓▓▓▓░░              
            ▒▒██                                      ░░              ▒▒  ░░░░░░░░░░                ▓▓▓▓▓▓▓▓██████▓▓██▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓                
          ░░████                                      ░░              ██░░  ░░░░░░░░              ░░▓▓▓▓▓▓▓▓██████▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒▒              
          ▓▓██▓▓▒▒                                    ░░░░░░          ██▓▓  ▒▒░░░░░░            ░░▓▓▓▓▓▓▓▓▓▓████████▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓              
        ▓▓██▓▓▓▓▓▓      ░░                              ░░░░░░      ░░████▓▓  ▒▒░░░░░░          ░░▓▓▓▓▓▓▓▓▓▓████████▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓██      ░░      
      ▒▒▓▓▓▓▓▓▓▓▓▓▓▓░░░░░░                                ░░░░░░  ░░████████▓▓  ░░░░░░        ░░░░▓▓▓▓▓▓▓▓▓▓████████▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓      ░░░░    
    ░░▓▓▓▓▓▓▓▓▓▓▓▓████░░░░                                    ░░░░░░██████████▓▓░░░░░░          ▒▒▓▓▓▓▓▓▓▓██████████▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒▒      ██    
    ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓██▓▓                                  ░░░░░░░░▓▓██▓▓████████▓▓░░░░░░        ▓▓▓▓▓▓▓▓▓▓██████████▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓      ██    
  ░░▓▓▓▓▓▓▓▓▓▓▓▓████████                      ░░            ░░░░  ██████████████▓▓▒▒░░░░      ░░▓▓▓▓▓▓▓▓▓▓██████████▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓██    ▒▒▓▓▒▒  
  ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓██████▓▓          ░░  ░░    ░░            ░░░░▓▓██████████████████░░░░░░    ░░▓▓▓▓▓▓▓▓▓▓████████▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓  ▓▓▓▓████  
▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓████████▓▓████▒▒  ░░  ░░░░    ░░  ░░          ▓▓██████████▓▓████████▓▓░░░░░░  ░░▓▓▓▓▓▓▓▓▓▓████████▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓████▓▓██▓▓  
▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓██████████████████▒▒░░    ░░  ░░      ████████▓▓██████████████████████▓▓░░░░  ▒▒▓▓▓▓▓▓████████████▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓██▓▓████░░
▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓████████████████████████▓▓▒▒▒▒▒▒▓▓██████████████████████████████████████▓▓░░  ▓▓▓▓▓▓▓▓▓▓██████████▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓██▓▓
▒▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓████████▓▓▓▓████████████████████████████████████████████████████████████▒▒░░  ▓▓▓▓▓▓▓▓▓▓██████████▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓████
      ▒▒▓▓▓▓▓▓▓▓██████████████████████████████████████████████████████████▓▓▓▓▓▓██████▓▓████░░▓▓▓▓▓▓▓▓████████████▓▓██▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓██
          ▒▒██▓▓██▓▓▓▓████████████████████████████████████████████████████▓▓▓▓██▓▓██████▒▒  ▓▓██▒▒▒▒▓▓████████████▓▓▓▓▒▒▒▒▓▓██▓▓▓▓▓▓▓▓▓▓▓▓▓▓██▓▓▓▓██▒▒
              ▒▒▓▓▓▓▓▓▓▓████▓▓████████████████████████████████████████▓▓████▓▓██▓▓▒▒        ▓▓▓▓▓▓░░░░░░░░▓▓████████▒▒░░░░░░░░▒▒▒▒▒▒▒▒░░░░  ░░░░░░    
                  ░░████▓▓▓▓▓▓████▓▓██████████████████████████████▓▓██▓▓██▓▓░░            ▒▒▓▓██░░▒▒░░░░░░░░░░░░░░░░░░░░░░░░░░░░▒▒░░                  
                    ░░░░░░░░▒▒▓▓▓▓████████████████▓▓████████████████▓▓░░                            ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░                  
                      ░░░░▒▒▒▒░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  ░░░░                              ░░░░    ░░░░░░░░░░░░░░░░░░░░░░░░                  
                      ░░░░░░▒▒▒▒░░▒▒░░░░░░      ░░░░░░                                                      ░░░░░░░░░░░░░░░░░░░░░░░░                  
                      ░░░░░░▒▒░░░░░░░░░░░░░░░░░░░░░░                                                          ░░░░░░░░░░░░░░░░░░░░░░                  
                        ░░░░░░░░░░░░░░░░░░░░░░░░░░                                                            ░░░░░░░░░░░░░░░░░░░░                    
                        ░░░░░░░░░░░░░░░░░░░░░░░░░░                                                            ░░░░░░░░░░░░░░░░░░░░                    
                        ░░░░░░░░░░░░░░░░░░░░░░░░                                                              ░░░░░░░░░░░░░░░░░░░░                    
                        ██▓▓▓▓▒▒░░░░░░░░░░░░░░░░                                                                ░░░░░░░░░░░░░░▓▓                      
                        ▒▒████████████▓▓▒▒░░░░                                                                  ██████▓▓▓▓▓▓████                      
                        ▓▓▓▓▓▓██████████████                                                                  ░░████████████▓▓██                      
                        ▓▓▓▓▓▓████████████                                                                      ████████████████                      


"#;

pub const WITCH_CRAFT_MAIN_HELP: &str = r#"                                                                                          
WITCH_CRAFT

NAME

    witch_craft - witch_craft is a versatile task automation software designed to serve as 
    the foundation for various cyber security modules.

SYNOPSIS

    witch_craft module_name [--key value] [-k value] [--debug true│false] [--path file] 

DESCRIPTION

    This  manual  page  describes the  witch_craft,  a versatile task automation software 
    designed to serve as the foundation for various cyber security modules. It provides 
    capabilities for tasks such as forensic research, OSINT (Open Source Intelligence), 
    scanning, backup and copying, intrusion testing of applications and APIs, and more.

OPTIONS

CORE    
    core                        Core describes advanced base functions for one or more 
                                secondary function, the core module permit access and 
                                use of the this functions by the user

    --remove_metadata           Remove metadata from a picture 
    --path                      Image file path ./folder/image
            
    --web_downloader            Download an web page an all relatable files 
    --url                       Target web page URL with www.example.com
    --debug                     Optional value for debug can be true or false

    --session                   Manage sessions
    --name                      Setup session name
    --desc                      Setup session description
    --wipe                      If "yes" will set name and description to default

LOOKUP
    lookup                      Lookup contains automation's for the lookup process in 
                                cyber security

    --mac_address key value     Lookup mac vendor based on first 3 pairs
    --mac                       MAC string like: 00:00 or 00:00:00
    --path                      MAC lookup file or use --path default
    --debug                     Optional value for debug can be true or false

    --lookup_re                 Lookup basic reverse engineering
    --sample                        File to be analyzed
    --type                          Type of analysis:
            ┌────────────────┬──────────────────────────────────────────┐
            │     TYPE       │       DESCRIPTION                        │
            ├────────────────┼──────────────────────────────────────────┤
            │   s            │ search for string                        │
            │   h            │ search for hexadecimals                  │
            │   b            │ search for binary                        │
            │   d            │ file details                             │
            │   l            │ search for linked library                │
            └────────────────┴──────────────────────────────────────────┘
    --debug                     Optional value for debug can be true or false

WEB SCANNER

    scanner                     Scanner contains automation's for the web scanning 
                                process thats include namp, dirbuster, dnsenum, etc.

    --web_scanner               Scanning domain and ip's 
    --target                    Set target ip or dns can be 172.16.0.1 or example.com
    --type                      Select and scanning type:

            ┌────────────────────────┬──────────────────────────────────┐
            │       TYPE             │       DESCRIPTION                │
            ├────────────────────────┼──────────────────────────────────┤
            │   ip                   │ Basic ping                       │
            │   whois                │ Basic whois                      │
            │   routes               │ Basic traceroute                 │
            │   dns                  │ Basic dns enumeration            │
            │   nmap_tcp             │ Basic nmap TCP scanner           │
            │   nmap_udp             │ Basic nmap UDP scanner           │
            │   sub_domains          │ DNS sub domains scanner          │
            │   sub_directory        │ Web page sub directory scanner   │
            │   build_with           │ Scan common frameworks on a page │
            │   c_api_url            │ Scan common api urls             │
            │   c_web_url            │ Scan common web page urls        │
            └────────────────────────┴──────────────────────────────────┘

    --path                      File path need by: sub_domains, sub_directory, build_with,
                                c_api_url, c_web_url, must be: ./folder/list.bin, check
                                witch_spells.
    --debug                     Optional value for debug can be true or false.

    --scanner_auto_nmap         Advanced NMAP automation and custom binds
    --target                    Setup target ip or dns can be 172.16.0.1 or example.com
    --delay                     Setup an delay interval to avoid detection by IDS, 
                                Options are: fast, slow and paranoid
    --ports                     Setup ports to be scanned 

            ┌────────────────┬──────────────────────────────────────────┐
            │     PORT       │       DESCRIPTION                        │
            ├────────────────┼──────────────────────────────────────────┤
            │   all          │ Search for possible open ports           │
            │   auto         │ Use an list of the most common           │
            │   80,443       │ Search for 80 and 443                    │
            │   80-1337      │ Search from 80 to 1337                   │
            └────────────────┴──────────────────────────────────────────┘ 

    --type                      Setup the nmap scanner technic 

            ┌────────────────┬──────────────────────────────────────────┐
            │     TECHNIC    │       DESCRIPTION                        │
            ├────────────────┼──────────────────────────────────────────┤
            │   tcp_syn      │ TCP SYN                                  │
            │   tcp_ack      │ TCP ACK                                  │
            │   tcp_null     │ TCP packets with no flags                │
            │   udp          │ UDP Scan                                 │
            │   connect      │ TCP connect                              │
            │   window       │ Window scan                              │
            │   maimon       │ Maimon scan CVE-2012-0507                │
            │   fin          │ TCP with the FIN (Finish) flag           │
            │   xmas         │ TCP with the FIN, URG, and PSH flags     │
            └────────────────┴──────────────────────────────────────────┘
            
    --debug                     Optional value for debug can be true or false.

BCURL

    bcurl                       Rust binding for the curl command.

    --curl_bind                 Binds for curl command.
    --method                    Allow http methods: get, post, put, patch, delete, head, 
                                options, connect, trace.
    --auth_type                 Setup the authentication type: basic, bearer, api_key, 
                                ntlm.   
    --auth_token                Setup the authentication token or user:password.
    --url                       Setup target url, they must be: www.example.com.
    --ctn_type                  Setup content type, they need to be one of this: 
                                json, xml, form_data, text, multi_part_form_data.
    --data                      Setup data, if needed, but ony in the fallow formats: 
                                json/formData/xml/text/multi_part_form_data.
    --header                    Show http response header.
    --status_code               Show status code from a GET request (useful for is_alive 
                                tests).

ANTIVIRUS

    antivirus                     antivirus is a basic malware detection tool based on hashes. 

    --hash                      Hash sha256, return the hash if an malware are detected  
    --pattern                   Search on a database for hash md5, sha256, name, extension 
    --scanner                   Automatically scanner all files and folders inside an base path
                                like: /path/ or ./path

FIREWALL

    firewall                    Setup, add, remove, backup, restore and flush firewall rules

    --preset                    Use iptables to setup pre-build rule sets 
                                                                        
            ┌────────────────┬──────────────────────────────────────────┐
            │    OPTION      │       DESCRIPTION                        │
            ├────────────────┼──────────────────────────────────────────┤
            │   reset        │ Remove all firewall rules                │
            │   kill         │ Drop all in/out connection and allow     │
            │                │ ports for outgoing: 80,443,24.           │
            │   hardened     │ Same kill rules, but allow more outgoing │
            │                │ ports: 80, 8080, 443, 20, 21, 22, 25,    │
            │                │ 110, 143, 53, 123.                       │
            └────────────────┴──────────────────────────────────────────┘ 
    
    --backup                    Backup and restore firewall rules
    --option                    backup to Backup and restore to restore
    --path                      Backup destination folder or file to restore
    
    --rule                      Setup an custom firewall rule
    --table                     ACCEPT, DROP, REJECT and LOG
    --chain                     INPUT, OUTPUT and FORWARD
    --protocol                  tcp and udp
    --port                      any

HELP 

    help                        Show help pages and common hints 
    
    header                      witch_craft ASCII art
    maid                        Show and maid image in ASCII

    deprecated modules: This modules can provide some hints only.  

    unix_network                Linux network 
    unix_sys_info               Linux system status an information 
    unix_utility                Linux utility
    unix_command                Linux command
    unix_misc                   Linux misc
    unix_files                  Linux files
    unix_folders                Linux folders
    windows_files               Windows important files                                                         
    windows_reg                 Windows register                                                   
    windows_cmd                 Windows cmd hints                                                   
    windows_powershell          Windows powershell hints                                                            

WARNINGS

    WARNING: This Tool Is For Cybersecurity Use May Have Legal Implications
    As you use this cybersecurity tool, it is important to be aware of the potential 
    legal implications. Depending on how the tool is used, there may be consequences 
    under various laws and regulations.

ENVIRONMENT

    To ensure a smooth installation process, make sure your Linux system has all 
    necessary dependencies, including packages and files.    

FILES

    Attention! The 'maid_list' folder must be located within '/var/witch_craft/' directory, 
    and the current user must have read/write permissions to access it. Failure to 
    meet these requirements may result in unexpected behavior or errors during the 
    cleaning process. 

"#;



pub fn system_exec_manual(page: &str) -> bool {
    match page {
        // UNIX
        "unix_network" => system_text(UNIX_NETWORK, "cyan"),
        "unix_sys_info" => system_text(UNIX_SYSTEMINFO, "cyan"),
        "unix_utility" => system_text(UNIX_UTILITY, "cyan"),
        "unix_command" => system_text(UNIX_COMMAND, "cyan"),
        "unix_misc" => system_text(UNIX_MISC, "cyan"),
        "unix_files" => system_text(UNIX_FILES, "cyan"),
        "unix_folders" => system_text(UNIX_FOLDERS, "cyan"),
        // WINDOWS
        "windows_files" => system_text(WINDOWS_FILES, "cyan"),
        "windows_reg" => system_text(WINDOWS_COMMON_REGISTER_LOCATIONS, "cyan"),
        "windows_cmd" => system_text(WINDOWS_CMD_BASICS, "cyan"),
        "windows_powershell" => system_text(WINDOWS_POWERSHELL_BASICS, "cyan"),
        // ETC
        "header" => system_text(WITCH_CRAFT_HEADER, "purple"),
        "maid" => system_text(MAID_BANNER, "white"),
        _ => {
            system_text(WITCH_CRAFT_HEADER, "purple");
            system_text(WITCH_CRAFT_MAIN_HELP, "purple");
            return true;
        }
    }
}

pub fn shell_manual(system_input: &mut Vec<String>) -> bool {
    if system_input.len() < 3 {
        return system_exec_manual("none");
    }
    let cmd = &system_input[2];
    system_exec_manual(&cmd)
}
