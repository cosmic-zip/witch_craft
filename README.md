![banner](maid_docs/images/maid.png)

![banner](maid_docs/images/lineBar.png)

![maid_runner](https://img.shields.io/github/actions/workflow/status/th3maid/MaidRunner/maid_runner.yml)
![GitHub issues](https://img.shields.io/github/issues/th3maid/MaidRunner)
![GitHub License](https://img.shields.io/github/license/th3maid/MaidRunner)
![GitHub top language](https://img.shields.io/github/languages/top/th3maid/MaidRunner)

# Maid Runner

Maid Runner is a versatile task automation software designed to serve
as the foundation for various cyber security modules. It provides
capabilities for tasks such as forensic research, OSINT (Open Source 
Intelligence),scanning, backup and copying, intrusion testing of 
applications and APIs, and more.


## MaidVisual

![maid_visual](maid_docs/images/maid_ui.png)

## MaidRunner

<center>
    <b>
        🚧 warning: For detailed information about how to use Maid Runner and 
        explore its features, please run maid_runner h or maid_runner --help.
    </b>
</center>

##

**MAIDRUNNER**

**NAME**

    MaidRunner - Maid Runner is a versatile task automation software designed to serve as 
    the foundation for various cyber security modules.

**SYNOPSIS**

    maid_runner module_name [--key value] [-k value] [--debug true│false] [--path file] 

**DESCRIPTION**

    This  manual  page  describes the  MaidRunner,  a versatile task automation software 
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

**LOOKUP**
    lookup                      Lookup contains automations for the lookup proccess in 
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

    scanner                     Scanner contains automations for the web scanning 
                                proccess thats include namp, dirbuster, dnsenum, etc.

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
                                maid_lists.
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

**MAID_AV**

    maid_av                     Maid_av is a basic malware detection tool based on hashes. 

    --hash                      Hash sha256, return the hash if an malware are dectec  
    --pattern                   Search on a database for hash md5, sha256, name, extencion 
    --scanner                   Automaticly scanner all files and folders inside an base path
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
            │   hardned      │ Same kill rules, but allow more outging  │
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
    
    header                      MaidRunner ASCII art
    maid                        Show and maid image in ASCII

    deprecated modules: This modules can provide some hints only.  

    unix_network                Linux network                                                                                            
    unix_sys_info               Linux system status an informations 
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

    Attention! The 'maid_list' folder must be located within '/var/maid/' directory, 
    and the current user must have read/write permissions to access it. Failure to 
    meet these requirements may result in unexpected behavior or errors during the 
    cleaning process. 

<hr>

### Instalation

The project initially includes a set of default files. These files
are created using the best possible data analysis techniques, and
their final versions are merged into the main project.

It consists of three main components: 

* maid_runner for application logic.
* maid_visual for data visualization 
* maid_api for hosting a local API with data generated by the maid_runner application.

### Build Instructions

To build the project, follow these instructions:

**Step 1: Clone the Repository**

```bash

git clone https://github.com/th3Maid/MaidRunner.git
cd MaidRunner

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

* **maid_runner**: The Maid application executable.
* **maid_visual**: Executable for data visualization.
* **maid_api**: Local API server hosting data created by maid_runner.

**Usage**

After building the project, you can run each component individually. Here's a brief overview:

**Running Maid Runner**

Execute the following command to run the Maid Runner application:

```bash

./release/maid_runner

```

**Running Maid Visual**

To visualize data, run the Maid Visual application:

```bash
./release/maid_visual
```

**Running Maid API**

Start the local API server with the following command:

```bash
./release/maid_api
```

The Maid API will serve data created by the Maid Runner application.
Contribution

Feel free to contribute to Maid Runner by submitting issues or pull requests. Your input is valuable!

### License

<center>
**This project is licensed under the GNU General Public License
v3.0.**</center>
