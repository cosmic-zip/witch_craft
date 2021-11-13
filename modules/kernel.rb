#-------------------------------------------------------------
#
#
#                     Linux Evil Toolkit
#                        By TH3V0ID
#
#
#-------------------------------------------------------------
# Copyright (c) 2020 - 2022 Th3Void <https://github.com/th3void>
#
# This program is free software; you can redistribute it and/or
# modify it under the terms of the GNU General Public
# License as published by the Free Software Foundation; either
# version 3 of the License, or (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
# General Public License for more details.
#
# You should have received a copy of the GNU General Public
# License along with this program; if not, write to the
# Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
# Boston, MA 02110-1301 USA

module Kernel

    # Get time
    
    def get_time 
        Time.now.strftime("%d.%m.%Y_%H::%M")
    end

    #Session
    def session(mtd)
        # create
        # read
        # update
        # delete

    end

    # init variable
    def init(props = [])

        if $state == nil
            $ip = set('Set taget ip:')
            $target = set('Set taget dns [without http/s]:')
            doc = set('Enable shell documentation? [yes|no]:')
            doc == 'yes' ? $doc = true : nil        
        end
        
    end

    # Get value and show text
    def set(msg = nil)

        msg == nil ? nil : print(msg + ' ')  
        var = gets.chomp.to_s

    end

	# Alias for system
	def sys(command)

        shell_code = "echo '#{ Time.now.strftime("%d-%m-%Y_%H-%M")} :: #{command}' >> shelldoc.txt"
        $doc == true ? system(shell_code) : nil 
		system(command)

	end

	# Extract files
	def extract

        file_name = set('Set file name:')
        msg = "[ERROR]: File extension fail"
        File.extname(file_name) == ".tar" ? sys("tar xf #{file_name}.tar") : prCyan(msg)
        File.extname(file_name) == ".gz" ? sys("tar xzf #{file_name}.tar.gz") : prCyan(msg)
        File.extname(file_name) == ".bz2" ? sys("tar xjf #{file_name}.tar.bz2") : prCyan(msg)
        File.extname(file_name) == ".zip" ? sys("unzip #{file_name}") : prCyan(msg)

    end
    
    # def note
    def note(text)
        shell_code = "echo #{text} >> $HOME/outputs/notes_#{time}.dnt"
        sys(shell_code)
    end

    # Compress files
    def compress

        input_file_name = set('Set input file:')
        output_file_name = set('Set output file:')
        ext = set('Set ext:')
        msg = "[ERROR]: wrong parameters"
        prRed($line)
        ext == "tar" ? sys("tar cf #{output_file_name}.tar #{input_file_name}"): prCyan(msg)
        ext == "gz" ? sys("tar czf #{output_file_name}.tar.gz #{input_file_name}"): prCyan(msg)
        ext == "bz2" ? sys("tar cjf #{output_file_name}.tar.bz2 #{input_file_name}"): prCyan(msg)
        ext == "zip" ? sys("gzip #{output_file_name}"): prCyan(msg)

    end

    # Cover
    def cover

        i = set('Cover? [y/n]:')
    	if i != nil
    		res = set('Remove history? [y/n]:')
                if res == 'y'
                    prCyan "[+] Clear bash_history"
                    sys('srm -rfD -/.bash_history')
                    prCyan "[+] Clear history"
                    sys('history -c')
                end
    		res = set('Remove or disable permanently history file? [r/d]:')
                if res == 'r'
                    prCyan "[+] Disable history"
                    sys('export HISTFILESIZE=O && export HISTSIZE=O && unset HISTFILE')
                else
                    prCyan "[+] Perrnanent send all bash history commands to /dev/null"
                    sys('ln /dev/null ~/.bash_history -sf')
                end
    		res = set('Clear /var/log? [y/n]:')
                if res == 'y'
                    prCyan "[+] Clear auth log"
                    sys('srm -rfD /var/log/')
                    print 'done'
                end
       	else
			sys('history -c')
       		sys('srm -rfD -/.bash_history')
			sys('export HISTFILESIZE=O && export HISTSIZE=O && unset HISTFILE')
			puts '[+] Bash history cleaned'
		end

        puts "\n\n"

    end 
    
    # Web vul scanner
    def search

        prRed($line)
        prYellow "[+] :: Get robots.txt"; sys("wget https://www.#{$target}/robots.txt")
        prYellow "[+] :: WHOIS"; sys("whois -a #{$target}")
        prYellow "[+] :: Email Enumeration"; sys("theharvester -d #{$target} -l 500 -b all")
        prYellow "[+] :: HTTP Banner grep at ~/outputs"; sys("ncat -v #{$ip} 80 >> ~/banner_port_80_#{$ip}_.html")
        prYellow "[+] :: HTTPS Banner grep"; sys("openssl s_client -quiet -connect #{$target}:443 >> ~/banner_port_443_#{$ip}_.html")
        prYellow "[+] :: Nikto scanner"; sys("nikto -h #{$ip}:443 -ssl")

    end

    # Web dns scanner
    def dns_scanner

        prYellow "[+] :: DNS Enumeration:"
        sys("dnsenum --enum #{$target} ./wordlist/dns.txt") 
        reg_dns = ['A', 'AAAA', 'CNAME', 'MX', 'NS', 'PTR', 'SOA']
        for reg in reg_dns
            prYellow "[+] :: Search for #{reg} in #{$target}:"
            sys("host -t #{reg} #{$target}")
        end

    end

    # Web directory scanner
    def dir_scanner()

        prCyan $line
        prCyan "======================== HOTKEYS ========================"
        prCyan " 'n' -> Go to next directory."
        prCyan " 'q' -> Stop scan. (Saving state for resume)"
        prCyan " 'r' -> Remaining scan stats.\n\n#{$line}\n"

        print('Set custom config? [yes|no]: '); opt = gets.chomp.to_s
        if opt == 'yes'
            print('Show terminal dump? [yes|no]: '); silent = gets.chomp.to_s
            silent == 'yes' ? silent = '-S' : silent = ''
            print('Write output in file? [yes|no]: '); output = gets.chomp.to_s
            output == 'yes' ? output = '-o ./dir_dump.txt' : output = ''
            print('Search for extension? [.type|no]: '); ext = gets.chomp.to_s
            extension != 'no' ?  extension = "-x #{ext}" : extension = ''
            print('Time delay mode "1: normal |2: slow |3: paranoid" [1|2|3]: ')
            delay = gets.chomp.to_i
            case delay
            when 1
                delay = '0'
            when 2
                delay = '500' 
            when 3
                delay = '2000'
            else
                delay = '100'
            end
            flags = ' -i -w '+silent+' '+output+' '+extension+' -z '+delay+'' 
            sys("dirb #{$target} ./wordlist/directory_list.txt #{flags}")
        else  
            sys("dirb -i -w -S #{$target} ./wordlist/directory_list.txt")
        end 

    end 

    # Function for simple_map only
    def exec(list, timing, ipv6)

        mac_vendor = [ "006017", "006018", "006019", "00601A", "00601B", "00601C", "00601D", "00601E", "00601F", "006020", "006021", "006022", "006023", "006024", "006025", "006026" ]
        for opt in list
            prYellow "[EXEC]: #{opt[1]}"
            cmd = sys "nmap #{opt[0]} #{$target} -O -T#{timing} --spoof-mac #{mac_vendor[rand(0..15)]} --data-length 256 --max-retries 10 --mtu 1024 --host-timeout 30 --ttl 60 #{ipv6 != nil ? '-6' : ''}"
            cmd == true ? prGreen("[SYS_COMMAND]: Done") : prRed("[SYS_OPTION_ERROR]: Bad option or invalid timing::#{cmd}\n")
        end

    end
  
    # Automatic nmap scans
    def simple_map()

        ipv6 = set('Use ipv6? [y/n]:')
        props = set('Set optional flag to nmap: [enter for null]:')
        timing = set('Set nmap velocity: [default 4]:')

        alone = [
            ["-sL", "List Scan - simply list targets to scan"],
            ["-sP", "Ping Scan - go no further than determining if host is online"],
        ]

        default = [
            ["-sS -sV", "TCP SYN"],
            ["-sU -sV", "UDP Scan"],
        ]

        icmp_echo = [
            ["-sS -sV -PE", "TCP SYN + ICMP echo discovery probes"],
            ["-sU -sV -PE", "UDP Scan + ICMP echo discovery probes"],
            ["-sA -sV -PE", "ACK + ICMP echo discovery probes"],
        ]
            
        port_list = [
            ["-sS", "TCP SYN + [portlist]: TCP SYN discovery probes to given ports"],
            ["-sA", "ACK + [portlist]: TCP ACK discovery probes to given ports"],
            ["-sU", "UDP Scan + [portlist]: TCP UDP discovery probes to given ports"],
        ]
            
        special = [
            ["-sT -sV", "Connect()"],
            ["-sW -sV", "Window"],
            ["-sM -sV", "Maimon scans"],
            ["-sN -sV", "TCP Null"],
            ["-sF -sV", "FIN"],
            ["-sX -sV", "Xmas scans"]
        ]

        case props
        when 'alone'
            exec(alone, timing, ipv6)
        when 'default'
            exec(default, timing, ipv6)
        when 'icmp_echo'
            exec(icmp_echo, timing, ipv6)
        when 'port_list'
            exec(port_list, timing, ipv6)
        when 'special'
            exec(special, timing, ipv6)
        else 
            prRed "[SYS_OPTION_ERROR]: #{props}::Invalid option"
        end 
    end

    def maclookup

        vendor = set('Set vendor mac like [A8:74:84]:')
        table = CSV.parse(File.read("database/macaddr.csv"), headers: true)
        i = 0
        for x in table 
            if table[i][0] = vendor
                prGreen table[i]
                break
            elsif
                i = i+1 #yep
            end
        end
    
    end

    def bluetooth

        target = set('Set target mac address [type 0 for list]: ')
        if number = "0"
            cmd = sys "bt-device -l"
            target = set('Set number of attempts [type 0 for list]: ')
        end
        number = set('Set number of attempts: ')

        x = 0
        while x < number 
            prRed "-------------- Sterting --------------"
            command = "bt-device -c #{target}"
            if command == true
                prRed "[attempt number #{x}]: ... Done"
            end
            x = x + 1
        end

    end

end