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
        ["-PM", "netmask request discovery probes"],
        ["-n",  "Never do DNS resolution [default: sometimes resolve]"],
        ["-R",  "Always resolve [default: sometimes resolve]"],
        ["--system-dns", "Use OS's DNS resolver"]
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

    # test
    def sys(p)
        system(p)
    end

    def assembly(method_flag, host_dicovery, os)
        mac = $VEN[rand(0..14)]
        return "nmap #{method_flag} #{host_dicovery} #{os} #{$target} --spoof-mac #{mac[0]} --data-length #{rand(2..256)} --max-retries 10 --mtu 1024 --host-timeout 30 --ttl 60 -f #{rand(1..6)}"
    end

    def simple()
        for flag in $SCA 
            
        end
    end

end


include Automap

Automap.simple()