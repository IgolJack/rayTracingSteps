use std::fmt::{Display, Formatter, Result};
use crate::math::vec3::Vec3;

#[derive(Clone, Copy)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Display for Ray {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f, "Ray(Origin({}), Direction({}))", self.origin, self.direction,
        )
    }
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {origin, direction}
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn set_origin(&mut self, origin: Vec3) {
        self.origin = origin;
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn set_direction(&mut self, direction: Vec3) {
        self.direction = direction;
    }

    // get_point
    pub fn at(&self, t: f64) -> Vec3 { 
        self.origin + self.direction * t
    }
}
