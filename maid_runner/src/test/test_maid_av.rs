use super::*;
use crate::*;

#[test]
fn test_maid_av_pass() {
    let debug = true;
    let instance = "2dd12ba5ec9cefe83d3d75f694fcb042e38bfa2497f7faab35925f502aa01200";

    // search_malware_hash(instance, debug)
    let exit = search_malware_pattern(instance, debug);
    assert_eq!(exit, true);
}

#[test]
fn test_calculate_sha256_hash() {
    let expected = "272eaffd682b22746d1eb86a1d86ab2e9fb7f49dd6985a652ac41932d45e10dc";
    match calculate_sha256_hash("/bin/yes", true) {
        Ok(result) => assert_eq!(result, expected),
        Err(err) => eprintln!("{}", err),
    }
}

#[test]
fn test_active_malware_scanner() {
    let exit = active_malware_scanner("./", true);
    assert_eq!(exit, true);
}
