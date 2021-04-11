# This is just an example to get you started. A typical binary package
# uses this file as the main entry point of the application.

import cute
import lollypop

when isMainModule:
    discard sys("ls -la", true)
    cute("BANNER")

