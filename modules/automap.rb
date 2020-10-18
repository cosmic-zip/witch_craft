#-------------------------------------------------------------
#
#                     Linux Evil Toolkit
# 
#                          By v0id
#
#                        2019 - 2020
#
#------------------------------------------------------------

module Automap

    $HOS = [
        ["-P0", "Treat all hosts as online -- skip host discovery"],
        ["-PS", "[portlist]: TCP SYN discovery probes to given ports"],
        ["-PA", "[portlist]: TCP ACK discovery probes to given ports"],
        ["-PU", "[portlist]: TCP UDP discovery probes to given ports"],
        ["-PE", "ICMP echo discovery probes"],
        ["-sV", "Probe open ports to determine service/version info"],
        ["-PP", "timestamp request discovery probes"],
        ["-PM", "netmask request discovery probes"]
    ]

    $ALO = [
        ["-sL", "List Scan - simply list targets to scan"],
        ["-sP", "Ping Scan - go no further than determining if host is online"],
    ]
  
    $SCA = [
        ["-sS", "TCP SYN"],
        ["-sT", "Connect()"],
        ["-sA", "ACK"],
        ["-sW", "Window"],
        ["-sM", "Maimon scans"],
        ["-sN", "TCP Null"],
        ["-sF", "FIN"],
        ["-sX", "Xmas scans"]
    ]

    $VEN = [
        ["006017", "Tokimec"],
        ["006018", "Stellar ONE"],
        ["006019", "Roche Diagnostics"],
        ["00601A", "Keithley Instruments"],
        ["00601B", "Mesa Electronics"],
        ["00601C", "Telxon"],
        ["00601D", "Lucent Technologies"],
        ["00601E", "Softlab"],
        ["00601F", "Stallion Technologies"],
        ["006020", "Pivotal Networking"],
        ["006021", "DSC"],
        ["006022", "Vicom Systems"],
        ["006023", "Pericom Semiconductor"],
        ["006024", "Gradient Technologies"],
        ["006025", "Active Imaging PLC"],
        ["006026", "Viking Modular Solutions"]
    ]

    def assembly(unity)
        # Define hidden flag
        mac = $VEN[rand(0..14)]
        return "nmap #{unity} -O #{$target} --spoof-mac #{mac[0]} --data-length #{rand(2..256)} --max-retries 10 --mtu 1024 --host-timeout 30 --ttl 60 -f #{rand(1..6)}"
    end

    def exec(method, list)
        for ht_method in list
            puts "[AUTO EXEC]: #{ht_method[1]}\n"
            command = sys(assembly("#{method} #{ht_method[0]}"))
            if command == true
                print("#{$line}\n[SUCCESSFUL]\n")
            else
                print("#{$line}\n[ERROR]: External command fail: Keep calm, monkeys working\n")
            end
        end
    end

    def less_boring()
        
        # Why?
        method = $SCA
        ht_method = $HOS
        # List all option
        for var in method
            puts "flag: #{var[0]} | value: #{var[1]}"
        end
        # Get value from monkey user
        print("#{$line}\nSet flarg option: [ex: -sS]: ")
        flag = gets.chomp.to_s
        # Test flag option
        case flag
        when "-sS"
            exec("-sS", ht_method)
        when "-sT"
            exec("-sT", ht_method)
        when "-sA"
            exec("-sA", ht_method)
        when "-sW"
            exec("-sW", ht_method)
        when "-sM"
            exec("-sM", ht_method)
        when "-sN"
            exec("-sN", ht_method)
        when "-sF"
            exec("-sF", ht_method)
        when "-sX"
            exec("-sX", ht_method)
        when "-all"
            for f_all in method
                exec(f_all[0], ht_method)
            end
        else
            print("#{$line}\n[ERROR]: less_boring() execution fail, sad monkey\n")    
        end   
    end
    
end
