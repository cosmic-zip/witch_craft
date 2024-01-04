#[derive(Debug)]
pub struct Cubes<'arch, 'path, 'name, 'disk, 'ram> {
    pub arch: &'arch str,
    pub path: &'path str,
    pub name: &'name str, 
    pub disk: &'disk str,
    pub ram : &'ram str,
}

