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
        when 'q'
            exit()
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
        when 'help'
            Visual.help
        when 'search'
            Kernel.search
        when 'init'
            Kernel.init
        when 'install'
            Kernel.install
        when 'dns_scanner'
            Kernel.dns_scanner
        when 'dir_scanner'
            Kernel.dir_scanner
        when 'cover'
            Kernel.cover
        when 'simple_map'
            Kernel.simple_map
        when 'maclookup'
            Kernel.maclookup
        when 'extract'
            Kernel.extract
        when 'compress'
            Kernel.compress
        when 'bluetooth'
            Kernel.bluetooth
        else
            sys(opt)
        end

    end

end