![ZYNIX FUSION](https://user-images.githubusercontent.com/36008397/110006679-b4ae7d80-7cf8-11eb-99a4-bc3dcb813ae8.png)

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

    compress                | Compress files
    extract                 | Extract files
    cover                   | Covers your tracks and logs
    port_scanner            | Replaced by automap
    note                    | Create simple notes
    search                  | Search whois, emails, banner grep
    dns_scanner             | Scan for 'A', 'AAAA', 'CNAME', 'MX', 'NS', 'PTR', 'SOA'
    dir_scanner             | Brute force for search files and folders
    simple_scan             | Runs an automatic scanner with nmap
    fakeEmail               | Generate fake emails
    call_cpf                | Generate fake cpf
    call_rg                 | Generate fake rg
    call_gem                | Generate fake name (Brazilian, Spanish and Portuquese only)
    simple_dump             | Generate return a simples dum with data in txt or xml format
    banner                  | Shows simple Zynix ascii banner
    web_dns                 | Shows web dns seekers
    linux_files             | Shows useful linux files
    linux_folders           | Shows function of linux folders
    linux_util              | Shows useful linux commands
    tor_search              | Shows tor network searchers
    tor_alt                 | Shows alternatives to tor

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
