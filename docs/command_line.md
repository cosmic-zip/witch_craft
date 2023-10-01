@core
	--remove_metadata           Remove metadata from a picture
		
		--debug [optional]
			true
			none
		--path                  Image file path
			./file/image
	--web_downloader            Download an web page an all relatable files 
		
		--debug [optional]      Turn debug on, was an optional flag isn't needed
			true
			none
		--url                   Target web page URL with www
			www.example.com
	
@lookup
	
    --mac_address key value     Lookup mac vendor based on first 3 pairs
		
		--debug [optional]      Turn debug on, was an optional flag isn't needed
			true
			none
		--mac                   MAC string with 00:00 or 00:00:00
			00:00:00
			01:23
		--path                  MAC lookup file (see file/general/macaddr_lockup.ascii)
			./file/mac_list

	--lookup_reverse_engineering	Lookup basic reverse engineering
		--debug [optional]      Turn debug on, was an optional flag isn't needed
			true
			none
		--sample                File to be analyzed
			./file/sample
		--type                  Type of analysis
			s                   	search for string
			h                   	search for hexadecimals
			b                   	search for binary
			r                   	todo
			l                   	search for linked library
	
@scanner 
	
    --web_scanner key value     Scanning domain and ip's 
		--target                Set target ip or dns
			172.16.0.1
			example.com

        --type                  Select and scanning type:
            ip                      basic ping
            whois                   basic whois
            routes                  basic traceroute
            dns                     basic dns enumeration
            nmap_tcp                basic nmap TCP scanner
            nmap_udp                basic nmap UDP scanner
            sub_domains             DNS sub domains scanner
            sub_directory           Web page sub directory scanner
            build_with              Scan common frameworks on a page
            c_api_url               Scan common api urls
            c_web_url               Scan common web page urls

        --path [optional]       File path need by:
                                    sub_domains
                                    sub_directory
                                    build_with
                                    c_api_url
                                    c_web_url
            ./file/generic_list File path

        --debug [optional]      Turn debug on, was an optional flag isn't needed
            true
		    none