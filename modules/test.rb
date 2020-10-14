
#! /bin/ruby
#-------------------------------------------------------------
#
#                     Linux Evil Toolkit
# 
#                          By v0id
#
#                        2019 - 2020
#
#-------------------------------------------------------------
module Test

	def set()
		$docmentation = false
	    $proxy        = false
	    $doc_name     = false
	    $target       = "www.uol.com"
	    $ip           = "1.1.1.1"
	end


	def debug_visual
		puts "Visual module"
		Visual.banner()
		Visual.web_dns()
		Visual.linux_files()
		Visual.linux_folders()
		Visual.linux_util()	
	end

	def debug_automap()
		# ...
	end

	def debug_engine()
		puts "Engine module"
		puts "init"
		Engine.INIT()
		puts "system sys"
		Engine.sys("ls")
		puts "compress"
		Engine.compress()
		puts "port scanner"
		Engine.port_scanner()
		puts "search"
		Engine.search()
		puts "status"
		Engine.status()
		puts "dns scanner"
		Engine.dns_scanner()
	end

	def debug_all()
		puts "Cheep debug, hole shit!"
		Test.set()
		Test.debug_visual()
		Test.debug_automap()
		Test.debug_engine()
	end
	
end
