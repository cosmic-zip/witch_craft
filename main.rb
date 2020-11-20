
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

# Simplicity as supreme sophistication ~Leonardo da Vinci

require_relative "./modules/interpreter.rb"
require_relative "./modules/engine.rb"
require_relative "./modules/visual.rb"
require_relative "./modules/colors.rb"
require_relative "./modules/test.rb"
require_relative "./modules/fakedump.rb"
require_relative "./database/names.rb"

require ''
require ''

include Interpreter
include Engine
include Colors
include Visual
include Test
include FakeDump

# ------------- start ----------------------------------------

Interpreter.main()

# -------------- end -----------------------------------------

