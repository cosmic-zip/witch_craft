pub struct SampleData<'data> {
    pub data: &'data str,
}

pub struct OsintLocationOSM<'city, 'zoom, 'long, 'lati, 'path> {
    pub query: &'city str,
    pub zoom: &'zoom str,
    pub long: &'long str,
    pub lati: &'lati str,
    pub path: &'path str,
}