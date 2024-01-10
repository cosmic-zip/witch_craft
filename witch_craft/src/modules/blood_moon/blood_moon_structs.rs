pub struct SampleData<'data> {
    pub data: &'data str,
}

pub struct BloodMoonBackdoorConfig<'url, 'ask, 'info> {
    pub url: &'url str,
    pub ask_command: &'ask str,
    pub get_target_info: &'info str,
}