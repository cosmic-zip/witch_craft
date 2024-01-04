#[derive(Debug)]
pub struct Cubes<'path, 'name, 'disk, 'ram> {
    pub path: &'path str,
    pub name: &'name str, 
    pub disk: &'disk str,
    pub ram : &'ram str,
}

