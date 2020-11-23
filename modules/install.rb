#-------------------------------------------------------------
#
#                     Linux Evil Toolkit
# 
#                          By v0id
#
#
#-------------------------------------------------------------

module Install

	def ruby_dep
		system 'gem install openssl builder'
	end
	
	def read_req 
		text=File.open('./config/requirements.txt').read
		text.gsub!(/\r\n?/, "\n")
		text.each_line do |line|
			return line.to_s
		end
	end

	def fedora_linux
		prCyan "#{$line}Install dependences on Fedora Linux"
		cmd = sys("dnf group install 'security lab' -y")
		cmd == true ? prYellow("Dependences instaled") : prRed("[SYSTEM_ERROR]: dnf error")
 	end

 	def arch_linux
		aur = system 'sudo pacman -Syu yay'
		if cmd == true 
			prYellow('AUR installed...')	 			 
		else
			prRed("[SYSTEM_ERROR]: pacman not found") 
		end
    end


end
