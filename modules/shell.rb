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
                
        when 'll'
            sys('ls -lha')
        when 'cls'
            sys('clear')
        when 'sht '
            sys('shutdown -h now')
        when 'rm '
            sys('rm -rf')
        when 'cp'
            sys('cp -r')
        when 'banner'
            banner
        when 'web_dns'
            web_dns
        when 'linux_files'
            linux_files
        when 'linux_folders'
            linux_folders
        when 'windows_files'
            windows_files
        when 'linux_util'
            linux_util
        when 'tor_search'
            tor_search
        when 'tor_alt'
            tor_alt
        when 'search'
            search
        when 'dns_scanner'
            dns_scanner
        when 'dir_scanner'
            dir_scanner
        when 'cover'
            cover
        when 'simple_map'
            simple_map
        when 'extract'
            extract
        when 'compress'
            compress
        when 'rg'
            rg
        when 'cpf'
            cpf
        when 'gem'
            gem
        when 'idhash'
            idhash
        when 'simple_dump'
            simple_dump
        when 'dump_xml'
            dump_xml
        else
            sys(opt)
        end

    end

end