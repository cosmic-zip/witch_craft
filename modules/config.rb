#-------------------------------------------------------------
#
#
#                     Linux Evil Toolkit
#                        By TH3V0ID
#
#
#-------------------------------------------------------------
# Copyright (c) 2020 - 2022 Th3Void <https://github.com/th3void>
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
#
# Shell name
$ps = "\033[93m [Zynix] :: [Shell] :: \033[00m"

# Applications state
$state = nil

#line 
$line = '------------------------------'

# Shell command documentation
$doc = false

# Time
$time = get_time()

# Silent mode
$sil = false

# DNS and ip
$ip = nil
$target = nil 