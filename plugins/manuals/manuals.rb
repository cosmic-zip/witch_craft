module Manuals

	def help_simple

		puts "\n\n"
		prWhite "generate --rg 					|	Generate fake rg"
		prWhite "generate --cpf 				|	Generate fake cpf"
		prWhite "generate --person 				|	Generate parson fake data"
		prWhite "generate --fakedump 			|	Generate fake database"
		prWhite "visuall --banner 				|	Show zynix banner"
		prWhite "visuall --web-dns 				|	Show web dns lookup"
		prWhite "visuall --linux-files 			|	Show useful linux files"
		prWhite "visuall --linux-folders 		|	Show useful linux folders"
		prWhite "visuall --win-files 			|	Show useful windows files"
		prWhite "visuall --linux-utilites 		|	Show linux command line utilites"
		prWhite "visuall --tor-search-link 		|	Show tor search engine links"
		prWhite "visuall --tor-alt 				|	Show tor alternatives"
		prWhite "visuall --help 				|	Show genereal help" 
		prWhite "km --init 						|	Initialize framework with data"
		prWhite "km --install 					|	Install framework"
		prWhite "km --dlookup 					|	Search whois, emails, phone numbers and banner grep"
		prWhite "km --dns-scanner 				|	Web dns scanner"
		prWhite "km --dir-scanner 				|	Web domain folder scanner"
		prWhite "km --cover 					|	Clear logs and files localy"
		prWhite "km --simple-map 				|	Automatic nmap"
		prWhite "km --maclookup 				|	MAC address lookup"
		prWhite "km --extract 					|	Extract files"
		prWhite "km --compress 					|	Compress files"
		prWhite "km --blue-attck 				|	Bluetooth attack module"
		puts "\n\n"

	end

	def help_simple_map

		puts "\n"
        prRed '**alone**'
        puts "\n"
        prGreen '    "-sL" --> "List Scan - simply list targets to scan"'
        prGreen '    "-sP" --> "Ping Scan - go no further than determining if host is online"'
        puts "\n"
        prRed '**default**'
        puts "\n"
        prGreen '    "-sS -sV" --> "TCP SYN"'
        prGreen '    "-sU -sV" --> "UDP Scan"'
        puts "\n"
        prRed '**icmp_echo**'
        puts "\n"
        prGreen '    "-sS -sV -PE" --> "TCP SYN + ICMP echo discovery probes"'
        prGreen '    "-sU -sV -PE" --> "UDP Scan + ICMP echo discovery probes"'
        prGreen '    "-sA -sV -PE" --> "ACK + ICMP echo discovery probes"'
        puts "\n"
        prRed '**port_list**'
        puts "\n"
        prGreen '    "-sS" --> "TCP SYN + [portlist]: TCP SYN discovery probes to given ports"'
        prGreen '    "-sA" --> "ACK + [portlist]: TCP ACK discovery probes to given ports"'
        prGreen '    "-sU" --> "UDP Scan + [portlist]: TCP UDP discovery probes to given ports"'
        puts "\n"
        prRed '**special**'
        puts "\n"
        prGreen '    "-sT -sV" --> "Connect()"'
        prGreen '    "-sW -sV" --> "Window"'
        prGreen '    "-sM -sV" --> "Maimon scans"'
        prGreen '    "-sN -sV" --> "TCP Null"'
        prGreen '    "-sF -sV" --> "FIN"'
        prGreen '    "-sX -sV" --> "Xmas scans"'

    end

end