# Contributing to this project <!-- omit in toc -->

## Getting started <!-- omit in toc -->

### Before you begin:

- This project is powered by ruby lang
- Files must have short descriptive names and by default use 
camelCase, with the first lower case.- Functions follow the
pattern of having a short name and in the case of more than
one name, separate them by underline. 
- Please write the code in English.
- Document your changes.
- Do not use auto promote.
- Spelling mistakes are tolerable, but avoid.

### New features

    Inside the module folder, write your code in a new file, define 
    the name of the module with the first capital letter and import
    it into the main.rb file along with its dependencies.

    Then write the dependencies in the ruby_dep function in the 
    module install.rb inside the folder modules.

    write in the module to interpreter a condition with an alias to
    call each function relevant to the module.

    by default do not use parameters in the functions that will be 
    called by the interpreter module. write on the screen the parameters 
    that the user must inform. Whenever necessary, use the global 
    variables inside config/letk.rb in the same style as the previous
    ones.

### Global variables

    --> Set time
        $time = Time.now.strftime("%d-%m-%Y_%H-%M")
    --> Write results in file?
        $documentation = false
    --> Enable proxy
        $proxy         = false
    --> Set target. Ex: www.google.com
        $target        = 'www.google.com'
    --> Set target ip. Ex: 192.0.0.1
        $ip            = 'localhost'
    --> automatically generate new ip. [true| false]
        $silent_mode   = false
    --> Change mac [true| false]
        $change_mac = false
    --> Set network interface name: [wlan0, wlp2s0]: "
        $interface = false

### Config Files

    If your modification needs a configuration or control file, place
    it in the config folder.

### database

    Databases in txt, json, sqllite, etc. must be put into the databases
    folder.

### wordlists

    wordlists of any kind must be placed in the wordlist folder.

## Common issues:

    * Option is invalid:

        If you receive this message in the console: "Option is invalid",
        read the documentation or type help to see the options.

    * Imported module not found or file not found or Could not find the gem:

        ruby gems must be imported into the main.rb file, but during
        testing only the module these dependencies will not be imported.

    * SYS_COMMAND_ERROR:

        The command cannot be executed because it is not installed 
        on your system.

    * SYS_PROXY__ERROR:

        proxychains is not installed.

    * SYS_DOCUMENTATION_ERROR:

        This feature will no longer be implemented, do not use.

    * SYS_DOC_PROXY_ERROR:

        This feature will no longer be implemented, do not use 
        and proxychains is not installed.

## small details




