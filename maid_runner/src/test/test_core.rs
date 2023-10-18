use super::*;
use crate::*;

#[test]
fn test_maid_runner_command_chain() {
    let _exp_shell_env = "target/debug/maid_runner";
    let _exp_shell_code = "0";
    let _exp_shell_out = r###" 
    linux-vdso.so.1 (0x00007fffffba3000)
	libc.so.6 => /lib/x86_64-linux-gnu/libc.so.6 (0x00007f53e6a13000)
	/lib64/ld-linux-x86-64.so.2 (0x00007f53e6c17000)
    "###;

    let _current_shell_env = system_exec_shell(true);
    let current_cmd = "ldd /bin/cat";
    let msg = "test_maid_runner_command_chain unit testing";
    let res = system_command_exec(current_cmd, msg, true);

    assert_eq!(res, true);
}

#[test]
fn test_system_command_deep_exec() {
    let current_cmd = "ls -lha";

    match system_command_deep_exec(current_cmd, true) {
        Ok(res) => {
            if res.stdout != "none".to_string() {
                system_text(&res.stdout, "green");
            }
            system_text(&res.stdout, "red");
        }
        Err(_e) => eprintln!("Fail system_command_deep_exec toolchain"),
    }
}
