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

pub struct IpGeoLocation<'ip_string> {
    pub ip_string: &'ip_string str,
}

pub struct CityGeoLocation<'city, 'path> {
    pub city: &'city str,
    pub path: &'path str,
}
