module Shell

    def cow
        sys('reset')
        banner() # Show banner
    end

    def main
        cow()
        while true
            print $ps; cmd = gets.chomp.to_s
            cmd == 'exit' ? exit() : nil
            shell(cmd)
        end
    end

    def shell(opt)

        case opt
        # From shell
        when 'rsh --cow' || '1'
            cow()
    
        # From plugins
        when 'plugin --fpage' || '2'
            FakePages.gem_fake_page
        when 'help'
            NumberMenu.help || 0
        when 'menu'
            NumberMenu.menu

        # From fake dump
        when 'gem --g rg' || '3'
            FakeDump.call_rg
        when 'gem --g cpf' || '4'
            FakeDump.call_cpf
        when 'gem --g name' || '5'
            FakeDump.call_gem
        when 'gem --g fakedb' || '6'
            FakeDump.simple_dump

        # From visual
        when 'vs --v banner'  || '7'
            Visual.banner
        when 'vs --v dns'  || '8'
            Visual.web_dns
        when 'vs --v lfile'  || '9'
            Visual.linux_files
        when 'vs --v lfolder'  || '10'
            Visual.linux_folders
        when 'vs --v wfile'  || '11'
            Visual.windows_files
        when 'vs --v luss'  || '12'
            Visual.linux_util
        when 'vs --v tsearch'  || '13'
            Visual.tor_search
        when 'vs --v talt'  || '14'
            Visual.tor_alt

        # Alias
        when 'ls' || '15'
            Kernel.sys('ls -lha')
        when 'cls' || '16'
            Kernel.sys('clear')
        when 'sdown' || '17'
            Kernel.sys('shutdown -h now')
        when 'rm' || '18'
            Kernel.sys('rm -rf')
        when 'cp' || '19'
            Kernel.sys('cp -r')

        # From kernel    
        when 'kl --init' || '20'
            Kernel.init
        when 'kl --cover' || '21'
            Kernel.cover
        when 'kl --compress' || '22'
            Kernel.compress
        when 'kl --extract' || '23'
            Kernel.extract
        when 'kl ---ginfo' || '24'
            Kernel.grep_domain_info
        when 'kl --dns --brute' || '25'
            Kernel.dns_brute
        when 'kl --dns --scan' || '26'
            Kernel.dns_scanner
        when 'kl --dir --scan' || '27'
            Kernel.dir_scanner
        when 'kl --smap ' || '28'
            Kernel.simple_map
        when 'kl --maclookup' || '29'
            Kernel.maclookup
        when 'kl --myip' || '30'
            Kernel.myip

        # Else
        else 
            sys(opt)    
        end # case end
    
    end # fn end

end #END