// Network
pub const UNIX_NETWORK: &str = r#"
    watch ss -tp                                        Network connections
    netstat -ant                                        Tcp connections -anu=udp
    netstat -tulpn                                      Connections with PIDs
    lsof -i                                             Established connections
    smb:// ip /share                                    Access windows smb share
    share user x.x.x.x c$                               Mount Windows share
    smbclient -0 user\\\\ ip \\ share                   Sl1B connect
    ifconfig eth# ip I cidr                             Set IP and netmask
    ifconfig ethO:l ip I cidr                           Set virtual interface
    route add default gw gw lp                          Set GW
    ifconfig eth# mtu [size]                            Change t~TO size
    export l1AC=xx: XX: XX: XX: XX: XX                  Change t~AC
    ifconfig int hw ether t~AC                          Change t~AC
    macchanger -m MAC                                   Backtrack t~AC changer
    iwlist int scan                                     Built-in wifi scanner
    dig -x ip                                           Domain lookup for IP
    host ip                                             Domain lookup for IP
    host -t SRV service tcp.url.com                     Domain SRV lookup
    dig @ ip domain -t AXrR                             DNS Zone Transfer
    host -1 domain namesvr                              DNS Zone Transfer
    ip xfrm state list                                  Print existing VPN kejs
    ip addr add ip /cidr aev ethO                       Adds 'hidden' interface
    cat /var/log/messages | grep DHCP                       List DHCP assignments
    tcpkill host ip and port port                       Block ip:port
    echo "1" /proc/sys/net/ipv4/ip forward              Turn on IP Forwarding
    echo ''nameserver x.x.x.x'' /etc7resolv.conf        Add DNS Server
"#;

// System info
pub const UNIX_SYSTEMINFO: &str = r#"
    netstat -A ip                                       Get hostname for ip             
    id                                                  Current username                
    w                                                   Logged on users                 
    who -a                                              User information                
    last -a                                             Last users logged on            
    ps -ef                                              Process listing (top)           
    df -h                                               Disk usage (free)               
    uname -a                                            Kernel version/CPU info         
    mount                                               Mounted file Systems           
    getent passwd                                       Show list of users              
    PATH~$PATH:/home/mypath                             Add to PATH variable            
    kill pid                                            Kills process with pid          
    cat /etc/issue                                      Show OS info                    
    cat /etc/'release'                                  Show OS version info            
    cat /proc/version                                   Show kernel info                
    rpm --querJ -all                                    Installed pkgs (Redhat)         
    rpm -ivh ) .rpm                                     Install RPM (-e~remove)         
    dpkg -get-selections                                Installed pkgs (Ubuntu)         
    dpkg -I .deb                                        Install DEB (-r~remove)         
    pkginfo                                             Installed pkgs (Solaris)        
    which tscsh/csh/ksh/bash                            Show location of executable     
    chmod -so tcsh/csh/ksh                              Disable shell , force bash      
"#;

// UNIX UTILITY COMMANDS
pub const UNIX_UTILITY: &str = r#"
    wget http:// url -0 url.txt -o /dev/null                                    Grab url
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
"#;

// UNIX FILE COMMANDS
pub const UNIX_COMMAND: &str = r#"
    diff filel file2                                    Compare files
    rm -rf dir                                          Force delete of dir
    shred -f -u file                                    Overwrite/delete file
    touch -r ref_file file                              slashes ref_ file timestamp
    touch -t YYYYDDHHSS file                            Set file timestamp
    sudo fdisk -l                                       List connected drives
    mount /dev/sda# /mnt/usbkey                         Mount USB key
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
    find / -perm -4000 -o -perm -2000 -exec ls -ldb {) \;        Search for set uid files
    dos2unix file                                       Convert to ~nix format
    file file.txt                                       Determine file type/info
    chattr (+/-)i file                                  Set/Unset immutable bit
"#;

// UNIX misc
pub const UNIX_MISC: &str = r#"
    unset HISTFILE                                      Disable history logging
    ssh user@ ip arecord - │ aplay -                    Record remote mic
    gee -o outfile myfile.c                             Compile C,C++
    init 6                                              Reboot (0 = shutdown)
    cat /etc/ 1 'syslog'.conf │ grep -v "*#"            List of log files
    dd if=/dev/urandom of= file bs=3145728              Make random 311B file
    count=lOO                                           
"#;

// UNIX files
pub const UNIX_FILES: &str = r#"
    Local users hashes               /etc/shadow
    Local users                      /etc/passwd
    Local groups                     /etc/group"
    Startup services                 /etc/rc.d
    Service                          /etc/init.d
    Known hostnames and IPs          /etc/hosts
    Full hostnarne with domain       /etc/HOSTNAl1E
    Network configuration            /etc/network/interfaces
    System environment variables     /etc/profile
    Ubuntu sources list              /etc/apt/sources.list
    Narneserver configuration        /etc/resolv.conf
    Bash history (also /root/)       /horne/ user /.bash historj
    Vendor-t1AC lookup               /usr/share/wireshark/rnanuf
    SSH keystore                     -/.ssh/
    System log files (most UNIX)     /var/log
    System log files (Unix)          /var/adrn
    List cron files                  /var/spool/cron
    Apache connection log            /var/log/apache/access.log
    Static file system info          /etc/fstab
"#;

// UNIX folders
pub const UNIX_FOLDERS: &str = r#"
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
    Temporary files                    /temp
    Less critical files                /usr
    Variable Sjstern files             /var
"#;

// Windows files
pub const WINDOWS_FILES: &str = r#"
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
"#;

// Commonly Used Windows Registry Locations
pub const WINDOWS_COMMON_REGISTER_LOCATIONS: &str = r#"
    HKLM\Software\Microsoft\Windows NT\CurrentVersion         OS Information             
    HKLM\Software\Microsoft\Windows NT\CurrentVersion /v      Product name               
    HKLM\Software\Microsoft\Windows NT\CurrentVersion /v      Date of Install            
    HKLM\Software\Microsoft\Windows NT\CurrentVersion /v      Registered Owner           
    HKLM\Software\Microsoft\Windows NT\CurrentVersion /v      System Root                
    HKLM\System\CurrentControllerSet\Control\TimeZoneInfo     Time Zone                  
    HKLM\Software\Microsoft\Windows NT\CurrentVersion\E MRU   Mapped Network Drives      
    HKLM\System\MountedDevices                                Mounted Devices            
    HKLM\System\CurrentControllerSet\Enum\USBStor             USB Devices                
    HKLM\Security\Policy\PolAdTev                             Audit Policies             
    HKLM\Software                                             Installed Software(Machine)
    HKCU\Software                                             Installed Software(User)   
    HKCU\Software\Microsoft\Windows\CurrentVersion\Explo      Recent Documents           
    HKCU\Software\Microsoft\Windows\CurrentVersion\Explo      Recent User Locations      
    HKCU\Software\Microsoft\Internet Explorer\TypedURLs       Typed URLs                 
    HKCU\Software\Microsoft\Windows\CurrentVersion\Explo      MRU List                   
    HKCU\Software\Microsoft\Windows\CurrentVersion\Apple      Last Registry Key Accessed 
"#;

// windows cmd basic commands
pub const WINDOWS_CMD_BASICS: &str = r#"
    dir                             List files and folders
    cd <dir>                        Change directory to <dir>
    mkdir <dir>                     Create Directory <dir>
    rmdir <dir>                     Remove Directory <dir>
    copy <source> <target>          Copy <source> to <target>
    move <source> <target>          Move file from <source> to <target>
    ren <old> <new>                 Rename from <old> to <new>
    del <file>                      Delete <file>
    echo <text>                     Display <text>
    type <text.txt>                 Display contents of <text.txt>
    cls                             Clear contents of the screen
    ver                             Windows Version
    <drive>:                        Change drive, Ex: (D: )
    ipconfig /all                   Get your IP address
    sc query state=all              Show Services
    tasklist /m                     Show Services and processes
    taskkill /PID <pid> /F          Force kill process by ID
    assoc                           Show File Type Association
    cipher /w:<dir>                 Secure delete file or directory
    fc <file> <file>                File compare
    netstat -an                     Display currently open ports
    pathping                        Displays each hop in ping
    tracert                         Displays each hop and time
    powercfg                        Change power configuration
    chkdsk /f <drive>               Check and fix disk errors
    driverquery /FO list /v         List of drivers and status
    osk                             Onscreen keyboard
    shutdown -s -t 3600             Schedule shutdown for 3600 sec or 1 hr
"#;

// windows powershell basic commands
pub const WINDOWS_POWERSHELL_BASICS: &str = r#"
    Get-Content                         cat     Get contents of a file
    Get-Service                         gsv     Get Services
    Get-Process                         gps     Show Services and processes
    Stop-Process -Id <PID> - Force      kill    Force kill process by ID
    Clear-Content                       clc     Clear contents of a file
    Get-Command                         gc      Gets all commands
    Compare-Object (cat <f1>) (cat<f2>) compare Compare file f1 and f2
    Copy-Item                           cp      Copy an item
    Get-Member                          gm      Gets the properties and methods of objects.
    Invoke-WMIMethod                    iwmi    Calls Windows Management Instrumentation 
                                                (WMI) methods.
    cmd /c <command>                            Run command as windows command line
    Set-Aliassal                                Creates or changes an alias
    Select-Objectselect                         Selects objects or object properties
    ForEach-Object                      %       Performs an operation against each item in
                                                a collection of input objects.
    Where-Object                        ?       Selects objects from a collection based 
                                                on their property values.
"#;


// // UNIX
// "unix_network" => system_text(UNIX_NETWORK, "cyan"),
// "unix_sys_info" => system_text(UNIX_SYSTEMINFO, "cyan"),
// "unix_utility" => system_text(UNIX_UTILITY, "cyan"),
// "unix_command" => system_text(UNIX_COMMAND, "cyan"),
// "unix_misc" => system_text(UNIX_MISC, "cyan"),
// "unix_files" => system_text(UNIX_FILES, "cyan"),
// "unix_folders" => system_text(UNIX_FOLDERS, "cyan"),
// // WINDOWS
// "windows_files" => system_text(WINDOWS_FILES, "cyan"),
// "windows_reg" => system_text(WINDOWS_COMMON_REGISTER_LOCATIONS, "cyan"),
// "windows_cmd" => system_text(WINDOWS_CMD_BASICS, "cyan"),
// "windows_powershell" => system_text(WINDOWS_POWERSHELL_BASICS, "cyan"),