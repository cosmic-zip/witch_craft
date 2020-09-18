#-------------------------------------------------------------
#
#                     Linux Evil Toolkit
# 
#                          By v0id
#
#                        2019 - 2020
#
#-------------------------------------------------------------

module Visual
     
    def prRed(string);            puts "\033[91m #{string}\033[00m"; end
    def prGreen(string);          puts "\033[92m #{string}\033[00m"; end
    def prYellow(string);         puts "\033[93m #{string}\033[00m"; end
    def prLightPurple(string);    puts "\033[94m #{string}\033[00m"; end
    def prPurple(string);         puts "\033[95m #{string}\033[00m"; end
    def prCyan(string);           puts "\033[96m #{string}\033[00m"; end
    def prLightGray(string);      puts "\033[97m #{string}\033[00m"; end
    def prBlack(string);          puts "\033[98m #{string}\033[00m"; end

    def banner()
        puts "\n"
        prRed(' _      _                    ______     _ _   _______          _ _    _ _   ')
        prRed('| |    (_)                  |  ____|   (_) | |__   __|        | | |  (_) |  ')
        prRed('| |     _ _ __  _   ___  __ | |____   ___| |    | | ___   ___ | | | ___| |_ ')
        prRed("| |    | | '_ \\| | | \\ \\/ / |  __\\ \\ / / | |    | |/ _ \\ / _ \\| | |/ / | __|")
        prRed("| |____| | | | | |_| |>  <  | |___\  V /| | |    | | (_) | (_) | |   <| | |_ ")
        prRed("|______|_|_| |_|\\__,_/_/\\_\\ |______\\_/ |_|_|    |_|\\___/ \\___/|_|_|\\_\\_|\\__|")
        puts "\n\n"
        prLightPurple("help for more commands")

    end

    # def init_menu()

    # end        
    
    # def screen_options()

    #   prCyan("\nRaw commands\n"  )
    #   prCyan('[A0] ping test       [08] composer        [19] whois           [29] wireshark-qt')
    #   prCyan('[B0] enable proxy    [09] npm             [20] nslookup        [30] hping3')
    #   prCyan('[00] htop            [10] findtools       [21] nikto           [31] iptraf')
    #   prCyan('[01] ngrok           [12] tor             [22] nbscan          [32] foremost')
    #   prCyan('[02] ruby            [13] squid           [23] ncat            [33] dc3dd')
    #   prCyan('[03] php             [14] proxychains     [24] netcat          [34] macchanger')
    #   prCyan('[04] gcc             [15] nmap            [25] httprint        [35] john')
    #   prCyan('[05] gdb             [16] ncrack          [26] netstat         [36] hydra')
    #   prCyan('[06] hexedit         [17] dnsenum         [27] traceroute      [37] sqlmap')
    #   prCyan('[07] nodejs          [18] dnsmap          [28] tcpdump         '); puts "\n\n"
      
    # end

    def webDns()

      prRed("Netcraft      – endereços fora do Brasil      http://news.netcraft.com/")
      prRed("Domaintools   - whois, lookup, IP, etc.       http://www.domaintools.com/")
      prRed("Registro BR   – endereços no Brasil           https://registro.br/cgi-bin/whois/")
      prRed("Arin          – endereços fora do Brasil      https://www.arin.net/")
      prRed("Apnic         - endereços Ásia e Pacífico     http://www.apnic.net/apnic-info/search")
      prRed("Whois         – endereços fora do Brasil      http://new.whois.net/")
      prRed("Ripe          – endereços europeus            http://www.ripe.net/"); puts "\n\n"
      
    end

    def general_help()

      puts "\n\n"
      prYellow("Outros: clear, reset, logout, exit, rawCommands")
      puts "\n\n"
      prYellow("doc               Gera um arquivo de documentação")
      prYellow("wdns              Mostra servidores web para pesquida de dns")
      prYellow("help              Exibe ajuda")
      prYellow("server            Criar um servidor em localhost")
      prYellow("ngrok             Criar um servidor web externo")
      prYellow("metasploit        Instala o metasploit")
      prYellow("install           Instala todas as ferramentas no ArchLinux")
      prYellow("ssh               Faz uso de ssh")
      prYellow("telnet            Faz uso de telnet")
      prYellow("dumptcp           Faz dump de pacote da rede atual")
      prYellow("mac               Altera o endereço mac")
      prYellow("footprint         Faz o ping scan")
      prYellow("scanner           Faz Varredura de pportas, whois, ")
      prYellow("                  encontra flahas,pequisa dns, e brute")
      prYellow("                  force de dns")
      prYellow("nmap              Faz uso de nmap com alias TCP SYN, UDP, ")
      prYellow("                  e bypass de firewall"); puts "\n\n"
    
    end
    
end