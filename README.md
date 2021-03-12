![ZYNIX FUSION](https://user-images.githubusercontent.com/36008397/110006679-b4ae7d80-7cf8-11eb-99a4-bc3dcb813ae8.png)

# :: Zynix Fusion :: 

    Zynix fusion is a framework for performing pentest and hacking in web applications, having several renowned tools in its backend, it aims to make it simpler to quickly use several of these tools, also exposing native tools of the project.

## :: A brief history ::

    Zynix Fusion is a project derived from other projects of my own "th3void", and has already had several names, shapes and functions, today it is as I wanted it to be, simple and straightforward.
    In 2015 this was just a shell script for installing some tools on Arch Linux, and it was left aside for many years, but now it is close to what I wanted.

## :: Warnings ::

    This software depends on other software and libraries to work properly, make sure your system has all of them.

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

## :: How to use ::

    There are two types of commands: the internal ones, which are restricted to the back-end
    system and the usable front-end ones that are used to execute the functions.
    red are the functions that do not work at this time or that have been removed.



### Installation:

    git clone https://github.com/th3void/zynix-fusion

    on Fedora:         
        
        sudo dnf install ruby ruby-devel -y && sudo dnf dnf group install 'Security Lab' -y 

    on Kali linux, parrot, black Arch, etc, just install ruby and ruby-dev.

    cd zynix-fusion

    bundle config set --local path 'vendor/bundle'
    bundle install

    if there is a permission error, run as sudo


### later:

    ruby main.rb

    Enter the command and pass the requested information, if necessary.

## :: Commands ::

### Alias 

    ls                      | ls -lha
    cls                     | clear
    sdown                   | shutdown -h now
    rm                      | rm -rf
    cp                      | cp

### From shell

    cow                     | Reset shell view

### From plugins

    plugin --fpage          | Generate fake website for bypass monetization systems  
    
### From gem    

    gem --g rg              | Generate fake Brazilian rg
    gem --g cpf             | Generate fake Brazilian
    gem --g name            | Generate fake Brazilian/Portuquese/Spanish name
    gem --g fakedb          | Generate fake database 

### From vs   

    vs --v banner'          | Show Banner
    vs --v dns'             | Show Web dns search engines
    vs --v lfile'           | Show useful linux file 
    vs --v lfolder'         | Show useful linux folders
    vs --v wfile'           | Show useful Windows files
    vs --v luss'            | Show linux useful information 
    vs --v tsearch'         | Show Tor search engines
    vs --v talt'            | Show Tor alternatives

### From kl

    kl --init               | Set global target variable   
    kl --cover              | Cover your logs  
    kl --compress           | Compress files       
    kl --extract            | Extract files       
    kl ---ginfo             | Grep banners and domain information
    kl --dns --brute        | Dns brute force           
    kl --dns --scan         | Dns scanner      
    kl --dir --scan         | Dir scanner       
    kl --smap               | Simple map
    kl --maclookup          | Mac address lookup
    kl --myip               | Show your ip
