#-------------------------------------------------------------
#
#                     Linux Evil Toolkit
# 
#                          By v0id
#
#
#-------------------------------------------------------------

module Engine

    # LINE
    $line = "\n\n[+]----------------------------------------------------[+]\n\n"
    $pline ="\n ::::::::::::::::::::::::::::::::::::::::::::::::::::::::::"

    # INIT options and set target
    def INIT()
        while $target == nil && $ip == nil
            if $documentation == false
                print "Enable documentation? [yes|no]: "
                $documentation = gets.chomp.to_s
                $documentation == 'yes' ? $documentation = true : nil
            end
            if $proxy == false
                print "Enable proxy? [yes|no]: "
                $proxy = gets.chomp.to_s
                $proxy == 'yes' ? $proxy = true : $proxy = ''
            end
            prCyan "Set target dns name and/or ip address"
            if $target == false
                print "Set Target URL: "; $target = gets.chomp.to_s
                $target != nil ? prYellow("Target set to #{$target}") : prRed("No target")
            end
            if $ip == false
                print "Set Target IP: "; $ip = gets.chomp.to_s
                $proxy != nil ? prYellow("Target set to #{$ip}") : prRed("No target ip")
            end
            if $change_mac == false
                print "Set network interface name: [wlan0, wlp2s0]: "; $interface = gets.chomp.to_s
                command = sys("ip link set #{$interface} down")
                command = sys("macchanger -r #{$interface}")
                command = sys("ip link set #{$interface} up")
                $command == false ? prYellow("Change mac address") : prRed("[ERROR]: Interface not found")
            else
                prYellow "Set target, or die!"
            end
        end
    end

    # Reset to default values
    def R()
        system("clear && reset")
        $documentation = false
        $proxy         = false
        $target        = false
        $ip            = false
        $silent        = false
        $change_mac    = false
        prCyan "\nClear console and reset values...[done]"
    end

    # Alias for system(), why?
    def sys(props)
        if $silent == true
            cmd = system("proxychains #{props}  >> /dev/null")
            cmd ? nil : prRed("[SYS_COMMAND_ERROR]: #{cmd}")
            system('systemctl restart tor')
        elsif $proxy == false && $documentation == false
            cmd = system("#{props}")
            cmd == false ? nil : prRed("[SYS_COMMAND_ERROR]: #{cmd}\n")
        elsif $proxy == true && $documentation == false
            cmd = system("proxychains #{props}")
            cmd ? nil : prRed("[SYS_COMMAND_ERROR]: #{cmd} | proxy fail?\n")
        elsif $poxy == false && $documentation == true
            cmd = system("#{props}  >> #{}_documentation_no_proxy.txt")
            cmd ? nil : prRed("[SYS_COMMAND_ERROR]: #{cmd}")
        else 
            cmd = system("proxychains #{props}  >> #{$time}_documentation_with_proxy.txt")
            cmd ? nil : prRed("[SYS_COMMAND_ERROR]: #{cmd}")
        end
    end

    # Extract files
    def extract()
        prRed($line)
        print "Set file name: "; file_name = gets.chomp.to_s
        msg = "[ERROR]: File extension fail"
        File.extname(file_name) == ".tar" ? sys("tar xf #{file_name}.tar") : prCyan(msg)
        File.extname(file_name) == ".gz" ? sys("tar xzf #{file_name}.tar.gz") : prCyan(msg)
        File.extname(file_name) == ".bz2" ? sys("tar xjf #{file_name}.tar.bz2") : prCyan(msg)
        File.extname(file_name) == ".zip" ? sys("unzip #{file_name}") : prCyan(msg)
    end
    
    # Compress files
    def compress()
        print "Set file name: "; file_name = gets.chomp.to_s
        print "Set output file name: "; files = gets.chomp.to_s
        print "Set format [tar/gz/bz2/zip]: "; ext = gets.chomp.to_s
        msg = "[ERROR]: wrong parameters"
        prRed($line)
        ext == "tar" ? sys("tar cf #{file_name}.tar #{files}"): prCyan(msg)
        ext == "gz" ? sys("tar czf #{file_name}.tar.gz #{files}"): prCyan(msg)
        ext == "bz2" ? sys("tar cjf #{file_name}.tar.bz2 #{files}"): prCyan(msg)
        ext == "zip" ? sys("gzip #{file_name}"): prCyan(msg)
    end

    # Set cover your tracks (or yes)
    def cover()
        prRed($line)
        # Clear
        prCyan "[+] Clear auth log"
        sys('srm -rfD /var/log/')
        # History
        prCyan "[+] Clear bash_history"
        sys('srm -rfD -/.bash_history')
        prCyan "[+] Clear history"
        sys('history -c')
        # Disable history
        prCyan "[+] Disable history"
        sys('export HISTFILESIZE=O')
        sys('export HISTSIZE=O')
        sys('unset HISTFILE')
        # kill your sel... session
        # No history, (UwU)
        prCyan "[+] Perrnanentlj send all bash history
        commands to /dev/null"
        sys('ln /dev/null ~/.bash_history -sf')
        puts "\n\n"
    end

    # Machine status
    def status()
       prRed $pline
       prRed "\n[+] #{$time} [+]"
       prGreen "\n[+] Memory:\n"
       system("free -lh")
       prGreen "\n[+] Machine:\n"
       system("uname -a")
       prGreen "\n[+] Temp:\n"
       system("sensors")
       prRed $pline
    end

    # Web vul scanner
    def search()
        prRed($line)
        prYellow "#{$line}[+] WHOIS"; sys("whois -a #{$target}")
        prYellow "#{$line}[+] Test connection"; sys("ping -c4 #{$ip}")
        prYellow "#{$line}[+] Email Enumeration"; sys("theharvester -d #{$target} -l 500 -b all")
        prYellow "#{$line}[+] HTTP Banner grep"; sys("ncat -v #{$ip} 80")
        prYellow "#{$line}[+] HTTPS Banner grep"; sys("openssl s_client -quiet -connect #{$target}:443")
        prYellow "#{$line}[+] Nikto scanner"; sys("nikto -h #{$ip}:443 -ssl")
    end

    # Web dns scanner
    def dns_scanner()
        prYellow "#{$line}DNS Enumeration"
        sys("dnsenum --enum #{$target} ./wordlist/dns2.txt") 
        reg_dns = ['A', 'AAAA', 'CNAME', 'MX', 'NS', 'PTR', 'SOA']
        for reg in reg_dns
            prYellow "Search for #{reg} in #{$target}"
            sys("host -t #{reg} #{$target}")
        end
    end

    # Web directory scanner
    def dir_scanner()
        prCyan $line
        prCyan "======================== HOTKEYS ========================"
        prCyan " 'n' -> Go to next directory."
        prCyan " 'q' -> Stop scan. (Saving state for resume)"
        prCyan " 'r' -> Remaining scan stats.\n\n#{$line}\n"

        print('Set custom config? [yes|no]: '); opt = gets.chomp.to_s
        if opt == 'yes'
            print('Show terminal dump? [yes|no]: '); silent = gets.chomp.to_s
            silent == 'yes' ? silent = '-S' : silent = ''
            print('Write output in file? [yes|no]: '); output = gets.chomp.to_s
            output == 'yes' ? output = '-o ./dir_dump.txt' : output = ''
            print('Search for extension? [.type|no]: '); ext = gets.chomp.to_s
            extension != 'no' ?  extension = "-x #{ext}" : extension = ''
            print('Time delay mode "1: normal |2: slow |3: paranoid" [1|2|3]: ')
            delay = gets.chomp.to_i
            case delay
            when 1
                delay = '0'
            when 2
                delay = '500' 
            when 3
                delay = '2000'
            else
                delay = '100'
            end
            flags = ' -i -w '+silent+' '+output+' '+extension+' -z '+delay+'' 
            sys("dirb #{$target} ./wordlist/directory_list.txt #{flags}")
        else  
            sys("dirb -i -w -S #{$target} ./wordlist/directory_list.txt")
        end 
    end 

    def assembly(string, timing, ipv6)
        # Define hidden flag
        mac_vendor = [ "006017", "006018", "006019", "00601A", "00601B", "00601C", "00601D", "00601E", "00601F", "006020", "006021", "006022", "006023", "006024", "006025", "006026" ]
        sys "nmap #{string} -O -T#{timing} #{$target} --spoof-mac #{mac_vendor[rand(0..15)]} --data-length #{rand(2..256)} --max-retries 10 --mtu 1024 --host-timeout 30 --ttl 60 -f #{rand(1..6)} #{ipv6 != nil ? '-6' : ''}"
    end

    def exec(list)
        for opt in list
            prYellow "[EXEC]: #{opt[1]}"
            cmd = assembly(opt[0])
            cmd == true ? prGreen("[SYS_COMMAND]: Done") : prRed("[COMMAND_ERROR]: Fail")
        end
    end
  
    def simple_scan(props)

        alone = [
            ["-sL", "List Scan - simply list targets to scan"],
            ["-sP", "Ping Scan - go no further than determining if host is online"],
        ]

        default = [
            ["-sS -sV", "TCP SYN"],
            ["-sU -sV", "UDP Scan"],
        ]


        icmp_echo = [
            ["-sS -sV -PE", "TCP SYN + ICMP echo discovery probes"],
            ["-sU -sV -PE", "UDP Scan + ICMP echo discovery probes"],
            ["-sA -sV -PE", "ACK + ICMP echo discovery probes"],
        ]
            
        port_list = [
            ["-sS", "TCP SYN + [portlist]: TCP SYN discovery probes to given ports"],
            ["-sA", "ACK + [portlist]: TCP ACK discovery probes to given ports"],
            ["-sU", "UDP Scan + [portlist]: TCP UDP discovery probes to given ports"],
        ]
            
        special = [
            ["-sT -sV", "Connect()"],
            ["-sW -sV", "Window"],
            ["-sM -sV", "Maimon scans"],
            ["-sN -sV", "TCP Null"],
            ["-sF -sV", "FIN"],
            ["-sX -sV", "Xmas scans"]
        ]

        case props
        when 'alone'
            exec(alone)
        when 'default'
            exec(default)
        when 'icmp_echo'
            exec(icmp_echo)
        when 'port_list'
            exec(port_list)
        when 'special'
            exec(special)
        else 
            prRed "[COMMAND_ERROR]: Invalid option"
        end
    end

end
