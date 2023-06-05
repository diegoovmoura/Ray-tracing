use std::ops;

// #[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    e: [f32;3],
}

impl Vec3{
    pub fn new(e0: f32, e1: f32, e2: f32) -> Vec3 {
        Vec3 {
            e: [e0, e1, e2]
        }
    }

    pub fn x(self) -> f32 {
        self.e[0]
    }
    pub fn y(self) -> f32 {
        self.e[1]
    }
    pub fn z(self) -> f32 {
        self.e[2]
    }

    pub fn r(self) -> f32 {
        self.e[0]
    }
    pub fn g(self) -> f32 {
        self.e[1]
    }
    pub fn b(self) -> f32 {
        self.e[2]
    }

    pub fn length(self) -> f32 {
        (self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]).sqrt()
    }

    pub fn unit_vector(v: &Vec3) -> Vec3 {
        *v / v.length()
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, _rhs: Vec3) -> Self::Output {
        Vec3 { e: 
            [self.e[0] + _rhs.e[0] 
            ,self.e[1] + _rhs.e[1]
            ,self.e[2] + _rhs.e[2]] 
            }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 {
            e: [self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs]
        }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        let k = 1.0/rhs;
        Vec3 {
            e: [self.e[0] * k, self.e[1] * k, self.e[2] * k]
        }
    }
}
