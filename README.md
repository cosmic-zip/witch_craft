# linux-evil-toolkit

![LINUX EVIL TOOLKIT](https://user-images.githubusercontent.com/36008397/92520390-83da0980-f1e9-11ea-821a-4f4fe3420e2e.png)


    Linux evil toolkit is a pentest framework that aims to centralize, 
    standardize and simplify the use of various tools by security 
    professionals. LITK (Linux evil toolkit) have several common commands, 
    such as dnsenum, nmap, netcat, telnet, ssh, etc. Tools to clone 
    websites, and a page repository used to social engineering.

    Is LETK better than setoolkit? Yes, and No, they are frameworks that serve to
    the same thing and do it in a similar way, but LITK is meant to be
    an alias with more functions, not a suite that even passes coffee.


##  considerations

    
    § 1 About use

        This script was made to automate the steps of gathering information about web
        targets, the misuse and responsibility of the user, to report bugs or make
        suggestions open a report on github.

    § 2 About automap

        The automap module is extremely heavy and not very discreet if used in the wrong
        way, so enable the proxy, and leave the search with the parameter -T3 (
        hardcoded), this will avoid problems,

    § 3 About Console

        The output of the script can be extremely long, so see if your console, 
        (gnome-terminal, cmd, konsole) is configured to display 1000 lines 
        (I particularly recommend 10,000 lines), for professional purposes it allows
        the documentation, it records the commands, exits and formats the text.  


##  Usage

    
    |exit           |   Close this script                                           
    |clear          |   Clear terminal                                              
    |update         |   Update Linux evil toolkit                                   
    |train          |   Show train in terminal, tuutuu                              
    |INIT           |   Setup global variables                                      
    |reset          |   Clear terminal and reset global variables                   
    |cover          |   Cover your tracks on your computer                          
    |portscanner    |   This command is replaced by automap                         
    |automap        |   Scanner target, grep ports, services, operational system,   
    |               |   firewall rules and more.                                       
    |search         |   Search email, whois and banner grep      
    |status         |   Show machine status         
    |dnsscanner     |   Scan for 'A', 'AAAA', 'CNAME', 'MX', 'NS', 'PTR', 'SOA'         
    |dirscanner     |   Scan files and folders       
    |banner         |   Show Linux evil Toolkit banner in terminal      
    |webdns         |   Show Web Sites for dns scanner      
    |linuxfiles     |   Show important linux files      
    |linuxfolders   |   Show important linux folders        
    |linuxutil      |   Show useful commands in linux       
    |test           |   For development only        


##  Backend Functions

###     From engine module

    Engine.INIT()               | Setup variables
    Engine.sys("ls")            | Test Function
    Engine.R()                  | Reset variables
    Engine.cover()              | Cover bash history
    Engine.compress()           | Compress files
    Engine.port_scanner()       | Repleced by automap
    Engine.search()             | Search whois, emails, banner grep
    Engine.status()             | Show machine status
    Engine.dns_scanner()        | Scan for 'A', 'AAAA', 'CNAME', 'MX', 'NS', 'PTR', 'SOA'
    Emgine.dir_scanner()        | Brute force for search files and folders

###     From Automap module

    Automap.less_boring()       | Execute automap host scan
    Automap.assembly()          | Backend function
    Automap.exec()              | Backend function 


###     From Visual module

    Visual.banner()             | Function for show text 
	Visual.web_dns()            | Function for show text
	Visual.linux_files()        | Function for show text
	Visual.linux_folders()      | Function for show text
	Visual.linux_util()         | Function for show text
  

###     From Interpreter Module

    Interpreter.interpreter()   | Backend function
    Interpreter.main()          | Backend function
