pub struct CurlBind<'method, 'auth_type, 'auth_token, 'url, 'ctn_type, 'data> {
    pub method: &'method str,
    pub auth_type: &'auth_type str,
    pub auth_token: &'auth_token str,
    pub url: &'url str,
    pub ctn_type: &'ctn_type str,
    pub data: &'data str,
}
