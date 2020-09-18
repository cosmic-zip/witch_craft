
#! /bin/ruby
#-------------------------------------------------------------
#
#                     Linux Evil Toolkit
# 
#                          By v0id
#
#                        2019 - 2020
#
#-------------------------------------------------------------

require_relative "./modules/core.rb"
require_relative "./modules/shell.rb"
require_relative "./modules/visual.rb"

include Core
include Shell
include Visual

# INIT
Visual.banner()
while true
    Shell.interpreter(Shell.PS1())    
end

# END MAIN