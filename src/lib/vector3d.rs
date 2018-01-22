use std::ops;

pub struct Vector3d {
    x: f64,
    y: f64,
    z: f64
}

impl Vector3d {
    pub fn new (x: f64, y: f64, z: f64) -> Vector3d {
        Vector3d {x: x,
                  y: y,
                  z: z}
    }
    
    pub fn transform (&mut self, f: fn(&mut Vector3d)) {
        f(self)
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