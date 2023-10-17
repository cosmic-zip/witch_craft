use super::*;
use crate::*;

#[test]
fn test_lookup_mac_addr() {
    let config = read_meow("/var/maid/maid_lists/embedded/config.meow", false);
    let file = &format!("{}{}", config["GENRAL_BASE_PATH"], config["MACADDR"]);

    let lk_mac = LookupMacAddress {
        vendor_mac: "01:23",
        list_path: file,
    };

    let output = lookup_mac_address(lk_mac, true);
    assert_eq!(output, true);
}

#[test]
fn test_lookup_reverse_engineering() {
    let opt = vec!["s".to_string(), "h".to_string(), "l".to_string()];

    for item in opt {
        let lk_re = LookupGenericPathOpType {
            sample_path: "/bin/cat",
            op_type: &item,
        };

        let output = lookup_reverse_engineering(lk_re, true);
        assert_eq!(output, true);
    }
}

#[test]
fn test_lookup_exif_metadata() {
    let photo = LookupGenericPathOpType {
        sample_path: "../docs/images/maid.png",
        op_type: "none",
    };
    let output = lookup_exif_metadata(photo, true);
    assert_eq!(output, true);
}
