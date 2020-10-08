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
    $docmentation = false
    $proxy = false
    $target = nil
    $ip     = nil

    # Init options and set target
    def INIT()
        while $target == nil || $ip == nil
            if $docmentation == false
                print "Enable documentation [true|false]: "; $docmentation = gets.chomp.to_b
                if $docmentation; puts "Documentation running"; end
            end
            if $proxy == false
                print "Enable proxy [true|false]: "; $proxy = gets.chomp.to_b
                if $proxy; puts "Proxy running"; end
            end
            # but
            puts "Set target dns name and/or ip address"
            if $target == nil
                print "Set Target URL"; $target = gets.chomp.to_s
                puts "Target set to #{$target}"
            end
            if $ip == nil
                print "Set Target IP"; $ip = gets.chomp.to_s
                puts "Target set to #{$ip}"
            
            else; puts "Set target, or die!"; end
        end
    end

    # Alias for system()
    def sys(props)
        if $docmentation = true
            date_time = 'TIME_00-00_DATE_01-01-1999_'
            doc_name = "#{date_time}documentation"
            system("#{line}  >> ./docs/#{doc_name}")
            system("echo date >> ")
            system("#{props} >> ./docs/#{doc_name}")
        else
            system("#{props}")
        end
    end

    # Extract files
    def extract(file_name)

        puts $line
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
    def compress(file_name, files, ext)
        
        puts $line
        if ext = "tar"
            sys("tar cf #{file_name}.tar #{files}")
        elsif ext = "gz"
            sys("tar czf #{file_name}.tar.gz #{files}")
        elsif ext = "bz2"
            sys("tar cjf #{file_name}.tar.bz2 #{files}")
        elsif ext = "zip"
            sys("gzip #{file_name}")
        else
            puts "not accepted this format"
        end

    end

    # Set cover your tracks (or not)
    def cover()

        puts $line

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


    # Web vul scanner
    def web_scanner($target)
        prRed($line)
        sys("whois #{target}")
        sys("whois #{target}")
        sys("whois #{target}")
        sys("whois #{target}")
        sys("whois #{target}")
        sys("whois #{target}")
        sys("whois #{target}")
        sys("whois #{target}")


    end
    
    def host_scanner($target)
        
    end
    
    def dns_scanner($target)
        commands = ["whois -a #{target}", "host #{target}", "host -t ns #{target}"]
        for ip in  do
            
        end
        
    end

    def dir_scanner($target)
        
    end


end
