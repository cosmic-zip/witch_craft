
#! /bin/ruby
#-------------------------------------------------------------
#
#                     Linux Evil Toolkit
# 
#                          By v0id
#
#
#-------------------------------------------------------------

module Test

	def set()
		$docmentation = true
	    $proxy        = true
	    $target       = "www.uol.com"
	    $ip           = "1.1.1.1"
	end

	def debug_visual()
		prCyan "Visual module"
		Visual.help()
		Visual.banner()
		Visual.web_dns()
		Visual.linux_files()
		Visual.linux_folders()
		Visual.linux_util()	
	end

	def debug_automap()
		prCyan "Automap"
		Automap.less_boring()
	end

	def debug_engine()
		prCyan "Engine module"
		prCyan "init"
		Engine.INIT()
		prCyan "system sys"
		Engine.sys("ls")
		prCyan "compress"
		Engine.compress()
		prCyan "search"
		Engine.search()
		prCyan "status"
		Engine.status()
		prCyan "dns scanner"
		Engine.dns_scanner()
	end

	def debug_all()
		prCyan "[CHEAP DEBUG]"
		Test.set()
		Test.debug_visual()
		Test.debug_automap()
		Test.debug_engine()
	end
	
end
