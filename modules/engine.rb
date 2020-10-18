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
    # Line Line Line Line Line Line Line Line
    $line = "\n\n[+]---------------------------------------[+]\n\n"
    
    # GLOBALS
    $docmentation = false
    $proxy        = false
    $doc_name     = nil
    $target       = nil
    $ip           = nil

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
            # but, but, but, but, fuck but!
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

    # Reset to default values
    def R()
        system("clear && reset")
        # Default values or not
        $docmentation = false
        $proxy = false
        $doc_name = nil
        $target = nil
        $ip = nil
    end

    # Alias for system(), why?
    def sys(props)
        if $proxy == true
            cmd = system("proxychains #{props}")
            if cmd == false; puts "[COMMAND_ERROR] #{cmd}"; end
        elsif $proxy == false
            cmd = system("#{props}")
            if cmd == false; puts "[COMMAND_ERROR] #{cmd}"; end
        elsif $docmentation == true && $proxy == false
            doc_name = "#{Time.now.strftime("%d-%m-%Y_%H-%M")}_documentation"
            cmd = system("#{props}  >> ./docs/#{doc_name}")
            if cmd == false; puts "[COMMAND_ERROR] #{cmd}"; end
        else
            puts "[ERROR]: UNDEFINED"; exit
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
        # Clear
        puts "[+] Clear auth log"
        sys('echo "" /var/log/auth.log')
        # History
        puts "[+] Clear bash_history"
        sys('echo "" -/.bash_history')
        sys('rm -rf ~/.bash_history')
        puts "[+] Clear history"
        sys('history -c')
        # Disable history
        puts "[+] Disable history"
        sys('export HISTFILESIZE=O')
        sys('export HISTSIZE=O')
        sys('unset HISTFILE')
        # kill your sel... session
        puts "[+] Kill session";
        sys('kill -9 $$')
        # No history, (UwU)
        puts "[+] Perrnanentlj send all bash history
        commands to /dev/null"
        sys('ln /dev/null -/.bash_historj -sf')
        puts "\n\n"
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
        puts "WHOIS"; sys("whois -a #{$target}")
        puts "Test connection"; sys("ping -c4 #{$ip}")
        puts "Email Enumeration"; sys("theharvester -d #{$target} -l 500 -b all")
        puts "HTTP Banner grep"; sys("ncat -v #{$ip} 80")
        puts "HTTPS Banner grep"; sys("openssl s_client -quiet -connect #{$target}:443")
    end

    # Web dns scanner
    def dns_scanner()
        puts "DNS Enumeration"
        sys("dnsenum --enum #{$target} ./wordlist/dns2.txt") 
        reg_dns = ['A', 'AAAA', 'CNAME', 'MX', 'NS', 'PTR', 'SOA']
        for reg in reg_dns
            puts "Search for #{reg} in #{$target}"
            sys("host -t #{reg} #{$target}")
        end
    end

    # Web directory scanner
    def dir_scanner()
       puts "scanner" 
    end

end
