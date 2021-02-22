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
        when 'call_rg'
            FakeDump.call_rg
        when 'call_cpf'
            FakeDump.call_cpf
        when 'call_gem'
            FakeDump.call_gem
        when 'simple_dump'
            FakeDump.simple_dump
        when 'll'
            Kernel.sys('ls -lha')
        when 'cls'
            Kernel.sys('clear')
        when 'sht '
            Kernel.sys('shutdown -h now')
        when 'rm '
            Kernel.sys('rm -rf')
        when 'cp'
            Kernel.sys('cp -r')
        when 'banner'
            Visual.banner
        when 'web_dns'
            Visual.web_dns
        when 'linux_files'
            Visual.linux_files
        when 'linux_folders'
            Visual.linux_folders
        when 'windows_files'
            Visual.windows_files
        when 'linux_util'
            Visual.linux_util
        when 'tor_search'
            Visual.tor_search
        when 'tor_alt'
            Visual.tor_alt
        when 'search'
            Kenrel.search
        when 'dns_scanner'
            Kenrel.dns_scanner
        when 'dir_scanner'
            Kenrel.dir_scanner
        when 'cover'
            Kenrel.cover
        when 'simple_map'
            Kenrel.simple_map
        when 'extract'
            Kenrel.extract
        when 'compress'
          Kenrel.compress
        else
            sys(opt)
        end

    end

end