use crate::core::consts::SYSINFO_MAID;

pub fn sysinfo_maid(_argsv: &[String]) -> i32 {
    println!("{}", SYSINFO_MAID);
    return 0;
}
