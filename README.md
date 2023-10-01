![banner](docs/images/banner.png)

# Maid Runner

Maid Runner is a versatile task automation software designed to serve as the foundation for various cyber security modules. It provides capabilities for tasks such as forensic research, OSINT (Open Source Intelligence), scanning, backup and copying, intrusion testing of applications and APIs, and more.

##  Why is there only one commit?

*September 12, 2023, The codebase has undergone substantial transformations over time, making it distinctly different from its earlier iterations. As a result, all previous versions have been decisively discarded, giving rise to this new and optimized rendition.*

## HELP

@general 

	help unix_network           Help page for unix network       
	help unix_sys_info          Help page for unix system information   
	help unix_utility           Help page for unix common commands      
	help unix_command           Help page for unix useful unix commands      
	help unix_misc              Help page for unix less common unix command         
	help unix_files             Help page for unix important files and config files       
	help unix_folders           Help page for unix file system structure      
	help windows_files          Help page for windows important files and config files     
	help windows_reg            Help page for windows common registry entry points       
	help windows_cmd            Help page for windows CMD commands       
	help windows_powershell     Help page for windows powershell commands
	help maid                   A cute maid ASCII art (⸝⸝⸝╸w╺⸝⸝⸝)

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

@bcurl 

	--curl_bind					Binds for curl command
		--method				Set http method
				get
				post
				put
				patch
				delete
				head
				options
				connect
				trace
		--auth_type				Set the authentication type
				basic
				bearer
				api_key
				ntlm	
		--auth_token			Set the authentication token or user:password
		--url					Set target url
		--ctn_type				Set content type
				json
				xml
				form_data
				text
				multi_part_form_data
		--data					Set data (if needed) json/formData/xml/text/multi_part_form_data


	--header					Show http header
	--status_code				Show status code from a GET request  

### License

**This project is licensed under the GNU General Public License v3.0.**