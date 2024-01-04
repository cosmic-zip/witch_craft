pub struct LookupGenericPathOpType<'s, 'o> {
    pub sample_path: &'s str,
    pub op_type: &'o str,
}

pub struct LookupMacAddress<'m, 'l> {
    pub vendor_mac: &'m str,
    pub list_path: &'l str,
}
