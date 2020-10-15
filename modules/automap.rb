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

    $HOST_DISCOVERY = [
        ["-sL", "List Scan - simply list targets to scan"],
        ["-sP", "Ping Scan - go no further than determining if host is online"],
        ["-P0", "Treat all hosts as online -- skip host discovery"],
        ["-PS", "[portlist]: TCP SYN discovery probes to given ports"],
        ["-PA", "[portlist]: TCP ACK discovery probes to given ports"],
        ["-PU", "[portlist]: TCP UDP discovery probes to given ports"],
        ["-PE", "ICMP echo, timestamp, and netmask request discovery probes"],
        ["-PP", "ICMP echo, timestamp, and netmask request discovery probes"],
        ["-PM", "ICMP echo, timestamp, and netmask request discovery probes"],
        ["-n",  "Never do DNS resolution/Always resolve [default: sometimes resolve]"],
        ["-R",  "Never do DNS resolution/Always resolve [default: sometimes resolve]"],
        ["--system-dns", "Use OS's DNS resolver"]
    ]
  
      $SCAN_TECHNIQUES = [
        ["-sS", "TCP SYN"],
        ["-sT", "Connect()"],
        ["-sA", "ACK"],
        ["-sW", "Window"],
        ["-sM", "Maimon scans"],
        ["-sN", "TCP Null"],
        ["-sF", "FIN"],
        ["-sX", "Xmas scans"]
    ]
        
      $EVASION = [
        ["--spoof-mac", "<mac address, prefix, or vendor name>: Spoof your MAC address"],
        ["--data-length", "<num>: Append random data to sent packets"],
        ["-T", "[0-5] Set timing template (higher is faster)"],
        ["-sV", "Probe open ports to determine service/version info"],
        ["--version-intensity", "<level>: Set from 0 (light) to 9 (try all probes)"],
        ["--mtu", "<val>:  optionally w/given MTU"],
        ["--source-port", "<portnum>: Use given port number"],
        ["--ttl", "<val>: Set IP time-to-live field"],
        ["-f", "<val>: fragment packets"],
        ["-D", "<decoy1,decoy2[,ME],...>: Cloak a scan with decoys"],
        ["-S", "<IP_Address>: Spoof source address"],
        ["-e", "<iface>: Use specified interface"],
        ["--max-retries", "<tries>: Caps number of port scan probe retransmissions."],
        ["--host-timeout", "<time>: Give up on target after this long"],
        ["--scan-delay", "<time>: Adjust delay between probes"],
        ["--max-scan-delay", "<time>: Adjust delay between probes"]
    ]
  
    OS_DETECTION = [
        ["-O", "Enable OS detection (try 2nd generation, then 1st if that fails)"],
        ["-O1", "Only use the old (1st generation) OS detection system"],
        ["-O2", "Only use the new OS detection system (no fallback)"]
    ]

end