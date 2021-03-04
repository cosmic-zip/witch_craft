# Viual 


module Visual


    def prRed(string);            puts "\033[91m #{string}\033[00m"; end
    def prGreen(string);          puts "\033[92m #{string}\033[00m"; end
    def prYellow(string);         puts "\033[93m #{string}\033[00m"; end
    def prLightPurple(string);    puts "\033[94m #{string}\033[00m"; end
    def prPurple(string);         puts "\033[95m #{string}\033[00m"; end
    def prCyan(string);           puts "\033[96m #{string}\033[00m"; end
    def prLightGray(string);      puts "\033[97m #{string}\033[00m"; end
    def prBlack(string);          puts "\033[98m #{string}\033[00m"; end

	def banner
		prRed '▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄'
        prRed '██░▄▄▄░██░███░██░▀██░█▄░▄█▄▀█▀▄████░▄▄▄██░██░██░▄▄▄░█▄░▄██░▄▄▄░██░▀██░█'
        prRed '██▀▀▀▄▄██▄▀▀▀▄██░█░█░██░████░██████░▄▄███░██░██▄▄▄▀▀██░███░███░██░█░█░█'
        prRed '██░▀▀▀░████░████░██▄░█▀░▀█▀▄█▄▀████░█████▄▀▀▄██░▀▀▀░█▀░▀██░▀▀▀░██░██▄░█'
        prRed '▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀'
		puts  "\n\n by th3void \n\n"
	end

	def web_dns
  
    	prRed("Netcraft      – endereços fora do Brasil      http://news.netcraft.com/")
        prRed("Domaintools   - whois, lookup, IP, etc.       http://www.domaintools.com/")
        prRed("Registro BR   – endereços no Brasil           https://registro.br/cgi-bin/whois/")
        prRed("Arin          – endereços fora do Brasil      https://www.arin.net/")
        prRed("Apnic         - endereços Ásia e Pacífico     http://www.apnic.net/apnic-info/search")
        prRed("Whois         – endereços fora do Brasil      http://new.whois.net/")
        prRed("Ripe          – endereços europeus            http://www.ripe.net/")
        puts "\n\n"
      
    end

    def linux_files

        prRed($line)
        prCyan("Local users' hashes              /etc/shadow")
        prCyan("Local users                      /etc/passwd")
        prCyan("Local groups                     /etc/group" )
        prCyan("Startup services                 /etc/rc.d")
        prCyan("Service                          /etc/init.d")
        prCyan("Known hostnames and IPs          /etc/hosts" )
        prCyan("Full hostnarne with domain       /etc/HOSTNAl1E" )
        prCyan("Network configuration            /etc/network/interfaces")
        prCyan("System environment variables     /etc/profile" )
        prCyan("Ubuntu sources list              /etc/apt/sources.list")
        prCyan("Narneserver configuration        /etc/resolv.conf" )
        prCyan("Bash history (also /root/)       /horne/ user /.bash historj")
        prCyan("Vendor-t1AC lookup               /usr/share/wireshark/rnanuf")
        prCyan("SSH keystore                     -/.ssh/")
        prCyan("System log files (most Linux)    /var/log" )
        prCyan("System log files (Unix)          /var/adrn")
        prCyan("List cron files                  /var/spool/cron")
        prCyan("Apache connection log            /var/log/apache/access.log")
        prCyan("Static file system info          /etc/fstab")
        puts "\n\n"

    end

    def linux_folders

        prRed($line)
        prGreen("User binaries                      /bin")
        prGreen("Boot-up related files              /boot")
        prGreen("Interface for system devices       /dev")
        prGreen("Sjstern configuration files        /etc")
        prGreen("Base directory for user files      /horne")
        prGreen("Critical software libraries        /lib")
        prGreen("Third party software               /opt")
        prGreen("Sjstern and running programs       /proc")
        prGreen("Home directory of root user        /root")
        prGreen("System administrator binaries      /sbin")
        prGreen("Temporary files                    /trnp")
        prGreen("Less critical files                /usr")
        prGreen("Variable Sjstern files             /var")
        puts "\n\n"

    end
 
    def windows_files

        prRed($line)
        prPurple 'WINDOWS FILES'
        prPurple 'Typically C:\Windows        %SYSTEMROOT%'
        prPurple 'DNS entries                 %SYSTEMROOT%\\System32\\drivers\\etc\\hosts'
        prPurple 'Network settings            %SYSTEMROOT%\\System32\\drivers\\etc\\networks'
        prPurple 'User & password hashes      %SYSTEMROOT%\\system32\\config\\SAM'
        prPurple 'Backup copy of SAM          %SYSTEMROOT%\\repair\\SAM'
        prPurple 'Backup copy of SAM          %SYSTEMROOT%\\System32\\config\\RegBack\\SAM'
        prPurple 'Application Log             %WINDIR%\\system32\\config\\AppEvent.Evt'
        prPurple 'Security Log                %WINDIR%\\system32\\config\\SecEvent.Evt'
        prPurple 'Startup Location            %ALLUSERSPROFILE%\Start Menu\Programs\Startup\\'
        prPurple 'Startup Location            %USERPROFILE%\\Start Menu\\Programs\\Startup\\'
        prPurple 'Prefetch dir (EXE logs)     %SYSTEMROOT%\\Prefetch'
        puts "\n\n"

    end

    def linux_util

        prRed($line)
        prYellow("Grab url                      wget http:// url -0 url.txt -o /dev/null")
        prYellow("Remote Desktop to ip          desktop ip")
        prYellow("Put file                      cp /tmp/file user@x.x.x.x:/tmp/file")
        prYellow("Get file                      cp user@ remoteip :/tmp/file /tmp/file")
        prYellow("Add user                      seradd -m user")
        prYellow("Change user password          asswd user")
        prYellow("Remove user                   muser unarne")
        prYellow("Record shell : Ctrl-D stops   cript -a outfile")
        prYellow("Find related command          propos subject")
        prYellow("View users command history    istory")
        prYellow("Executes line # in history    num")
        puts "\n\n"

    end

    def tor_search 

        prRed 'AHMIA http://msydqstlz2kzerdg.onion '
        prRed 'tor2web https://giyvshdnojeivkom.onion.ly/'
        prRed 'Tor Search Engine https://www.torsearchengine.net/'
        puts "\n\n"
    
    end

    def tor_alt 

        prYellow 'Tor Browser – The stated goals of the Tor Browser are to protect the privacy of the users and to defend the users against the network surveillance and traffic analysis, While another latest goals have been added to the previous one which is to bring the wider access to the anonymous web browsing through increasing user-friendliness. This is not something you see as a major goal for the darknet networks other than Tor Browser and OpenBazaar. Tor gets lots of media attention for terrorism, but this is not necessarily a valid perception whatsoever.'
        prYellow 'Freenet – Freenet is a peer-to-peer platform (P2P) for censorship-resistant communication and publishing, and focuses heavily on the promotion of freedom of speech over censorship, copyright, and as well as takedowns. Freenet is driven by the ideology rather than financial motives, and it has a number of social platforms and chat systems. Its users tend to lean toward a small or non-intrusive government, and it is most popular with crypto activists. On this darknet, we have not found anything for sale as per the research, most likely because its users give away useful information. It is home to the hacked documents, including leaked and confidential TTIP negotiation documents; internal Diebold emails about how their voting machines are flawed; pre-written Spectre exploit code and guide as well; and data or document dumps that are public. Till date, it has resisted any external takedown attempts.'
        prYellow 'I2P Project – It is a popular darknet framework for multiple self-proclaimed factions of Anonymous and other self-described hacktivists. In fact, its stated mission is that it is intended to protect communication from darknet surveillance and monitoring by third parties such as the ISPs and is generally used by many people who care about their privacy: activists, oppressed people, journalists, and whistleblowers, along with the average person. The content is primarily in Russian, Chinese, and English, and includes an archive of past classes for the hacktivists that cover hacking and various other techniques. Some of the talks still have the names of the presenters on them, which includes a course on advanced web application hacking given by a researcher from a top technology firm. This darknet framework also includes a chat portal, access to a DDoS tool, and a web application vulnerability scanner.'
        prYellow 'OpenBazaar – OpenBazaar is one of the latest dark web frameworks, and its purpose is to offer a feeless, peer-to-peer (P2P) marketplace that leverages the Cryptocurrencies for transactions. There are some illicit offerings that include drugs, hacking tools, books and services, stolen media streaming accounts, and bulk social media accounts. Nevertheless, the majority is more mundane, like original artwork, jewelry, books, clothing, and health supplements. There is a wealth of interesting information based on geographies and language use that can help us contextualize these frameworks and their underpinnings and offerings.'
        puts "\n\n"

    end

end