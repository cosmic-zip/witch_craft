require './modules/core.rb'
require './modules/shell.rb'
require './modules/visual.rb'

include Core
include Shell
include Visual


Shell.init()