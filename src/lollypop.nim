import osproc
import strformat

proc sys*(command: string; verbose: bool = false): bool = 
    try:
        var a = execCmdEx(command)
        if verbose == true:
            echo a[0]
        return true
    except:
        echo "system_exec_command_error"
        return false

proc get_status*(val: bool): void = 
    if val == true:
        echo "[:: SUCCESS ::]"
    else:
        echo "[:: ERROR :: ]"

proc helper*(): void = 

    let get_my_ip = "curl ifconfig.me \n"
    let linux_alive = "ls -lct /etc | tail -1 | awk '{print $6, $7, $8}'"

proc server*(option: string = "start"; port: string = "127.0.0.1"; path: string = "./wordlist/dns.txt"): bool = 
    
    const PATH = "/opt/lampp/lampp"
    var value = false

    if option == "start" or option == "":
        value = sys(fmt("{PATH} {option}"))

    elif option == "stop" or option == "break":
        value = sys(fmt("{PATH} {option}"))

    elif option == "restart" or option == "reboot":
        value = sys(fmt("{PATH} {option}"))

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
        discard sys(fmt"ncat -v #{host} 80 >> ~/banner_port_80_#{host}_.html", true)

    elif option == "banner443" or option == "all":
        echo "[+] :: HTTPS Banner grep" 
        discard sys(fmt"openssl s_client -quiet -connect #{ip}:443 >> ~/banner_port_443_#{$ip}_.html", true)

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