use std::sync::Arc;

use crate::{perlin, vec3::Vec3};

pub type Texture = Arc<dyn Fn(Vec3) -> Vec3 + Send + Sync>;

pub fn constant(color: Vec3) -> Texture {
    Arc::new(move |_| color)
}

pub fn checker(t0: Texture, t1: Texture) -> Texture {
    Arc::new(move |p| {
        let s = (10. * p).map(f32::sin).reduce(std::ops::Mul::mul);
        if s < 0. {
            t1(p)
        } else {
            t0(p)
        }
    })
}

pub fn perlin(scale: f32) -> Texture {
    Arc::new(move |p| Vec3::from(perlin::turb(scale * p, 7)))
}
