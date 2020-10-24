#-------------------------------------------------------------
#
#                     Linux Evil Toolkit
# 
#                          By v0id
#
#                        2019 - 2020
#
#-------------------------------------------------------------

module Colors
    
    def prRed(string);            puts "\033[91m #{string}\033[00m"; end
    def prGreen(string);          puts "\033[92m #{string}\033[00m"; end
    def prYellow(string);         puts "\033[93m #{string}\033[00m"; end
    def prLightPurple(string);    puts "\033[94m #{string}\033[00m"; end
    def prPurple(string);         puts "\033[95m #{string}\033[00m"; end
    def prCyan(string);           puts "\033[96m #{string}\033[00m"; end
    def prLightGray(string);      puts "\033[97m #{string}\033[00m"; end
    def prBlack(string);          puts "\033[98m #{string}\033[00m"; end

    def bg_black(string)       "\e[40m#{string}\e[0m" end
    def bg_red(string)         "\e[41m#{string}\e[0m" end
    def bg_green(string)       "\e[42m#{string}\e[0m" end
    def bg_brown(string)       "\e[43m#{string}\e[0m" end
    def bg_blue(string)        "\e[44m#{string}\e[0m" end
    def bg_magenta(string)     "\e[45m#{string}\e[0m" end
    def bg_cyan(string)        "\e[46m#{string}\e[0m" end
    def bg_gray(string)        "\e[47m#{string}\e[0m" end

    def bold(string)           "\e[1m#{string}\e[22m" end
    def italic(string)         "\e[3m#{string}\e[23m" end
    def underline(string)      "\e[4m#{string}\e[24m" end
    def blink(string)          "\e[5m#{string}\e[25m" end
    def reverse_color(string)  "\e[7m#{string}\e[27m" end
        
end