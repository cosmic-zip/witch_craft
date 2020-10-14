
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

require_relative "./modules/engine.rb"
require_relative "./modules/visual.rb"
require_relative "./modules/colors.rb"
require_relative "./modules/automap.rb"
require_relative "./modules/test.rb"

include Engine
include Colors
include Visual
include Automap
include Test

Test.debug_all()
