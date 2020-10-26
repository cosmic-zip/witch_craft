#-------------------------------------------------------------
#
#                     Linux Evil Toolkit
# 
#                          By v0id
#
#
#-------------------------------------------------------------

module Interpreter

    $PS1 = "\033[92m[LETK]::[anon]$ \033[00m" 

    def interpreter(props)    
        case props
            when 'exit'
                Engine.R()
                Engine.sys('clear')
                exit
            when 'clear'                    
                Engine.sys('clear')
            when 'update'
                Engine.sys('git pull')
            when 'train'
                Engine.sys('sl')
            when "INIT"
                Engine.INIT()
            when "reset"
                Engine.R()
            when "cover"
                Engine.cover()
            when "extract"
                Engine.extract()
            when "compress"
                Engine.compress()
            when "portscanner"
                Engine.port_scanner()
            when "search"
                Engine.search()
            when "status"
                Engine.status()
            when "dnsscanner"
                Engine.dns_scanner()
            when "dirscanner"
                Engine.dir_scanner()
            when "banner"
                Visual.banner()
            when "help"
                Visual.help()
            when "webdns"
                Visual.web_dns()
            when "linuxfiles"
                Visual.linux_files()
            when "linuxfolders"
                Visual.linux_folders()
            when "linuxutil"
                Visual.linux_util()	
            when "automap"
                Automap.less_boring()
            when "test"
                Test.debug_all()
            else
                prCyan "\n[COMMAND ERROR]: Option is ivalid, look for monkeys"
        end
    end

    def main()
        # main while in russia
        while true
            interpreter('clear')
            interpreter('banner')
            while true
                print($PS1); command = gets.chomp.to_s
                interpreter(command)
            end
        end
    end

end