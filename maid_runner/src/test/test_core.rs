use super::*;
use crate::core::structs::ProcessInit;
use crate::*;

#[test]
fn test_maid_runner_command_chain() {
    let _exp_shell_env = "target/debug/maid_runner";
    let _exp_shell_code = "0";
    let _exp_shell_out = r#" 
    linux-vdso.so.1 (0x00007fffffba3000)
	libc.so.6 => /lib/x86_64-linux-gnu/libc.so.6 (0x00007f53e6a13000)
	/lib64/ld-linux-x86-64.so.2 (0x00007f53e6c17000)
    "#;

    let _current_shell_env = system_exec_shell(true);
    let instance = ProcessInit {
        source: "ldd /bin/cat",
        source_from: "test",
        source_description: "Running test_maid_runner_command_chain unit testing",
        debug: true,
    };
    let res = system_command_exec(instance);

    assert_eq!(res, true);
}

// DELETED → fn test_system_command_deep_exec()
