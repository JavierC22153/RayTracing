
mod framebuffer;
mod ray_intersect;
mod color;
mod camera;
mod light;
mod material;
mod cube;
mod texture;
mod diorama;
mod square;

use minifb::{ Window, WindowOptions, Key };
use nalgebra_glm::{Vec3, normalize};
use std::time::Duration;
use std::f32::consts::PI;

use crate::color::Color;
use crate::ray_intersect::{Intersect, RayIntersect};
use crate::framebuffer::Framebuffer;
use crate::camera::Camera;
use crate::light::Light;

const ORIGIN_BIAS: f32 = 1e-4;
const DAY_SKY_COLOR: Color = Color::new(68, 142, 228);
const NIGHT_SKY_COLOR: Color = Color::new(25, 25, 112);

fn offset_origin(intersect: &Intersect, direction: &Vec3) -> Vec3 {
    let offset = intersect.normal * ORIGIN_BIAS;
    if direction.dot(&intersect.normal) < 0.0 {
        intersect.point - offset
    } else {
        intersect.point + offset
    }
}

fn reflect(incident: &Vec3, normal: &Vec3) -> Vec3 {
    incident - 2.0 * incident.dot(normal) * normal
}

fn refract(incident: &Vec3, normal: &Vec3, eta_t: f32) -> Vec3 {
    let cosi = -incident.dot(normal).max(-1.0).min(1.0);
    
    let (n_cosi, eta, n_normal);

    if cosi < 0.0 {
        n_cosi = -cosi;
        eta = 1.0 / eta_t;
        n_normal = -normal;
    } else {
        n_cosi = cosi;
        eta = eta_t;
        n_normal = *normal;
    }
    
    let k = 1.0 - eta * eta * (1.0 - n_cosi * n_cosi);
    
    if k < 0.0 {
        reflect(incident, &n_normal)
    } else {
        eta * incident + (eta * n_cosi - k.sqrt()) * n_normal
    }
}

fn cast_shadow(
    intersect: &Intersect,
    light: &Light,
    objects: &[Box<dyn RayIntersect>],
) -> f32 {
    let light_dir = (light.position - intersect.point).normalize();
    let light_distance = (light.position - intersect.point).magnitude();

    let shadow_ray_origin = offset_origin(intersect, &light_dir);
    let mut shadow_intensity = 0.0;

    for object in objects {
        let shadow_intersect = object.ray_intersect(&shadow_ray_origin, &light_dir);
        if shadow_intersect.is_intersecting && shadow_intersect.distance < light_distance {
            let distance_ratio = shadow_intersect.distance / light_distance;
            shadow_intensity = 1.0 - distance_ratio.powf(2.0).min(1.0);
            break;
        }
    }

    shadow_intensity
}

fn calculate_uv(normal: Vec3, point: Vec3, size: f32) -> (f32, f32) {
    let norm_point = point / size;

    let (u, v) = if normal.y.abs() > normal.x.abs() && normal.y.abs() > normal.z.abs() {
        // Cara superior o inferior
        (norm_point.x * 0.5 + 0.5, norm_point.z * 0.5 + 0.5)
    } else if normal.x.abs() > normal.y.abs() && normal.x.abs() > normal.z.abs() {
        // Cara lateral (x)
        (norm_point.z * 0.5 + 0.5, norm_point.y * 0.5 + 0.5)
    } else {
        // Cara lateral (z)
        (norm_point.x * 0.5 + 0.5, norm_point.y * 0.5 + 0.5)
    };

    (u.clamp(0.0, 1.0), v.clamp(0.0, 1.0))
}

pub fn cast_ray(
    ray_origin: &Vec3,
    ray_direction: &Vec3,
    objects: &[Box<dyn RayIntersect>],
    light: &Light,
    depth: u32,
    is_day: bool,
) -> Color {
    if depth > 3 {
        return if is_day { DAY_SKY_COLOR } else { NIGHT_SKY_COLOR };
    }

    let mut intersect = Intersect::empty();
    let mut zbuffer = f32::INFINITY;

    for object in objects {
        let i = object.ray_intersect(ray_origin, ray_direction);
        if i.is_intersecting && i.distance < zbuffer {
            zbuffer = i.distance;
            intersect = i;
        }
    }

    let is_daytime = is_day;
    if !intersect.is_intersecting {
        return if is_daytime { DAY_SKY_COLOR } else { NIGHT_SKY_COLOR };
    }
    


    let light_dir = (light.position - intersect.point).normalize();
    let view_dir = (ray_origin - intersect.point).normalize();
    let reflect_dir = reflect(&-light_dir, &intersect.normal).normalize();

    let shadow_intensity = cast_shadow(&intersect, light, objects);
    let light_intensity = light.intensity * (1.0 - shadow_intensity);

    const DEFAULT_CUBE_SIZE: f32 = 0.5; // Cambia esto por el tamaño que prefieras
    let uv = calculate_uv(intersect.normal, intersect.point, DEFAULT_CUBE_SIZE);


    let texture_diffuse = intersect.material.texture.as_ref().map_or(intersect.material.diffuse, |texture| {
        texture.sample(uv)
    });
    let diffuse_intensity = intersect.normal.dot(&light_dir).max(0.0).min(1.0);
    let diffuse = texture_diffuse * intersect.material.albedo[0] * diffuse_intensity * light_intensity;

    let specular_intensity = view_dir.dot(&reflect_dir).max(0.0).powf(intersect.material.specular);
    let specular = light.color * intersect.material.albedo[1] * specular_intensity * light_intensity;

    let mut reflect_color = Color::black();
    let reflectivity = intersect.material.albedo[2];
    if reflectivity > 0.0 {
        let reflect_dir = reflect(&ray_direction, &intersect.normal).normalize();
        let reflect_origin = offset_origin(&intersect, &reflect_dir);
        reflect_color = cast_ray(&reflect_origin, &reflect_dir, objects, light, depth + 1, is_day);
    }


    let mut refract_color = Color::black();
    let transparency = intersect.material.albedo[3];
    if transparency > 0.0 {
        let refract_dir = refract(&ray_direction, &intersect.normal, intersect.material.refractive_index);
        let refract_origin = offset_origin(&intersect, &refract_dir);
        refract_color = cast_ray(&refract_origin, &refract_dir, objects, light, depth + 1, is_day);
    }

    let emissive = intersect.material.emissive_color;

    (diffuse + specular) * (1.0 - reflectivity - transparency) + (reflect_color * reflectivity) + (refract_color * transparency) + emissive
}

pub fn render(framebuffer: &mut Framebuffer, objects: &[Box<dyn RayIntersect>], camera: &Camera, light: &Light, is_day: bool) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;
    let fov = PI/3.0;
    let perspective_scale = (fov * 0.5).tan();

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;

            let screen_x = screen_x * aspect_ratio * perspective_scale;
            let screen_y = screen_y * perspective_scale;

            let ray_direction = normalize(&Vec3::new(screen_x, screen_y, -1.0));

            let rotated_direction = camera.base_change(&ray_direction);

            let pixel_color = cast_ray(&camera.eye, &rotated_direction, objects, light, 0, is_day);

            framebuffer.set_current_color(pixel_color.to_hex());
            framebuffer.point(x, y);
        }
    }
}


fn main() {
    let window_width = 800;
    let window_height = 600;
    let framebuffer_width = 800;
    let framebuffer_height = 600;
    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Refractor",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    let objects = diorama::create_diorama();

    let mut camera = Camera::new(
        Vec3::new(-1.5, 2.0, 5.0), // Ajustar la posición de la cámara
        Vec3::new(-1.5, 2.0, 0.5), // Mirar hacia el centro del portal
        Vec3::new(0.0, 1.0, 0.0),  // Arriba
    );

    let mut light = Light::new(
        Vec3::new(1.0, 5.0, 5.0),
        Color::new(255, 255, 255),
        1.0
    );

    let rotation_speed = PI/10.0;

    let mut is_day = true;

    while window.is_open() && !window.is_key_down(Key::Escape) {

        if window.is_key_down(Key::Left) {
            camera.orbit(rotation_speed, 0.0); 
        }

        if window.is_key_down(Key::Right) {
            camera.orbit(-rotation_speed, 0.0);
        }

        if window.is_key_down(Key::Up) {
            camera.orbit(0.0, -rotation_speed);
        }

        if window.is_key_down(Key::Down) {
            camera.orbit(0.0, rotation_speed);
        }

        if window.is_key_down(Key::W) {
            camera.zoom(0.4); 
        }
    
        if window.is_key_down(Key::S) {
            camera.zoom(-0.4); 
        }


        if window.is_key_down(Key::T) {
            is_day = !is_day; 

        if is_day {
            light.position = Vec3::new(2.0, 5.0, 5.0); 
            light.color = Color::new(255, 255, 255); 
        } else {
            light.position = Vec3::new(-2.0, 5.0, 5.0); 
            light.color = Color::new(100, 100, 200); 
        }
    }
        render(&mut framebuffer, &objects.as_slice(), &camera, &light, is_day);

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}   
