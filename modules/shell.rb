module Shell

    def main
        quit = false
        sys('reset')
        banner
        while true
            print $ps; cmd = gets.chomp.to_s
            cmd == 'exit' ? exit() : nil
            shell(cmd)
        end
    end

    def shell(opt)

        case opt
        when 'generate --rg'
            FakeDump.call_rg
        when 'generate --cpf'
            FakeDump.call_cpf
        when 'generate --person'
            FakeDump.call_gem
        when 'generate --fakedump'
            FakeDump.simple_dump
        when 'visuall --banner'
            Visual.banner
        when 'visuall --web-dns'
            Visual.web_dns
        when 'visuall --linux-files'
            Visual.linux_files
        when 'visuall --linux-folders'
            Visual.linux_folders
        when 'visuall --win-files'
            Visual.windows_files
        when 'visuall --linux-utilites'
            Visual.linux_util
        when 'visuall --tor-search-link'
            Visual.tor_search
        when 'visuall --tor-alt'
            Visual.tor_alt
        when 'visuall --help'
            Visual.help
        when 'km --dlookup'
            Kernel.search
        when 'km --init'
            Kernel.init
        when 'km --install'
            Kernel.install
        when 'km --dns-scanner'
            Kernel.dns_scanner
        when 'km --dir-scanner'
            Kernel.dir_scanner
        when 'km --cover'
            Kernel.cover
        when 'km --simple-map'
            Kernel.simple_map
        when 'km --maclookup'
            Kernel.maclookup
        when 'km --extract'
            Kernel.extract
        when 'km --compress'
            Kernel.compress
        when 'km --blue-attck'
            Kernel.bluetooth
        else
            sys(opt)
        end

    end

end
