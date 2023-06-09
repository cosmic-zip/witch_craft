![ZYNIX FUSION](./docs/zynix-fusion.png)

#   :: ZYNIX FUSION ::

zynix-Fusion is a comprehensive framework designed to streamline, standardize, and simplify the utilization of various security tools for pentesting professionals. Formerly known as Linux Evil Toolkit, zynix-Fusion offers a set of user-friendly commands, including the "init" function, which allows users to define a target and seamlessly access all the tools without the need for additional typing.

Zynix is implemented as a Ruby script with the primary objective of serving as a shortcut. Rather than creating multiple tools from scratch, zynix leverages a range of existing tools to achieve its intended purpose.

When comparing zynix-Fusion and setoolkit, it is important to note that they both serve similar purposes but employ different approaches. Zynix-Fusion is a framework that consolidates various tools, while setoolkit is an automated attack information automation script. Therefore, determining which one is better depends on individual preferences and specific requirements.

## :: Considerations ::

Section 2: About Use
This script is designed to automate the process of gathering information about web targets. It is important to note that the user bears the responsibility for any misuse. If you encounter any bugs or have suggestions, please report them by opening an issue on GitHub.

Section 2: About simple_scan
The "Automap" tool has been replaced by "simple_scan." Simple_scan is not only lighter and faster but also less detectable. It offers different execution modes, ranging from quick and simple to more complex modes.

Section 3: About Console
Please ensure that your console (such as gnome-terminal, cmd, konsole) is configured to display a sufficient number of lines, preferably 1000 lines (I personally recommend 10,000 lines). The script's output can be quite extensive. Additionally, for professional purposes, the console allows documentation, command recording, exiting, and text formatting.
    
##  :: How to work? ::
  
Zynix operates on the concept of offering a personalized and customizable command-line interface. This means that users will need to enter internal zynix commands and provide the corresponding parameters. However, to avoid repeatedly entering the same parameters, Zynix includes the "init" function. This function allows users to globally store these parameters and automatically use them in various commands at a later stage. By utilizing the "init" function, users can save time and streamline their command inputs.
    
##  :: How to use it? ::

There are two types of commands in Zynix: internal commands and front-end commands.

Internal commands are restricted to the back-end system and are not directly accessible to users. These commands handle the internal workings of Zynix and are responsible for managing its functionality.

Front-end commands, on the other hand, are the commands that users can execute to access the desired functions of Zynix. These commands serve as the interface between the user and the framework, allowing users to interact with and utilize its features.

Please note that any commands mentioned in red are either currently non-functional or have been removed from the system.
    
##  :: Installation and configuration ::

To use Zynix, it is necessary to have the required dependencies installed on your computer. Currently, there is no script available to automatically install these dependencies. However, you can still use Zynix without any issues on distributions such as Parrot Security, Kali Linux, Back Box, Fedora Security, and others.

In the near future, I plan to add a script that will simplify the process of installing the necessary dependencies.

If you are using Fedora or systems with DNF package manager, you can try the following command to install the required dependencies:

```bash
sudo dnf group install "Security Lab"
```

To get started with Zynix, open your Linux terminal and enter the following commands:

```bash
git clone https://github.com/th3void/zynix-fusion.git
cd zynix-fusion
ruby main.rb

```

To update the program, navigate to the main folder of Zynix and execute the following command:

```bash
git pull
```



##  :: Shell commands ::

    generate --rg                   |   Generate fake rg                                        
    generate --cpf                  |   Generate fake cpf                               
    generate --person               |   Generate parson fake data                                       
    generate --fakedump             |   Generate fake database
    visuall --banner                |   Show zynix banner
    visuall --web-dns               |   Show web dns lookup
    visuall --linux-files           |   Show useful linux files
    visuall --linux-folders         |   Show useful linux folders
    visuall --win-files             |   Show useful windows files
    visuall --linux-utilites        |   Show linux command line utilites
    visuall --tor-search-link       |   Show tor search engine links
    visuall --tor-alt               |   Show tor alternatives
    visuall --help                  |   Show genereal help 
    km --init                       |   Initialize framework with data
    km --install                    |   Install framework
    km --dlookup                    |   Search whois, emails, phone numbers and banner grep
    km --dns-scanner                |   Web dns scanner
    km --dir-scanner                |   Web domain folder scanner
    km --cover                      |   Clear logs and files localy
    km --simple-map                 |   Automatic nmap
    km --maclookup                  |   MAC address lookup
    km --extract                    |   Extract files
    km --compress                   |   Compress files
    km --blue-attck                 |   Bluetooth attack module

    **alone**

        "-sL" --> "List Scan - simply list targets to scan"
        "-sP" --> "Ping Scan - go no further than determining if host is online"

    **default**

        "-sS -sV" --> "TCP SYN"
        "-sU -sV" --> "UDP Scan"

    **icmp_echo**

        "-sS -sV -PE" --> "TCP SYN + ICMP echo discovery probes"
        "-sU -sV -PE" --> "UDP Scan + ICMP echo discovery probes"
        "-sA -sV -PE" --> "ACK + ICMP echo discovery probes"

    **port_list**

        "-sS" --> "TCP SYN + [portlist]: TCP SYN discovery probes to given ports"
        "-sA" --> "ACK + [portlist]: TCP ACK discovery probes to given ports"
        "-sU" --> "UDP Scan + [portlist]: TCP UDP discovery probes to given ports"

    **special**

        "-sT -sV" --> "Connect()"
        "-sW -sV" --> "Window"
        "-sM -sV" --> "Maimon scans"
        "-sN -sV" --> "TCP Null"
        "-sF -sV" --> "FIN"
        "-sX -sV" --> "Xmas scans"

## Kernel functions

    Kernel.install              | Detecting your system and installs all dependencies
    Kernel.mac_vendor           | Search for vendor information
    Kernel.compress             | Compress files
    Kernel.extract              | Extract files
    Kernel.cover                | Covers your tracks and logs 
    Kernel.port_scanner         | Repleced by automap
    Kernel.note                 | Create simple notes
    Kernel.search               | Search whois, emails, banner grep
    Kernel.dns_scanner          | Scan for 'A', 'AAAA', 'CNAME', 'MX', 'NS', 'PTR', 'SOA'
    Kernel.dir_scanner          | Brute force for search files and folders
    Kernel.simple_scan          | Runs an automatic scanner with nmap 

## Kernel  backend

    Kernel.get_time             | Set time
    Kernel.session              | Create sessions
    Kernel.init                 | Init variables
    Kernel.set                  | Set value and show message
    Kernel.sys                  | Execute system command
    Kernel.assembly             | Backend function
    Kernel.exec                 | Backend function 

## From Visual module

    Visual.banner               | Function for show text 
    Visual.web_dns              | Function for show text
    Visual.linux_files          | Function for show text
    Visual.linux_folders        | Function for show text
    Visual.linux_util           | Function for show text
    Visual.tor_search           | Function for show text
    Visual.tor_alt              | Function for show text

## From Interpreter module

    Shell.shell                 | Backend function :: select an option
    Shell.main                  | Backend function :: main loop

## From FakeDump module

    FakeDump.fakeEmail          | Genate fake emails
    FakeDump.cpf                | Genate fake cpf
    FakeDump.gem                | Genate fake names (Brazilian, Latino and Portuquese only)
    FakeDump.idhash             | Genate hash com string
    FakeDump.simple_dump        | Genate return a simples string with data
    FakeDump.dump_xml           | Genate return xml code with data
    FakeDump.simple_call        | Genate Simple function for select xml or txt output file
