pub struct ScannerWebGenericInput<'t, 'o, 'l> {
    pub target: &'t str,
    pub op_type: &'o str,
    pub list_path: &'l str,
}

pub struct ScannerWebAutoNmap<'t, 'd, 'p, 's> {
    pub target: &'t str,
    pub delay: &'d str,
    pub ports: &'p str,
    pub scan_type: &'s str,
}
