#! /bin/ruby
#-------------------------------------------------------------
#
#                     Linux    Evil Toolkit
# 
#                       By Xx_ZEREF_xX
#
#-------------------------------------------------------------

def banner()

puts ' _      _                    ______     _ _   _______          _ _    _ _   '
puts '| |    (_)                  |  ____|   (_) | |__   __|        | | |  (_) |  '
puts '| |     _ _ __  _   ___  __ | |____   ___| |    | | ___   ___ | | | ___| |_ '
puts "| |    | | '_ \\| | | \\ \\/ / |  __\\ \\ / / | |    | |/ _ \\ / _ \\| | |/ / | __|"
puts "| |____| | | | | |_| |>  <  | |___\ V /| | |    | | (_) | (_) | |   <| | |_ "
puts "|______|_|_| |_|\\__,_/_/\\_\\ |______\\_/ |_|_|    |_|\\___/ \\___/|_|_|\\_\\_|\\__|"

end

def screen_options()

  puts "\n"; puts 'OPTIONS: scanner install_arch install_fedora blakcarch_repo'
  puts 'install_metasploit http_server screen screen_options'
  puts "\n\n"  
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

def ERROR_RETURN(type, error, return_success=false)

  PACKAGE_INSTALLED_SUCC = "[SUCCESSFUL] Package #{pak} installed"
  PACKAGE_INSTALLED_FAIL = "[WARNING] Package #{pak} not installed"
  CMD_NOT_FOUND = "[WARNING] Command #{cmd} not found"
  CMD_FAIL = "[WARNING] Command #{cmd} fail"
  CMD_NO_ROOT = "[WARNING] Command #{cmd} require root"
  CMD_SUCCESSFUL = "[SUCCESSFUL] Command #{cmd} exec successful"
  IN_FAIL = "[WARNING] Input fail"

  #-------------------------------------------------------#

  if type == "package" 

    if error == true
      puts PACKAGE_INSTALLED_SUCC
    else
      puts PACKAGE_INSTALLED_FAIL
    end  

  elsif type == "cmd"

    if cmd == nil 
      puts CMD_NOT_FOUND
    elsif cmd == false
      puts CMD_FAIL
    elsif return_success == true and cmd == true
      puts CMD_SUCCESSFUL
    else 
      puts IN_FAIL
    end

end  

def metasploit()

  cmd = system("curl https://raw.githubusercontent.com/rapid7/metasploit-omnibus/master/config/templates/metasploit-framework-wrappers/msfupdate.erb > msfinstall && \
  chmod 755 msfinstall && \
  ./msfinstall")
  ERROR_RETURN("cmd", cmd, debug)

end

def blakcarch_repo()

  cmd = system("curl -O https://blackarch.org/strap.sh")
  ERROR_RETURN("cmd", cmd, debug)
  cmd = system("echo 9c15f5d3d6f3f8ad63a6927ba78ed54f1a52176b strap.sh | sha1sum -c")
  ERROR_RETURN("cmd", cmd, debug)
  cmd = system("chmod +x strap.sh")
  ERROR_RETURN("cmd", cmd, debug)
  cmd = system("sudo ./strap.sh")
  ERROR_RETURN("cmd", cmd, debug)

end

def install_fedora()

  system("dnf group install ")

end
  
def install_arch(debug=false) 

    IO.foreach('./tools.ttx',) do |line|  
      cmd = system("sudo pacman --noconfirm -S #{line}")
      ERROR_RETURN("cmd", cmd, debug)
    end

end

def server(port)

  print "set port: "; port = gets.chomp.to_s
  system("ruby -run -e httpd . -p #{port}")

end

def scanner()

  print('Set host ip or dns: '); hst = gets.chomp.to_s
  print('Set dnsmap adicional flags: [optional]'); add = gets.chomp.to_s
  print('User proxy: [yes|no]');proxy = gets.chomp.to_s
  if proxy == 'yes'; proxy = true; else; proxy = false; end
  print('Debug: [yes|no]');debug = gets.chomp.to_s
  if proxy == 'yes'; debug = true; else; debug = false; end
  line = "\n\n[+]--------------------------------------[+]\n\n"
  # Why?
  if proxy == true; proxy = 'proxychains'; end 
  puts line; cmd = system("#{proxy} nslookup #{hst}")
  ERROR_RETURN("cmd", cmd, debug)
  puts line; cmd = system("#{proxy} dnsmap #{add} #{hst}")
  ERROR_RETURN("cmd", cmd, debug)
  puts line; cmd = system("#{proxy} wohis -A #{hst}")
  ERROR_RETURN("cmd", cmd, debug)
  puts line; cmd = system("#{proxy} nikto --host #{hst}")
  ERROR_RETURN("cmd", cmd, debug)
  puts line; cmd = system("#{proxy} nmap -sS -sv -A --spoof-mac --data 64 #{hst}")
  ERROR_RETURN("cmd", cmd, debug)
  puts line; cmd = system("#{proxy} nmap --udp -sV -A --spoof-mac --data 64 #{hst}")
  ERROR_RETURN("cmd", cmd, debug)

end

def change_mac(board_name)
  cmd = system("ifconfig #{board_name} down")
  cmd = system("machanger -r #{board_name}")
  cmd = system("ifconfig #{board_name} up")
    
end

def shell(debug=false)
  
  PS1='\[\]\u:\[\]\[\033[38;5;33m\]\W\[\] \[\]\$\[\] '
  while true

    print(PS1); var = gets.chomp.to_s
    if var == 'screen'
      screen()
    elsif var == 'screen_options'
      screen_options()
    elsif var ==  'http_server'; 
      server()
    elsif var == 'metaesploit'
      metaesploit()
    elsif var == 'blakcarch_repo'
      blakcarch_repo()
    elsif var == 'install_arch'
      install_arch()
    elsif var == 'install_fedora'
      install_fedora()
    elsif var == 'scanner'
      print scanner()
    elsif var == 'change_mac'
      change_mac()
    else 
      puts "not work, why?"
    end # end if input

  end # end while
  
end # end shell func

