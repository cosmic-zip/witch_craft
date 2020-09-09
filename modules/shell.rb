#! /bin/ruby
#-------------------------------------------------------------
#
#                     Linux Evil Toolkit
# 
#                       By Xx_VOID_xX
#
#-------------------------------------------------------------

module Shell

    def init()
        
    end

    def ps1()
        # read shell.conf
        ps1 = '[LETK]@[v0id]: '
        print(ps1)
        
    end

    def props(msg)
        
        print("#{msg}: ")
        props = gets.chomp()
    
    end

    def interpreter(command)

        if command == 'server'
            port = props('SET HTTP PORT')
            server(port)                                
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
            pl = props('SET HOST')
            pl = props('USE PROXY [yes|no]')
            footprint(opt = '', host, proxy = nil)      
        elsif command == 'scanner'
            pl = props('SET INTERFACE')
            pl = props('USE PROXY [yes|no]')
            scanner(hst, proxy = nil)
        elsif command == 'nmap'
            pl = props('SET HOST')
            pl = props('USE ADICIONAL OPTIONS')
            pl = props('USE PROXY [yes|no]')
            nmap(host, opt, proxy = nil)
        else
            puts "LETK command not found, ti vira com o shell\n"
            system(command)
        end                   
        
    end
    
end # END SHELL MODULE
