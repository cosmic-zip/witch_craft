

const NETWORK: string = """
    watch ss -tp                                        Network connections
    netstat -ant                                        Tcp connections -anu=udp
    netstat -tulpn                                      Connections with PIDs
    lsof -i                                             Established connections
    smb:# ip /share                                    Access windows smb share
    share user x.x.x.x c$                               Mount Windows share
    smbclient -0 user\\\\ ip \\ share                   Sl1B connect
    ifconfig eth# ip I cidr                             Set IP and netmask
    ifconfig ethO:l ip I cidr                           Set virtual interface
    route add default gw gw lp                          Set GW
    ifconfig eth# mtu [size]                            Change t~TO size
    export l1AC=xx: XX: XX: XX: XX: XX                  Change t~AC
    ifconfig int hw ether t~AC                          Change t~AC
    macchanger -m l1AC i                                Backtrack t~AC changer
    iwlist int scan                                     Built-in wifi scanner
    dig -x ip                                           Domain lookup for IP
    host ip                                             Domain lookup for IP
    host -t SRV service tcp.url.com                     Domain SRV lookup
    dig @ ip domain -t AXrR                             DNS Zone Xfer
    host -1 domain namesvr                              DNS Zone Xfer
    ip xfrm state list                                  Print existing VPN kejs
    ip addr add ip /cidr aev ethO                       Adds 'hidden' interface
    /var/log/messages I grep DHCP                       List DHCP assignments
    tcpkill host ip and port port                       Block ip:port
    echo "1" /proc/sys/net/ipv4/ip forward              Turn on IP Forwarding
    echo ''nameserver x.x.x.x'' /etc7resolv.conf        Add DNS Server

"""

# System info
const SYSTEMINFO: string = """
    netstat -A ip                                       Get hostname for ip             
    id                                                  Current username                
    w                                                   Logged on users                 
    who -a                                              User information                
    last -a                                             Last users logged on            
    ps -ef                                              Process listing (top)           
    df -h                                               Disk usage (free)               
    uname -a                                            Kernel version/CPU info         
    mount                                               t1ounted file Sjstems           
    getent passwd                                       Show list of users              
    PATH~$PATH:/home/mypath                             Add to PATH variable            
    kill pid                                            Kills process with pid          
    cat /etc/issue                                      Show OS info                    
    cat /etc/'release'                                  Show OS version info            
    cat /proc/version                                   Show kernel info                
    rpm --querJ -all                                    Installed pkgs (Redhat)         
    rpm -ivh ) .rpm                                     Install RPM (-e~remove)         
    dpkg -get-selections                                Installed pkgs (Obuntu)         
    dpkg -I .deb                                        Install DEB (-r~remove)         
    pkginfo                                             Installed pkgs (Solaris)        
    which tscsh/csh/ksh/bash                            Show location of executable     
    chmod -so tcsh/csh/ksh                              Disable shell , force bash  

"""

# LINUX UTILITY COMMANDS
const UTILITY: string = """
    wget http:# url -0 url.txt -o /dev/null             Grab url
    rdesktop ip                                         Remote Desktop to ip
    scp /tmp/file user@x.x.x.x:/tmp/file                Put file
    scp user@ remoteip :/tmp/file /tmp/file             Get file
    useradd -m user                                     Add user
    passwd user                                         Change user password
    rmuser unarne                                       Remove user
    script -a outfile                                   Record shell : Ctrl-D stops
    apropos subject                                     Find related command
    history                                             View users command history
    ! num                                               Executes line # in history

"""

# LINUX FILE COMMANDS
const LINUXCOMMAND: string = """
    diff filel file2                                    Compare files
    rm -rf dir                                          Force delete of dir
    shred -f -u file                                    Overwrite/delete file
    touch -r ref_file file                              t1atches ref_ file timestamp
    touch -t YYYY11t1DDHHSS file                        Set file timestamp
    sudo fdisk -1                                       List connected drives
    mount /dev/sda# /mnt/usbkey                         t1ount USB key
    md5sum -t file                                      Compute md5 hash
    echo -n "str" | md5sum                              Generate md5 hash
    sha1sum file                                        SHAl hash of file
    sort -u                                             Sort/show unique lines
    grep -c "str" file                                  Count lines w/ ''str''
    tar cf file.tar files                               Create .tar from files
    tar xf file.tar                                     Extract .tar
    tar czf file.tar.gz files                           Create .tar.gz
    tar xzf file.tar.gz                                 Extract .tar.gz
    tar cjf file.tar.bz2 files                          Create .tar.bz2
    tar xjf file.tar.bz2                                Extract .tar.bz2
    gzip file                                           Compress/rename file
    gzip -d file. gz                                    Decompress file.gz
    upx -9 -o out.exe orig.exe                          UPX packs orig.exe
    zip -r zipname.zip \Directory\                      Create zip
    dd skip=lOOO count=2000 bs=S if=file of=file        Cut block 1K-3K from file
    split -b 9K \ file prefix                           Split file into 9K chunks
    awk 'sub("$"."\r")' unix.txt win.txt                Win compatible txt file
    find -i -name file -type *.pdf                      Find PDF files
    find / -perm -4000 -o -perm -2000 -exec ls -ldb {) \;        Search for setuid files
    dos2unix file                                       Convert to ~nix format
    file file.txt                                       Determine file type/info
    chattr (+/-)i file                                  Set/Unset immutable bit

"""

# Linux misc
const LINUXMISC: string = """
    unset HISTFILE                                      | Disable history logging
    ssh user@ ip arecord - | aplay -                    | Record remote mic
    gcc -o outfile myfile.c                             | Compile C,C++
    init 6                                              | Reboot (0 = shutdown)
    cat /etc/ 1 'syslog'.conf | grep -v "*#"            | List of log files
    grep 'href=' file 1 cut -d"/" -f3  grep url | sort -u            |Strip links in url.com
    dd if=/dev/urandom of= file bs=3145728              | Make random 311B file
    count=lOO                 
                              
"""

# Web dns
const WEBDNS: string = """
    Netcraft      – endereços fora do Brasil      http:#news.netcraft.com/
    Domaintools   - whois, lookup, IP, etc.       http:#www.domaintools.com/
    Registro BR   – endereços no Brasil           https:#registro.br/cgi-bin/whois/
    Arin          – endereços fora do Brasil      https:#www.arin.net/
    Apnic         - endereços Ásia e Pacífico     http:#www.apnic.net/apnic-info/search
    Whois         – endereços fora do Brasil      http:#new.whois.net/
    Ripe          – endereços europeus            http:#www.ripe.net/

"""

# Linux files
const LINUXFILES: string = """
    Local users hashes               /etc/shadow
    Local users                      /etc/passwd
    Local groups                     /etc/group
    Startup services                 /etc/rc.d
    Service                          /etc/init.d
    Known hostnames and IPs          /etc/hosts
    Full hostnarne with domain       /etc/HOSTNAl1E
    Network configuration            /etc/network/interfaces
    System environment variables     /etc/profile
    Ubuntu sources list              /etc/apt/sources.list
    Narneserver configuration        /etc/resolv.conf
    Bash history (also /root/)       /horne/ user /.bash historj
    Vendor-mac  lookup               /usr/share/wireshark/rnanuf
    SSH keystore                     -/.ssh/
    System log files (most Linux)    /var/log
    System log files (Unix)          /var/adrn
    List cron files                  /var/spool/cron
    Apache connection log            /var/log/apache/access.log
    Static file system info          /etc/fstab

"""

# Linux folders
const LINUXFOLDERS: string = """
    User binaries                      /bin
    Boot-up related files              /boot
    Interface for system devices       /dev
    Sjstern configuration files        /etc
    Base directory for user files      /horne
    Critical software libraries        /lib
    Third party software               /opt
    Sjstern and running programs       /proc
    Home directory of root user        /root
    System administrator binaries      /sbin
    Temporary files                    /trnp
    Less critical files                /usr
    Variable Sjstern files             /var

"""

# Windows files
const WINDOWSFILES: string = """
    Typically C:\Windows        %SYSTEMROOT%
    DNS entries                 %SYSTEMROOT%\\System32\\drivers\\etc\\hosts
    Network settings            %SYSTEMROOT%\\System32\\drivers\\etc\\networks
    User & password hashes      %SYSTEMROOT%\\system32\\config\\SAM
    Backup copy of SAM          %SYSTEMROOT%\\repair\\SAM
    Backup copy of SAM          %SYSTEMROOT%\\System32\\config\\RegBack\\SAM
    Application Log             %WINDIR%\\system32\\config\\AppEvent.Evt
    Security Log                %WINDIR%\\system32\\config\\SecEvent.Evt
    Startup Location            %ALLUSERSPROFILE%\Start Menu\Programs\Startup\\
    Startup Location            %USERPROFILE%\\Start Menu\\Programs\\Startup\\
    Prefetch dir (EXE logs)     %SYSTEMROOT%\\Prefetch

"""

const BANNER: string = """                                                     
    ███▀▀▀██████▀   ▀██▀███▄   ▀███▀████▀████▄     ▄███▀
    █▀   ███  ███   ▄█   ███▄    █   ██   ████    ████  
    ▀   ███    ███ ▄█    █ ███   █   ██   █ ██   ▄█ ██  
       ███      ████     █  ▀██▄ █   ██   █  ██  █▀ ██  
      ███   ▄    ██      █   ▀██▄█   ██   █  ██▄█▀  ██  
     ███   ▄█    ██      █     ███   ██   █  ▀██▀   ██  
    █████████  ▄████▄  ▄███▄    ██ ▄████▄███▄ ▀▀  ▄████▄

"""

proc cute*(name: string): void = 
    
    if name == "NETWORK":
        echo NETWORK
    elif name == "SYSTEMINFO":
        echo SYSTEMINFO
    elif name == "UTILITY":
        echo UTILITY
    elif name == "LINUXCOMMAND":
        echo LINUXCOMMAND
    elif name == "LINUXMISC":
        echo LINUXMISC
    elif name == "WEBDNS":
        echo WEBDNS
    elif name == "LINUXFILES":
        echo LINUXFILES
    elif name == "LINUXFOLDERS":
        echo LINUXFOLDERS
    elif name == "WINDOWSFILES":
        echo WINDOWSFILES
    elif name == "BANNER":
        echo BANNER
    else:
        echo "[option_not_found]"