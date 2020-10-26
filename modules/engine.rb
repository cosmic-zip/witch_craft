#-------------------------------------------------------------
#
#                     Linux Evil Toolkit
# 
#                          By v0id
#
#                        2019 - 2020
#
#-------------------------------------------------------------

module Engine

    # LINE
    $line = "\n\n[+]---------------------------------------[+]\n\n"

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
    end

    # Alias for system(), why?
    def sys(props)
        if $proxy == false && $documentation == false
            cmd = system("#{props}")
            cmd == false ? nil : prRed("[SYS_COMMAND_ERROR]: #{cmd}\n")
        elsif $proxy == true && $documentation == false
            cmd = system("proxychains #{props}")
            cmd ? nil : prRed("[SYS_COMMAND_ERROR]: #{cmd} | proxy fail?\n")
        elsif $poxy == false && $documentation == true
            time = Time.now.strftime("%d-%m-%Y_%H-%M")
            cmd = system("#{props}  >> #{}_documentation_no_proxy.txt")
            cmd ? nil : prRed("[SYS_COMMAND_ERROR]: #{cmd}")
        else 
            time = Time.now.strftime("%d-%m-%Y_%H-%M")
            cmd = system("proxychains #{props}  >> #{}_documentation_with_proxy.txt")
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

    # Set cover your tracks (or not)
    def cover()
        prRed($line)
        # Clear
        prCyan "[+] Clear auth log"
        sys('echo "" /var/log/auth.log')
        # History
        prCyan "[+] Clear bash_history"
        sys('echo "" -/.bash_history')
        sys('rm -rf ~/.bash_history')
        prCyan "[+] Clear history"
        sys('history -c')
        # Disable history
        prCyan "[+] Disable history"
        sys('export HISTFILESIZE=O')
        sys('export HISTSIZE=O')
        sys('unset HISTFILE')
        # kill your sel... session
        prCyan "[+] Kill session";
        sys('kill -9 $$')
        # No history, (UwU)
        prCyan "[+] Perrnanentlj send all bash history
        commands to /dev/null"
        sys('ln /dev/null -/.bash_historj -sf')
        prCyan "\n\n"
    end

    # Machine status
    def status()
       $pline ="\n::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::"
       prRed($pline)
       date_time = Time.now.strftime("%d-%m-%Y_%H-%M")
       prRed("\n[+] #{date_time} [+]"); 
       prGreen("\n[+] Memory:\n")
       system("free -lh")
       prGreen("\n[+] Machine:\n")
       system("uname -a")
       prGreen("\n[+] Temp:\n")
       system("sensors")
       prRed($pline)

    end

    # Web vul scanner
    def search()
        prRed($line)
        prYellow "WHOIS"; sys("whois -a #{$target}")
        prYellow "Test connection"; sys("ping -c4 #{$ip}")
        prYellow "Email Enumeration"; sys("theharvester -d #{$target} -l 500 -b all")
        prYellow "HTTP Banner grep"; sys("ncat -v #{$ip} 80")
        prYellow "HTTPS Banner grep"; sys("openssl s_client -quiet -connect #{$target}:443")
    end

    # Web dns scanner
    def dns_scanner()
        prYellow "DNS Enumeration"
        sys("dnsenum --enum #{$target} ./wordlist/dns2.txt") 
        reg_dns = ['A', 'AAAA', 'CNAME', 'MX', 'NS', 'PTR', 'SOA']
        for reg in reg_dns
            prYellow "Search for #{reg} in #{$target}"
            sys("host -t #{reg} #{$target}")
        end
    end

    # Web directory scanner
    def dir_scanner()
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

end
