use std::ops;

pub struct Vector3d {
    x: f64,
    y: f64,
    z: f64
}

impl Vector3d {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3d {
        Vector3d {x: x,
                  y: y,
                  z: z}
    }
    
    pub fn transform(&mut self, f: fn(&mut Vector3d)) {
        f(self)
    }
}

impl Eq for Vector3d {}

impl PartialEq<Vector3d> for Vector3d {
    fn eq(&self, other: &Vector3d) -> bool {
        self.x == other.x &&
        self.y == other.y &&
        self.z == other.z
    }

    fn ne(&self, other: &Vector3d) -> bool {
        !Vector3d::eq(self, other)
    }
}

impl ops::Add<Vector3d> for Vector3d {
    type Output = Vector3d;

    fn add(self, other: Vector3d) -> Vector3d {
        Vector3d::new(self.x + other.x,
                      self.y + other.y,
                      self.z + other.z)
    }
}

impl ops::Sub<Vector3d> for Vector3d {
    type Output = Vector3d;

    fn sub(self, other: Vector3d) -> Vector3d {
        Vector3d::new(self.x - other.x,
                      self.y - other.y,
                      self.z - other.z)
    }
}

impl ops::Mul<Vector3d> for Vector3d {
    type Output = Vector3d;

    fn mul(self, other: Vector3d) -> Vector3d {
        Vector3d::new(self.x * other.x,
                      self.y * other.y,
                      self.z * other.z)
    }
}

impl ops::Mul<f64> for Vector3d {
    type Output = Vector3d;

    fn mul(self, other: f64) -> Vector3d {
        Vector3d::new(self.x * other,
                      self.y * other,
                      self.z * other)
    }
}

impl ops::Div<Vector3d> for Vector3d {
    type Output = Vector3d;

    fn div(self, other: Vector3d) -> Vector3d {
        Vector3d::new(self.x / other.x,
                      self.y / other.y,
                      self.z / other.z)   
    }
}

impl ops::Div<f64> for Vector3d {
    type Output = Vector3d;

    fn div(self, other: f64) -> Vector3d {
        Vector3d::new(self.x * other,
                      self.y * other,
                      self.z * other)
    }
}

#[cfg(test)]
mod test {
    use super::Vector3d;

    // macro_rules! build_vector3d {
    //     ($x:ty , $y:ty , $z:ty; $name:ident) => (
    //         let ($name)_x: f64 = $x;
    //         let ($name)_y: f64 = $y;
    //         let ($name)_z: f64 = $z;
    //         let $name: Vector3d = Vector3d::new($x, $y, $z);
    //     )
    // }

    #[test]
    fn new_test() {
        //build_vector3d!(1f64, 2f64, 1f64 -> vec);
        let x = 1f64;
        let y = 2f64;
        let z = 1f64;
        let vec = new(x, y, z);
        
        assert_eq!(vec.x, x);
        assert_eq!(vec.y, y);
        assert_eq!(vec.z, z);
    }

    #[test]
    fn eq_test() {
        let vec_0 = new(1f64, 2f64, 1f64);
        let vec_1 = new(1f64, 2f64, 1f64);
        
        assert_eq!(vec_0, vec_1);
    }
}