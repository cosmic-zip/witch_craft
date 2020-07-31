#! /bin/ruby
#-------------------------------------------------------------
#
#                     Linux Evil Toolkit
# 
#                       By Xx_ZEREF_xX
#
#-------------------------------------------------------------

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

def metaesploit()

  cmd = system("curl https://raw.githubusercontent.com/rapid7/metasploit-omnibus/master/config/templates/metasploit-framework-wrappers/msfupdate.erb > msfinstall && \
  chmod 755 msfinstall && \
  ./msfinstall")
  ERROR_RETURN("cmd", cmd, debug)

end

def blakcarch_repo()

  system("curl -O https://blackarch.org/strap.sh")
  system("echo 9c15f5d3d6f3f8ad63a6927ba78ed54f1a52176b strap.sh | sha1sum -c")
  system("chmod +x strap.sh")
  system("sudo ./strap.sh")

end

def install(debug=false) 

    IO.foreach('./tools.ttx',) do |line|  
      cmd = system("sudo pacman --noconfirm -S #{line}")
      ERROR_RETURN("cmd", cmd, debug)
    end

end

def server(port)
  print "set port: "; port = gets.chomp.to_s
  system("ruby -run -e httpd . -p #{port}")
end

def scanner(hst, wrl, proxy='', debug=false)
  line = "\n\n[+]--------------------------------------[+]\n\n"
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


