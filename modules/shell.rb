#-------------------------------------------------------------
#
#
#                     Zynix Fusion
#                        By TH3V0ID
#
#
#-------------------------------------------------------------
# Copyright (c) 2020 - 2022 th3maid <https://github.com/th3maid>
#
# This program is free software; you can redistribute it and/or
# modify it under the terms of the GNU General Public
# License as published by the Free Software Foundation; either
# version 3 of the License, or (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
# General Public License for more details.
#
# You should have received a copy of the GNU General Public
# License along with this program; if not, write to the
# Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
# Boston, MA 02110-1301 USA

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
        when 'visual --banner'
            Visual.banner
        when 'visual --web-dns'
            Visual.web_dns
        when 'visual --linux-files'
            Visual.linux_files
        when 'visual --linux-folders'
            Visual.linux_folders
        when 'visual --win-files'
            Visual.windows_files
        when 'visual --linux-utilites'
            Visual.linux_util
        when 'visual --tor-search-link'
            Visual.tor_search
        when 'visual --tor-alt'
            Visual.tor_alt
        when 'visual --help'
            Manuals.help_simple
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
        when 'help'
            Manuals.help_simple
        when 'help --simple-map'
            Manuals.help_simple_map
        else
            sys(opt)
        end

    end

end
