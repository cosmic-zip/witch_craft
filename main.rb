#------------------------------------
#
#
# 			by th3void
# 		   11 - 02 - 2020
#
#
#------------------------------------

#PATH
$PATH = './modules'
$PLUGINS = './plugins'

# Modules 
require_relative "#{$PATH}/kernel.rb"
require_relative "#{$PATH}/config.rb"
require_relative "#{$PATH}/fakedump.rb"
require_relative "#{$PATH}/visual.rb"
require_relative "#{$PATH}/shell.rb"

# Plugins
require_relative "#{$PLUGINS}/numberMenu/numberMenu.rb"
require_relative "#{$PLUGINS}/fakePages/fakePages.rb"
require_relative "#{$PLUGINS}/sql/sqlinj.rb"

# Include
include NumberMenu
include Sqlinj
include FakePages
include FakeDump
include Kernel
include Visual 
include Shell

# Require
require "builder"
require "openssl"
require "csv"

# Init
Shell.main()