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
	
	def fedora_linux
		prCyan "#{$line}Install dependences on Fedora Linux"
		cmd = sys("dnf group install 'security lab' -y")
		cmd == true ? prYellow("Dependences instaled") : prRed("[SYSTEM_ERROR]: dnf error")
 	end

 	def arch_linux
 		# Nothing
    end


end
