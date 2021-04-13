import osproc
import strformat

proc sys*(command: string; verbose: bool = false): string = 
    try:
        var a = execCmdEx(command)
        if verbose == true:
            echo a[0]
        return "true"
    except:
        echo "[ERROR_SYS_FUNTION]"
        return "false"

proc get_status*(val: bool): void = 
    if val == true:
        echo "[:: SUCCESS ::]"
    else:
        echo "[:: ERROR :: ]"

proc writeFile*(input: string): bool =

    const PATH = "~/hackerman_outputs"

proc helper*(): void = 

    let init_script = "mkdir ~/hackerman_outputs"
    let get_my_ip = "curl ifconfig.me \n"
    let linux_alive = "ls -lct /etc | tail -1 | awk '{print $6, $7, $8}'"

proc server*(option: string = "start"; host: string = "127.0.0.1"; port: string = "8000"; path: string = "./wordlist/dns.txt"): bool = 
    
    const PATH = "/opt/lampp/lampp"

    if option == "start" or option == "":
        discard sys(fmt("{PATH} {option}"))

    elif option == "stop" or option == "break":
        discard sys(fmt("{PATH} {option}"))

    elif option == "restart" or option == "reboot":
        discard sys(fmt("{PATH} {option}"))

    elif option == "php":
        discard sys(fmt("php -S {host}:{port}"))

    else: 
        return false

    return true

proc dns*(host: string; ip: string; option: string): bool = 

    if option == "norever" or option == "all":
        echo "[+] :: Noreverse dns search"
        discard sys(fmt"dnsenum --noreverse {host}")

    elif option == "robots" or option == "all":
        echo "[+] :: Get robots.txt"
        discard sys(fmt"wget https://www.{host}/robots.txt", true)

    elif option == "whois" or option == "all":
        echo "[+] :: WHOIS"
        discard sys(fmt"whois -a #{host}", true)

    elif option == "banner80" or option == "all":
        echo "[+] :: HTTP Banner grep at ~/outputs" 
        discard sys(fmt"ncat -v #{host} 80 >> ~/hackerman_outputs/banner_port_80_#{host}_.html", true)

    elif option == "banner443" or option == "all":
        echo "[+] :: HTTPS Banner grep" 
        discard sys(fmt"openssl s_client -quiet -connect #{ip}:443 >> ~/hackerman_outputs/banner_port_443_#{$ip}_.html", true)

    elif option == "nikto" or option == "all":
        echo "[+] :: Nikto scanner" 
        discard sys(fmt"nikto -h #{ip}:443 -ssl", true)

    elif option == "brute" or option == "all":
        echo "[+] :: Dns brute force"
        discard sys("dnsenum --enum #{host} {path}", true) 

    elif option == "dnsearch" or option == "all":
        const reg_dns = ["A", "AAAA", "CNAME", "MX", "NS", "PTR", "SOA"]
        for reg in reg_dns:
            echo "[+] :: Search for #{reg} in #{$target}:"
            discard sys(fmt"host -t {reg} {host}", true)

    else: 
        echo "[ERROR] No option"
        return false
    
    return true 

proc post_explorer_linux*(write_output = false): bool = 

    echo "[+] :: Local users hashes             " 
    discard writeFile(sys("/etc/shadow"))

    echo "[+] :: Local users                    " 
    discard writeFile(sys("/etc/passwd"))

    echo "[+] :: Local groups                   " 
    discard writeFile(sys("/etc/group"))

    echo "[+] :: Startup services               " 
    discard writeFile(sys("/etc/rc.d"))

    echo "[+] :: Service                        " 
    discard writeFile(sys("/etc/init.d"))

    echo "[+] :: Known hostnames and IPs        " 
    discard writeFile(sys("/etc/hosts"))

    echo "[+] :: Full hostnarne with domain     " 
    discard writeFile(sys("/etc/HOSTNAl1E"))

    echo "[+] :: Network configuration          " 
    discard writeFile(sys("/etc/network/interfaces"))

    echo "[+] :: System environment variables   " 
    discard writeFile(sys("/etc/profile"))

    echo "[+] :: Ubuntu sources list            " 
    discard writeFile(sys("/etc/apt/sources.list"))

    echo "[+] :: Narneserver configuration      " 
    discard writeFile(sys("/etc/resolv.conf"))

    echo "[+] :: Bash history (also /root/)     " 
    discard writeFile(sys("/horne/ user /.bash historj"))

    echo "[+] :: Vendor-mac  lookup             " 
    discard writeFile(sys("/usr/share/wireshark/rnanuf"))

    echo "[+] :: SSH keystore                   " 
    discard writeFile(sys("-/.ssh/"))

    echo "[+] :: System log files (most Linux)  " 
    discard writeFile(sys("/var/log"))

    echo "[+] :: System log files (Unix)        " 
    discard writeFile(sys("/var/adrn"))

    echo "[+] :: List cron files                " 
    discard writeFile(sys("/var/spool/cron"))

    echo "[+] :: Apache connection log          " 
    discard writeFile(sys("/var/log/apache/access.log"))

    echo "[+] :: Static file system info        " 
    discard writeFile(sys("/etc/fstab"))
