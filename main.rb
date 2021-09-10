#! /bin/ruby -w
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
require_relative "#{$PATH}/names.rb"
require_relative "#{$PATH}/visual.rb"
require_relative "#{$PATH}/shell.rb"

# Plugins
require_relative "#{$PLUGINS}/manuals/manuals.rb"

# Include Modules
include FakeDump
include Kernel
include Visual 
include Shell

# Include Plugins
include Manuals

# Require
require "csv"
require "openssl"

# Init
Shell.main()