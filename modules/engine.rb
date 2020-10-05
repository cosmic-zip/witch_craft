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
    $line = "\n\n[+]----------------------------------------------[+]\n\n"
    $proxy = false

    # Alias for system()
    def sys(props)
        system("#{props}")
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

    # init, web_scanner, host_scanner, dns_scanner, dir_scanner

end
