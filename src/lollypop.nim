import osproc

proc sys*(command: string; verbose: bool = false): bool = 
    try:
        var a = execCmdEx(command)
        if verbose == true:
            echo a[0]
        return true
    except:
        echo "system_exec_command_error"
        return false
