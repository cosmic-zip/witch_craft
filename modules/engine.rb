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


    def prRed(s)
        puts s
    end

    # Line Line Line Line Line Line Line Line
    $line = "\n\n[+]---------------------------------------[+]\n\n"

    # GLOBALS
    $docmentation   = false
    $proxy          = false
    $doc_name       = nil
    $target         = nil
    $ip             = nil

    # Init options and set target
    def INIT()
        while $target == nil || $ip == nil
            if $docmentation == false
                print "Enable documentation [true|false]: "; $docmentation = gets.chomp
                if $docmentation == true; puts "Documentation running"; end
            end
            if $proxy == false
                print "Enable proxy [true|false]: "; $proxy = gets.chomp
                if $proxy == true; puts "Proxy running"
                else; $proxy = ''; end
            end
            # but
            puts "Set target dns name and/or ip address"
            if $target == nil
                print "Set Target URL: "; $target = gets.chomp.to_s
                puts "Target set to #{$target}"
            end
            if $ip == nil
                print "Set Target IP: "; $ip = gets.chomp.to_s
                puts "Target set to #{$ip}"
            
            else; puts "Set target, or die!"; end
        end
    end

    def R()
        system("clear && reset")
    end

    # Alias for system()
    def sys(props)
        
        if $proxy == true
            system("proxychains #{props}")
        elsif $proxy == false
            system("#{props}")
        elsif $docmentation == true && $proxy == false
            puts $docmentation
            date_time = Time.now.strftime("%d-%m-%Y_%H-%M")
            doc_name = "#{date_time}_documentation"
            form = 
            system("#{line}  >> ./docs/#{doc_name}")
            system("echo date >> ./docs/#{doc_name}")
            system("#{props} >> ./docs/#{doc_name}")
        else
            puts "[ERROR]: globals varibles is invalid"
        end
    end

    # Extract files
    def extract()
        prRed($line)
        print "Set file name: "; file_name = gets.chomp.to_s
        if File.extname(file_name) == ".tar"
            sys("tar xf #{file_name}.tar")
        elsif File.extname(file_name) == ".gz"
            sys("tar xzf #{file_name}.tar.gz")
        elsif File.extname(file_name) == ".bz2"
            sys("tar xjf #{file_name}.tar.bz2")
        elsif File.extname(file_name) == ".zip"
            sys("unzip #{file_name}")
        else
            puts "not accepted this format"
        end
    end
    
    # Compress files
    def compress()
        print "Set file name: "; file_name = gets.chomp.to_s
        print "Set output file name: "; files = gets.chomp.to_s
        print "Set format [tar/gz/bz2/zip]: "; ext = gets.chomp.to_s
        prRed($line)
        if ext == "tar"
            sys("tar cf #{file_name}.tar #{files}")
        elsif ext == "gz"
            sys("tar czf #{file_name}.tar.gz #{files}")
        elsif ext == "bz2"
            sys("tar cjf #{file_name}.tar.bz2 #{files}")
        elsif ext == "zip"
            sys("gzip #{file_name}")
        else
            puts "not accepted this format"
        end
    end

    # Set cover your tracks (or not)
    def cover()
        prRed($line)
        puts "[+] Clear auth log"
        sys('echo "" /var/log/auth.log')

        puts "[+] Clear bash_history"
        sys('echo "" -/.bash_history')
        sys('rm -rf ~/.bash_history')

        puts "[+] Clear history"
        sys('history -c')

        puts "[+] Disable history"
        sys('export HISTFILESIZE=O')
        sys('export HISTSIZE=O')
        sys('unset HISTFILE')

        puts "[+] Kill session";
        sys('kill -9 $$')

        puts "[+] Perrnanentlj send all bash history
        commands to /dev/null"
        sys('ln /dev/null -/.bash_historj -sf')

        puts "\n\n"
    end

    # Status
    def status()
       prRed("\n::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::")
       date_time = Time.now.strftime("%d-%m-%Y_%H-%M")
       prRed("\n[+] #{date_time} [+]"); 
       prGreen("\n[+] Memory:\n")
       system("free -l")
       prGreen("\n[+] Machine:\n")
       system("uname -a")
       prGreen("\n[+] Temp:\n")
       system("sensors")
       prRed("\n::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::")

    end


    # Web vul scanner
    def search()
        prRed($line)
        puts "WHOIS"
        sys("whois -a #{$target}")
        puts "Test connection"
        sys("ping -c4 #{$ip}")
        puts "Email Enumeration"
        sys("theharvester -d #{$target} -l 500 -b all")
        puts "HTTP Banner grep"
        sys("ncat -v #{$ip} 80")
        puts "HTTPS Banner grep"
        sys("openssl s_client -quiet -connect #{$target}:443")
        puts ""
    end
    
    def port_scanner()
        prRed($line)    
        spoof_mac = ''
        print "Spoof mac? [yes|no]: "; spoof_mac = gets.chomp.to_s
        print "Set option [type help for help, or not]: "; opt = gets.chomp.to_s
        if spoof_mac == 'yes'; spoof_mac = '--spoof-mac cisco'; end
        print "Add nmap custom flag: [type enter for skip]: "; custom = gets.chomp.to_s
        if opt == 'help'
            prRed($line); puts "HELP: "
            puts "Local eojfejofeoejofeoj"
        elsif opt == 'local' 
            puts $line; puts "LocalHost scan"
            sys("nmap #{custom_flag} 127.0.0.1")
        elsif opt == 'list-scan'
            puts $line; puts "List Scan"
            sys("nmap -sS -sL #{spoof_mac} #{$ip}")
        elsif opt == 'no-ping'
            puts $line; puts "No ping scan"
            sys("nmap #{custom_flag} -PN #{spoof_mac} #{$ip} ")
        elsif opt == 'scan'
            puts $line; puts "Default TCP scan"
            sys("nmap #{custom_flag} -sS -sv -A -O #{spoof_mac} #{$ip}")
            puts "Default FYN scan"
            sys("nmap -sF #{$ip}")
        elsif opt == 'udp-scan'
            puts $line; puts "Default UDP scan"
            sys("nmap #{custom_flag} -sUV #{spoof_mac} #{$ip}")
        elsif opt == 'all'            
            puts $line; puts "LocalHost scan"
            sys("nmap #{custom_flag} 127.0.0.1")
            puts "List Scan"
            sys("nmap #{custom_flag} -sS -sL #{spoof_mac} #{$ip}")
            puts "No ping scan"
            sys("nmap #{custom_flag} -PN #{spoof_mac} #{$ip} ")
            puts "Default TCP scan"
            sys("nmap #{custom_flag} -sS -sv -A -O #{spoof_mac} #{$ip}")
            puts "Default UDP scan"
            sys("nmap #{custom_flag} -sUV #{spoof_mac} #{$ip}")
        else 
            prRed("[ERROR]: invalid option")
        end         
            
    end
    
    def dns_scanner()
        puts "DNS Enumeration"
        sys("dnsenum --enum #{$target} ./wordlist/dns2.txt") 
        reg_dns = ['A', 'AAAA', 'CNAME', 'MX', 'NS', 'PTR', 'SOA']
        for reg in reg_dns
            puts "Search for #{reg} in #{$target}"
            sys("host -t #{reg} #{$target}")
        end
    end

    def dir_scanner()
       puts "scanner" 
    end


end
