#-------------------------------------------------------------
#
#                     Linux Evil Toolkit
# 
#                          By v0id
#
#                        2019 - 2020
#
#-------------------------------------------------------------

module Shell

    
    def PS1()
        print("\033[93m[ANON]::[LETK]:: \033[00m")
        props = gets.chomp.to_s

    end

    def props(msg)

        print("#{msg}: ")
        props = gets.chomp.to_s
    
    end

    def interpreter(command)

        if command == 'server'
            port = props('SET HTTP PORT')
            server(port)            
        elsif command == 'help'
            general_help()
        elsif command == 'wdns'
            webDns()
        elsif command == 'clear'
            system('clear')
        elsif command == 'exit'
            exit()
        elsif command == 'reset'
            system('reset')                    
        elsif command == 'ngrok'
            port = props('SET HTTP PORT')
            ngrok(port)                                 
        elsif command == 'metasploit'
            metasploit()                                
        elsif command == 'install'
            install()                                     
        elsif command == 'ssh'
            user = props('SET USERNAME')
            ip = props('SET TARGET IP')
            ssh(user, ip)                               
        elsif command == 'telnet'
            ip = props('SET TARGET IP')
            telnet(ip)                                  
        elsif command == 'dumptcp'
            pl = props('SET INTERFACE')
            ip = props('SET OUTPUT FILE NAME')
            dumptcp(pl, file)                           
        elsif command == 'mac'
            pl = props('SET INTERFACE')
            mac(pl)                                     
        elsif command == 'footprint'
            opt = props('SET ADICIONAL OPTIONS')
            host = props('SET HOST')
            proxy = props('USE PROXY [yes|no]')
            footprint(opt = '', host, proxy = nil)      
        elsif command == 'scanner'
            pl = props('SET INTERFACE')
            proxy = props('USE PROXY [yes|no]')
            scanner(pl, proxy = nil)
        elsif command == 'nmap'
            host = props('SET HOST')
            opt = props('USE ADICIONAL OPTIONS')
            proxy = props('USE PROXY [yes|no]')
            nmap(host, opt, proxy = nil)
        else
            prPurple("LETK command not found, ti vira com o shell\n")
            system(command)
        end                   
        
    end
    
end # END SHELL MODULE
