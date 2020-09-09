#! /bin/ruby
#-------------------------------------------------------------
#
#                     Linux Evil Toolkit
# 
#                       By Xx_VOID_xX
#
#-------------------------------------------------------------

module Core


    def getErro(msg = 'OUTPUT ', command = nil)

        line = "\n\n[+]----------------------------------------[+]\n\n"
        if command == false
            puts line; puts "#{msg}"
        end

    end

    def proxy(props)

        if props != nil; return 'proxychains'
        else; return ''; end
    
    end


    def server(port)

        system("ruby -run -e httpd . -p #{port}")
        getErro('SERVER NOT STARTED' , command)

    end

    def ngrok(port)

        system("ngro http -p #{port}")
        getErro('NGROK NOT STARTED' , command)

    end


    def metasploit()

        url = "curl https://raw.githubusercontent.com/rapid7/metasploit-omnibus/master/config/templates/metasploit-framework-wrappers/msfupdate.erb > msfinstall && chmod 755 msfinstall && ./msfinstall"
        cmd = system(url)
        getErro('METASPLOIT FAILED', cmd)

    end
        
    def install() 
    
        IO.foreach('./config/tools.conf',) do |line|  
            cmd = system("sudo pacman --noconfirm -S #{line}")
            getErro('INSTALL OUTPUT', command)
        end
    
    end

    def ssh(user, ip)
        
        command = "ssh #{user}@#{ip}"
        getErro('SSH OUTPUT', command)

    end    

    def telnet(ip)

        command = "telnet #{ip}"
        getErro('TELNET OUTPUT', command)

    end

    def dumptcp(pl, file)

        command = system("")
        getErro('TCPDUMP OUTPUT', command)

    end

    def mac(pl)
        
        command = system("ifconfig #{pl} down")
        getErro('FAILED SET IFCONFIG DOWN')
        command = system("macchanger -A cisco #{pl}")
        getErro('FAILED CHANGE MAC')
        command = system("ifconfig #{pl} up")
        getErro('FAILED SET IFCONFIG UP')

        
    end

    def footprint(host, opt = '', proxy = nil)
        
        line = "\n\n[+]----------------------------------------[+]\n\n"
        proxy = proxy(proxy)
        puts line; command = system("#{proxy} hping3 --icmp #{opt} #{host}")
        getErro('ICMP OUTPUT', command)
        puts line; command = system("#{proxy} hping3 --syn #{opt} #{host}")
        getErro('SYN OUTPUT', command)

    end

    def scanner(hst, proxy = nil)

        line = "\n\n[+]----------------------------------------[+]\n\n"
        
        proxy = proxy(proxy)
        puts line; command = system("#{proxy} dnsenum --enum #{hst} -f ./wordlist/dns2.txt")
        getErro('DNSENUM OUTPUT', command)
        puts line; command = system("#{proxy} nslookup #{hst}")
        getErro('NSLOOKUP OUTPUT', command)
        puts line; command = system("#{proxy} wohis -A #{hst}")
        getErro('WHOIS OUTPUT', command)
        puts line; command = system("#{proxy} nikto --host #{hst}")
        getErro('NIKTO OUTPUT', command)
        puts line; command = system("#{proxy} nmap -sS -sV -A --spoof-mac apple --data 64 #{hst}")
        getErro('NMAP SYN OUTPUT', command)
        puts line; command = system("#{proxy} nmap --udp -sV -A --spoof-mac apple --data 64 #{hst}")
        getErro('NMAP UDP OUTPUT', command)
        puts line; command = system("#{proxy} nmap -sA --spoof-mac apple --data 64 #{hst}")
        getErro('NMAP -sA OUTPUT', command)
        
        
      end

    def nmap(host, opt, proxy = nil)

        line = "\n\n[+]----------------------------------------[+]\n\n"
        proxy = proxy(proxy)
        puts line; command = system("#{proxy} nmap -sS -sV -A --spoof-mac apple --data 64 #{hst}")
        # Redundante porem diferente
        getErro('NMAP SYN OUTPUT', command)
        puts line; command = system("#{proxy} nmap --udp -sV -A --spoof-mac apple --data 64 #{hst}")
        getErro('NMAP UDP OUTPUT', command)
        puts line; command = system("#{proxy} nmap -sA --spoof-mac apple --data 64 #{hst}")
        getErro('NMAP -sA OUTPUT', command)
        
    end
    
end # END CORE MODULE
