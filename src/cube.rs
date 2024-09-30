use nalgebra_glm::Vec3;
use crate::ray_intersect::{RayIntersect, Intersect};
use crate::material::Material;
use std::rc::Rc;

#[derive(Debug)]
pub struct Cube {
    pub center: Vec3,
    pub size: f32,
    pub material: Rc<Material>, 
}

impl RayIntersect for Cube {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Intersect {
        let half_size = self.size / 2.0;
        let min = self.center - Vec3::new(half_size, half_size, half_size);
        let max = self.center + Vec3::new(half_size, half_size, half_size);

        let mut t_min = (min.x - ray_origin.x) / ray_direction.x;
        let mut t_max = (max.x - ray_origin.x) / ray_direction.x;

        if t_min > t_max {
            std::mem::swap(&mut t_min, &mut t_max);
        }

        let mut ty_min = (min.y - ray_origin.y) / ray_direction.y;
        let mut ty_max = (max.y - ray_origin.y) / ray_direction.y;

        if ty_min > ty_max {
            std::mem::swap(&mut ty_min, &mut ty_max);
        }

        if t_min > ty_max || ty_min > t_max {
            return Intersect::empty();
        }

        if ty_min > t_min {
            t_min = ty_min;
        }

        if ty_max < t_max {
            t_max = ty_max;
        }

        let mut tz_min = (min.z - ray_origin.z) / ray_direction.z;
        let mut tz_max = (max.z - ray_origin.z) / ray_direction.z;

        if tz_min > tz_max {
            std::mem::swap(&mut tz_min, &mut tz_max);
        }

        if t_min > tz_max || tz_min > t_max {
            return Intersect::empty();
        }

        if tz_min > t_min {
            t_min = tz_min;
        }

        if tz_max < t_max {
            t_max = tz_max;
        }

        if t_min < 0.0 {
            return Intersect::empty();
        }

        let point = ray_origin + ray_direction * t_min;
        let mut normal = Vec3::zeros();

        // Determinación de la normal
        if (point.x - min.x).abs() < 1e-4 {
            normal = Vec3::new(-1.0, 0.0, 0.0);
        } else if (point.x - max.x).abs() < 1e-4 {
            normal = Vec3::new(1.0, 0.0, 0.0);
        } else if (point.y - min.y).abs() < 1e-4 {
            normal = Vec3::new(0.0, -1.0, 0.0);
        } else if (point.y - max.y).abs() < 1e-4 {
            normal = Vec3::new(0.0, 1.0, 0.0);
        } else if (point.z - min.z).abs() < 1e-4 {
            normal = Vec3::new(0.0, 0.0, -1.0);
        } else if (point.z - max.z).abs() < 1e-4 {
            normal = Vec3::new(0.0, 0.0, 1.0);
        }

        // Retornar la intersección
        Intersect::new(point, normal, t_min, self.material.clone())
    }
}
