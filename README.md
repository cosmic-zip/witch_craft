![banner](witch_docs/media_kit/trans_banner/witch_craft_banner_transp.png)

![banner](witch_docs/images/lineBar.png)

![witch_craft](https://img.shields.io/github/actions/workflow/status/th3maid/witch_craft/witch_craft.yml)
![GitHub issues](https://img.shields.io/github/issues/th3maid/witch_craft)
![GitHub License](https://img.shields.io/github/license/th3maid/witch_craft)
![GitHub top language](https://img.shields.io/github/languages/top/th3maid/witch_craft)

# witch_craft

witch_craft is a versatile task automation software designed to serve
as the foundation for various cyber security modules. It provides
capabilities for tasks such as forensic research, OSINT (Open Source 
Intelligence),scanning, backup and copying, intrusion testing of 
applications and APIs, and more.


<center>
    <b>
        🚧 warning: For detailed information about how to use witch_craft and 
        explore its features, please run witch_craft h or witch_craft --help.
    </b>
</center>

##

**WITCH_CRAFT**

**NAME**

    witch_craft - witch_craft is a versatile task automation software designed to serve as 
    the foundation for various cyber security modules.

**SYNOPSIS**

    witch_craft module_name [--key value] [-k value] [--debug true│false] [--path file] 

**DESCRIPTION**

    This  manual  page  describes the  witch_craft,  a versatile task automation software 
    designed to serve as the foundation for various cyber security modules. It provides 
    capabilities for tasks such as forensic research, OSINT (Open Source Intelligence), 
    scanning, backup and copying, intrusion testing of applications and APIs, and more.

**OPTIONS**

**CORE**    
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

**LOOKUP**
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

**WEB SCANNER**

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

**BCURL**

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

**ANTIVIRUS**

    antivirus                   Maid_av is a basic malware detection tool based on hashes. 

    --hash                      Hash sha256, return the hash if an malware are detected 
    --pattern                   Search on a database for hash md5, sha256, name, extension 
    --scanner                   Automatically scanner all files and folders inside an base path
                                like: /path/ or ./path

**FIREWALL**

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

**HELP** 

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

**WARNINGS**

    WARNING: This Tool Is For Cybersecurity Use May Have Legal Implications
    As you use this cybersecurity tool, it is important to be aware of the potential 
    legal implications. Depending on how the tool is used, there may be consequences 
    under various laws and regulations.

**ENVIRONMENT**

    To ensure a smooth installation process, make sure your Linux system has all 
    necessary dependencies, including packages and files.    

**FILES**

    Attention! The 'maid_list' folder must be located within '/var/witch_craft/' directory, 
    and the current user must have read/write permissions to access it. Failure to 
    meet these requirements may result in unexpected behavior or errors during the 
    cleaning process. 

<hr>

### Instalation

The project initially includes a set of default files. These files
are created using the best possible data analysis techniques, and
their final versions are merged into the main project.

It consists of three main components: 

* witch_craft for CLI application.
* witch_oracle for GUI application 

### Build Instructions

To build the project, follow these instructions:

**Step 1: Clone the Repository**

```bash

git clone https://github.com/th3Maid/witch_craft.git
cd witch_craft

```

**Step 2: Run the Build Script**

Execute the provided build script build.sh:

```bash
chmod +x build.sh
./build.sh

```

The script will prompt you to enter the root password, create a folder called release, and place the built executables inside it.
Step 3: Explore the Release Folder

Navigate to the release folder to find the built components:

* **witch_craft**: The Maid application executable.
* **witch_oracle**: Executable for data visualization.

**Usage**

After building the project, you can run each component individually. Here's a brief overview:

**Running witch_craft**

Execute the following command to run the witch_craft application:

```bash

./release/witch_craft

```

**Running Maid Visual**

To visualize data, run the Maid Visual application:

```bash
./release/witch_oracle
```

Feel free to contribute to witch_craft by submitting issues or pull requests. Your input is valuable!

### License

<center>
<b>
This project is licensed under the GNU General Public License
v3.0.
</b>
</center>
