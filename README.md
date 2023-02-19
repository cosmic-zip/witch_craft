![ZYNIX FUSION](./docs/hackerahead.png)

# After years of inactivity, I have no reason to keep this software anymore and it will soon be archived.

#   :: ZYNIX FUSION ::

    zynix-Fusion is a framework that aims to centralize, standardize and simplify the use of various
    security tools for pentest professionals. zynix-Fusion (old name: Linux evil toolkit) has few
    simple commands, one of which is the init function that allows you to define a target, and thus
    use all the tools without typing anything else.

    Zynix is a ruby script made for the purpose of being a shortcut, so instead of creating several
    tools from scratch, it simply uses a range of existing tools.

    Is zynix-Fusion better than setoolkit? Yes and no, there are two that serve the same thing and in a
    different way, the Zynix-Fusion and an automated attack information automation script.

## :: Considerations ::

    § 1 About use
    This script was made to automate the steps of gathering information about web targets, the
    misuse and responsibility of the user, to report bugs or make suggestions to open a report on
    github.
    
    § 2 About simple_scan
    Automap was replaced by simple_scan, it is lighter and faster, in addition to being less
    detectable, now it has different modes of execution that make it possible from a quick and
    simple execution to more complex modes.
    
    § 3 About Console
    The output of the script can be extremely long, so see if your console, (gnome-terminal, cmd,
    konsole) is configured to display 1000 lines (I particularly recommend 10,000 lines), for
    professional purposes it allows the documentation, it records the commands, exits and formats
    the text.
    
##  :: How to work? ::
  
    Zynix works with the idea of providing a personalized (and customizable) command line
    interface, that is, you will have to type internal zynix commands and pass the parameters, but to
    avoid having to pass the same parameter several times there is the function init, which will
    globally store these parameters and use them later in various commands automatically.
    
##  :: How to use it? ::

    There are two types of commands: the internal ones, which are restricted to the back-end
    system and the usable front-end ones that are used to execute the functions.
    red are the functions that do not work at this time or that have been removed.
    
##  :: Installation and configuration ::

    In order to use zynix it is necessary that you have dependencies installed on your computer, at
    the moment there is no script that does this, however it is possible to use it without problems in
    distributions with parrot security, kali linux, back box and fedora security, etc. Soon I will add a
    script to install the dependencies.
    If you use fedora or systems with dnf you can try sudo dnf group install "Security Lab"
    open your terminal in linux and type:
    $ git clone https://github.com/th3void/zynix-fusion.git && cd zynix-fusion && ruby main.rb
    To update the program, enter the main folder and type:
    $ git pull

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
