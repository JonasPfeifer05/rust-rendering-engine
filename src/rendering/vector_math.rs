use std::ops::{Add, Div, Mul, Sub};
use rand::Rng;

pub struct Vec3(pub f32, pub f32, pub f32);

impl Vec3 {
    pub fn length_squared(&self) -> f32 {
        return self.0*self.0 + self.1*self.1 + self.2*self.2;
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn unit(&self) -> Self {
        *self / self.length()
    }

    pub fn random_vector() -> Self {
        let mut rng = rand::thread_rng();

        Vec3(rng.gen(), rng.gen(), rng.gen())
    }

    pub fn random_vector_ranged(min: f32, max: f32) -> Self {
        Vec3::random_vector() * (max - min) + min
    }

    pub fn random_unit_vector() -> Self {
        Vec3::random_vector().unit()
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let random = Vec3::random_vector_ranged(-1.0, 1.0);
            if random.length_squared() >= 1.0 {
                continue;
            }
            return random;
        }
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        return self.0.abs() < s && self.1.abs() < s && self.2.abs() < s;
    }

    pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        return *v - 2.0 * (*v * *n).mul(*n);
    }

    pub fn mul(&self, other: &Vec3) -> Vec3{
        Vec3(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl Clone for Vec3 {
    fn clone(&self) -> Self {
        Vec3(self.0, self.1, self.2)
    }
}

impl Copy for Vec3 {}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3(self.0*rhs,self.1*rhs,self.2*rhs)
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(self*rhs.0,self*rhs.1,self*rhs.2)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = f32;

    fn mul(self, rhs: Vec3) -> Self::Output {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3(self.0/rhs,self.1/rhs,self.2/rhs)
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        return Vec3(self.0+rhs.0,self.1+rhs.1,self.2+rhs.2);
    }
}

impl Add<f32> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: f32) -> Self::Output {
        return Vec3(self.0+rhs,self.1+rhs,self.2+rhs);
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0-rhs.0,self.1-rhs.1,self.2-rhs.2)
    }
}
