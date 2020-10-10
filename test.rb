
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

require_relative "./modules/engine.rb"
require_relative "./modules/visual.rb"
require_relative "./modules/colors.rb"


include Engine
include Colors
include Visual

puts "Cheep debug, hole shit!"; puts "Visual module"

Visual.banner()
Visual.web_dns()
Visual.linux_files()
Visual.linux_folders()
Visual.linux_util()

puts "Engine module"


# Test Function
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