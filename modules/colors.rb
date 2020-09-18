module Colors
    
    def black(string)          "\e[30m#{string}\e[0m" end
    def red(string)            "\e[31m#{string}\e[0m" end
    def green(string)          "\e[32m#{string}\e[0m" end
    def brown(string)          "\e[33m#{string}\e[0m" end
    def blue(string)           "\e[34m#{string}\e[0m" end
    def magenta(string)        "\e[35m#{string}\e[0m" end
    def cyan(string)           "\e[36m#{string}\e[0m" end
    def gray(string)           "\e[37m#{string}\e[0m" end

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