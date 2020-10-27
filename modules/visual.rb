#-------------------------------------------------------------
#
#                     Linux Evil Toolkit
# 
#                          By v0id
#
#
#-------------------------------------------------------------

module Visual
     
    def banner()
        puts "\n"
        prRed(' _      _                    ______     _ _   _______          _ _    _ _   ')
        prRed('| |    (_)                  |  ____|   (_) | |__   __|        | | |  (_) |  ')
        prRed('| |     _ _ __  _   ___  __ | |____   ___| |    | | ___   ___ | | | ___| |_ ')
        prRed("| |    | | '_ \\| | | \\ \\/ / |  __\\ \\ / / | |    | |/ _ \\ / _ \\| | |/ / | __|")
        prRed("| |____| | | | | |_| |>  <  | |___\  V /| | |    | | (_) | (_) | |   <| | |_ ")
        prRed("|______|_|_| |_|\\__,_/_/\\_\\ |______\\_/ |_|_|    |_|\\___/ \\___/|_|_|\\_\\_|\\__|")
        puts "\n\n"
        prLightPurple("help for more commands"); puts "\n\n"
  
    end 


    def web_dns()
  
        prRed("Netcraft      – endereços fora do Brasil      http://news.netcraft.com/")
        prRed("Domaintools   - whois, lookup, IP, etc.       http://www.domaintools.com/")
        prRed("Registro BR   – endereços no Brasil           https://registro.br/cgi-bin/whois/")
        prRed("Arin          – endereços fora do Brasil      https://www.arin.net/")
        prRed("Apnic         - endereços Ásia e Pacífico     http://www.apnic.net/apnic-info/search")
        prRed("Whois         – endereços fora do Brasil      http://new.whois.net/")
        prRed("Ripe          – endereços europeus            http://www.ripe.net/")
        puts "\n\n"
      
    end

    def linux_files()

        prRed($line)
        prCyan("Local users' hashes              /etc/shadow")
        prCyan("Local users                      /etc/passwd")
        prCyan("Local groups                     /etc/group" )
        prCyan("Startup services                 /etc/rc.d")
        prCyan("Service                          /etc/init.d")
        prCyan("Known hostnames and IPs          /etc/hosts" )
        prCyan("Full hostnarne with domain       /etc/HOSTNAl1E" )
        prCyan("Network configuration            /etc/network/interfaces")
        prCyan("System environment variables     /etc/profile" )
        prCyan("Ubuntu sources list              /etc/apt/sources.list")
        prCyan("Narneserver configuration        /etc/resolv.conf" )
        prCyan("Bash history (also /root/)       /horne/ user /.bash historj")
        prCyan("Vendor-t1AC lookup               /usr/share/wireshark/rnanuf")
        prCyan("SSH keystore                     -/.ssh/")
        prCyan("System log files (most Linux)    /var/log" )
        prCyan("System log files (Unix)          /var/adrn")
        prCyan("List cron files                  /var/spool/cron")
        prCyan("Apache connection log            /var/log/apache/access.log")
        prCyan("Static file system info          /etc/fstab")
        puts "\n\n"

    end

    def linux_folders()

        prGreen("User binaries                      /bin")
        prGreen("Boot-up related files              /boot")
        prGreen("Interface for system devices       /dev")
        prGreen("Sjstern configuration files        /etc")
        prGreen("Base directory for user files      /horne")
        prGreen("Critical software libraries        /lib")
        prGreen("Third party software               /opt")
        prGreen("Sjstern and running programs       /proc")
        prGreen("Home directory of root user        /root")
        prGreen("System administrator binaries      /sbin")
        prGreen("Temporary files                    /trnp")
        prGreen("Less critical files                /usr")
        prGreen("Variable Sjstern files             /var")
        puts "\n\n"

    end
 
    def linux_util()

        prYellow("Grab url                      wget http:// url -0 url.txt -o /dev/null")
        prYellow("Remote Desktop to ip          desktop ip")
        prYellow("Put file                      cp /tmp/file user@x.x.x.x:/tmp/file")
        prYellow("Get file                      cp user@ remoteip :/tmp/file /tmp/file")
        prYellow("Add user                      seradd -m user")
        prYellow("Change user password          asswd user")
        prYellow("Remove user                   muser unarne")
        prYellow("Record shell : Ctrl-D stops   cript -a outfile")
        prYellow("Find related command          propos subject")
        prYellow("View users command history    istory")
        prYellow("Executes line # in history    num")
        puts "\n\n"

    end

    def help()
        puts $line
        prRed('|exit           |   Close this script                                           ')
        prRed('|clear          |   Clear terminal                                              ')
        prRed('|update         |   Update Linux evil toolkit                                   ')
        prRed('|train          |   Show train in terminal, tuutuu                              ')
        prRed('|INIT           |   Setup global variables                                      ')
        prRed('|reset          |   Clear terminal and reset global variables                   ')
        prRed('|cover          |   Cover your tracks on your computer                          ')
        prRed('|portscanner    |   This command is replaced by automap                         ')
        prRed('|automap        |   Scanner target, grep ports, services, operational system,   ')
        prRed('|               |   firewall rules and more.                                    ')
        prRed('|search         |   Search email, whois and banner grep                         ')
        prRed('|status         |   Show machine status                                         ')
        prRed("|dnsscanner     |   Scan for 'A', 'AAAA', 'CNAME', 'MX', 'NS', 'PTR', 'SOA'     ")
        prRed('|dirscanner     |   Scan files and folders                                      ')
        prRed('|banner         |   Show Linux evil Toolkit banner in terminal                  ')
        prRed('|webdns         |   Show Web Sites for dns scanner                              ')
        prRed('|linuxfiles     |   Show important linux files                                  ')
        prRed('|linuxfolders   |   Show important linux folders                                ')
        prRed('|linuxutil      |   Show useful commands in linux                               ')
        prRed('|test           |   For development only                                        ')
        puts "\n\n"
    end
        
end
