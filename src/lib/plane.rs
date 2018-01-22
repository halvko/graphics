use lib::vector3d::Vector3d;

pub struct Plane<'a>{
    a: &'a Vector3d,
    b: &'a Vector3d,
    c: &'a Vector3d
}

impl<'a> Plane<'a> {
    fn new(a: &'a Vector3d, b: &'a Vector3d, c: &'a Vector3d) -> Plane<'a> {
        Plane {a, b, c}
    }
}