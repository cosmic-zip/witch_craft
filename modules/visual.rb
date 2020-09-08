#! /bin/ruby
#-------------------------------------------------------------
#
#                     Linux Evil Toolkit
# 
#                       By Xx_VOID_xX
#
#-------------------------------------------------------------

module Visual

    def banner()

        puts ' _      _                    ______     _ _   _______          _ _    _ _   '
        puts '| |    (_)                  |  ____|   (_) | |__   __|        | | |  (_) |  '
        puts '| |     _ _ __  _   ___  __ | |____   ___| |    | | ___   ___ | | | ___| |_ '
        puts "| |    | | '_ \\| | | \\ \\/ / |  __\\ \\ / / | |    | |/ _ \\ / _ \\| | |/ / | __|"
        puts "| |____| | | | | |_| |>  <  | |___\ V /| | |    | | (_) | (_) | |   <| | |_ "
        puts "|______|_|_| |_|\\__,_/_/\\_\\ |______\\_/ |_|_|    |_|\\___/ \\___/|_|_|\\_\\_|\\__|"
    
    end

    def init_menu()
        
    
    def screen_options()

      puts "\nRaw commands\n"  
      puts '[A0] ping test       [08] composer        [19] whois           [29] wireshark-qt'
      puts '[B0] enable proxy    [09] npm             [20] nslookup        [30] hping3'
      puts '[00] htop            [10] findtools       [21] nikto           [31] iptraf'
      puts '[01] ngrok           [12] tor             [22] nbscan          [32] foremost'
      puts '[02] ruby            [13] squid           [23] ncat            [33] dc3dd'
      puts '[03] php             [14] proxychains     [24] netcat          [34] macchanger'
      puts '[04] gcc             [15] nmap            [25] httprint        [35] john'
      puts '[05] gdb             [16] ncrack          [26] netstat         [36] hydra'
      puts '[06] hexedit         [17] dnsenum         [27] traceroute      [37] sqlmap'
      puts '[07] nodejs          [18] dnsmap          [28] tcpdump         '
      
    end

    def webDns()

      puts "Netcraft      – endereços fora do Brasil      http://news.netcraft.com/"
      puts "Domaintools   - whois, lookup, IP, etc.       http://www.domaintools.com/"
      puts "Registro BR   – endereços no Brasil           https://registro.br/cgi-bin/whois/"
      puts "Arin          – endereços fora do Brasil      https://www.arin.net/"
      puts "Apnic         - endereços Ásia e Pacífico     http://www.apnic.net/apnic-info/search"
      puts "Whois         – endereços fora do Brasil      http://new.whois.net/"
      puts "Ripe          – endereços europeus            http://www.ripe.net/"
      
    end

    def general_help()

      puts "server(port)                                Criar um servidor em localhost"
      puts "ngrok(port)                                 Criar um servidor web externo"
      puts "metasploit()                                Instala o metasploit"
      puts "install                                     Instala todas as ferramentas no Arch"
      puts "                                            Linux"
      puts "ssh(user, ip)                               Faz uso de ssh"
      puts "telnet(ip)                                  Faz uso de telnet"
      puts "dumptcp(pl, file)                           Faz dump de pacote da rede atual"
      puts "mac(pl)                                     Altera o endereço mac"
      puts "footprint(opt = '', host, proxy = nil)      Faz o ping scan"
      puts "scanner(hst, proxy = nil)                   Faz Varredura de pportas, whois, "
      puts "                                            encontra flahas,pequisa dns, e brute"
      puts "                                            force de dns"
      puts "nmap(host, opt, proxy = nil)                Faz uso de nmap com alias TCP SYN, UDP, "
      puts "                                            e bypass de firewall"
    
    end
    
end