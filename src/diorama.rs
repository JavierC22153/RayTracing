use nalgebra_glm::Vec3;
use std::rc::Rc;
use crate::cube::Cube;
use crate::material::Material;
use crate::texture::Texture;
use crate::color::Color; // Asegúrate de importar Color
use crate::ray_intersect::RayIntersect;
use crate::square::Square; 

pub fn create_diorama() -> Vec<Box<dyn RayIntersect>> {
    let obsidian_texture = Rc::new(Texture::new("assets/obsidian.png"));
    let stone_texture = Rc::new(Texture::new("assets/stone.png"));
    let netherrack_texture = Rc::new(Texture::new("assets/netherrack.png"));
    let gold_block_texture = Rc::new(Texture::new("assets/gold_block.png"));
    let stone_bricks_texture = Rc::new(Texture::new("assets/stone_bricks.png"));
    let chiseled_stone_bricks_texture = Rc::new(Texture::new("assets/chiseled_stone_bricks.png"));

    // Definir materiales como Rc<Material>
    let obsidian = Rc::new(Material::new(
        Color::new(30, 30, 30), 
        80.0, 
        [0.6, 0.4, 0.3, 0.0], 
        0.0, 
        Some(obsidian_texture), 
        Color::black(),
    ));

    let stone = Rc::new(Material::new(
        Color::new(128, 128, 128),
        0.2,
        [0.7, 0.7, 0.7, 0.0],
        0.0,
        Some(stone_texture.clone()), 
        Color::black(),
    ));

    let netherrack = Rc::new(Material::new(
        Color::new(150, 0, 0), 
        70.0, 
        [0.7, 0.2, 0.1, 0.0], 
        0.0, 
        Some(netherrack_texture), 
        Color::black(),
    ));

    let gold_block = Rc::new(Material::new(
        Color::new(255, 215, 0),
        1.0,
        [1.0, 0.843, 0.0, 0.0],
        0.0,
        Some(gold_block_texture), 
        Color::black(),
    ));

    let stone_bricks = Rc::new(Material::new(
        Color::new(180, 180, 180), 
        90.0, 
        [0.9, 0.5, 0.3, 0.0], 
        0.0, 
        Some(stone_bricks_texture), 
        Color::black(),
    ));

    let chiseled_stone_bricks = Rc::new(Material::new(
        Color::new(220, 220, 220), 
        85.0, 
        [0.8, 0.6, 0.4, 0.0], 
        0.0, 
        Some(chiseled_stone_bricks_texture), 
        Color::black(),
    ));

    let purple = Rc::new(Material::new(
        Color::new(160, 0, 190), 
        80.0, 
        [0.7, 0.5, 0.6, 0.0], 
        0.0, 
        None, 
        Color::new(75,0,90),
    ));

    let grass = Rc::new(Material::new(
        Color::new(0, 255, 0), 
        0.1, 
        [0.4, 0.6, 0.5, 0.0], 
        0.0, 
        None, 
        Color::black(),
    ));


       let cube_data = [
       
        // Portal
        (Vec3::new(-2.2, 2.4, -5.0), Rc::clone(&purple)),
        (Vec3::new(-1.8, 2.4, -5.0), Rc::clone(&purple)),
        (Vec3::new(-2.2, 2.8, -5.0), Rc::clone(&purple)),
        (Vec3::new(-1.8, 2.8, -5.0), Rc::clone(&purple)),
        (Vec3::new(-2.2, 3.2, -5.0), Rc::clone(&purple)),
        (Vec3::new(-1.8, 3.2, -5.0), Rc::clone(&purple)),
        //obsidiana

        (Vec3::new(-1.4, 2.0, -4.6), Rc::clone(&obsidian)), 
        (Vec3::new(-1.4, 2.4, -4.6), Rc::clone(&obsidian)), 
        (Vec3::new(-1.4, 2.8, -4.6), Rc::clone(&obsidian)), 
        (Vec3::new(-1.4, 3.2, -4.6), Rc::clone(&obsidian)),     
        (Vec3::new(-1.4, 3.6, -4.6), Rc::clone(&obsidian)), 

        (Vec3::new(-1.8, 2.0, -4.6), Rc::clone(&obsidian)),
        (Vec3::new(-2.2, 2.0, -4.6), Rc::clone(&obsidian)),

        (Vec3::new(-2.6, 2.0, -4.6), Rc::clone(&obsidian)),
        (Vec3::new(-2.6, 2.4, -4.6), Rc::clone(&obsidian)),
        (Vec3::new(-2.6, 2.8, -4.6), Rc::clone(&obsidian)),
        (Vec3::new(-2.6, 3.2, -4.6), Rc::clone(&obsidian)),
        (Vec3::new(-2.6, 3.6, -4.6), Rc::clone(&obsidian)),

        (Vec3::new(-1.8, 3.6, -4.6), Rc::clone(&obsidian)),
        (Vec3::new(-2.2, 3.6, -4.6), Rc::clone(&obsidian)),
       

        //superficie
        (Vec3::new(-0.6, 2.0, -4.2), Rc::clone(&gold_block)),
        (Vec3::new(-0.6, 1.6, -4.2), Rc::clone(&netherrack)),
        (Vec3::new(-1.0, 1.6, -4.2), Rc::clone(&stone)),
        (Vec3::new(-1.4, 1.6, -3.8), Rc::clone(&stone)),
        (Vec3::new(-1.8, 1.6, -3.8), Rc::clone(&stone)),
        (Vec3::new(-2.2, 1.6, -3.8), Rc::clone(&stone)),
        (Vec3::new(-2.6, 1.6, -3.8), Rc::clone(&stone)),
        (Vec3::new(-3.0, 1.6, -4.2), Rc::clone(&stone)),
        (Vec3::new(-3.4, 1.6, -4.2), Rc::clone(&netherrack)),
        (Vec3::new(-3.4, 2.0, -4.2), Rc::clone(&gold_block)),
        
        //siguiente nivel
        (Vec3::new(-0.6, 0.8, -3.8), Rc::clone(&netherrack)),
        (Vec3::new(-0.6, 1.2, -3.8), Rc::clone(&netherrack)),
        (Vec3::new(-1.0, 1.2, -3.8), Rc::clone(&stone)),
        (Vec3::new(-1.4, 1.2, -3.4), Rc::clone(&stone)),
        (Vec3::new(-1.8, 1.2, -3.4), Rc::clone(&stone)),
        (Vec3::new(-2.2, 1.2, -3.4), Rc::clone(&stone)),
        (Vec3::new(-2.6, 1.2, -3.4), Rc::clone(&stone)),
        (Vec3::new(-3.0, 1.2, -3.8), Rc::clone(&stone)),
        (Vec3::new(-3.4, 1.2, -3.8), Rc::clone(&netherrack)),
        (Vec3::new(-3.4, 0.8, -3.8), Rc::clone(&netherrack)),
        

        //rodea el portal
        (Vec3::new(-1.0, 2.0, -4.6), Rc::clone(&stone_bricks)),
        (Vec3::new(-1.0, 2.4, -4.6), Rc::clone(&chiseled_stone_bricks)),
        (Vec3::new(-1.0, 2.8, -4.6), Rc::clone(&stone_bricks)),
        (Vec3::new(-1.0, 3.2, -4.6), Rc::clone(&chiseled_stone_bricks)),
        (Vec3::new(-1.0, 3.6, -4.6), Rc::clone(&stone_bricks)),
        (Vec3::new(-1.0, 4.0, -4.6), Rc::clone(&stone_bricks)),

        (Vec3::new(-3.0, 2.0, -4.6), Rc::clone(&stone_bricks)),
        (Vec3::new(-3.0, 2.4, -4.6), Rc::clone(&chiseled_stone_bricks)),
        (Vec3::new(-3.0, 2.8, -4.6), Rc::clone(&stone_bricks)),
        (Vec3::new(-3.0, 3.2, -4.6), Rc::clone(&chiseled_stone_bricks)),
        (Vec3::new(-3.0, 3.6, -4.6), Rc::clone(&stone_bricks)),
        (Vec3::new(-3.0, 4.0, -4.6), Rc::clone(&stone_bricks)),

        (Vec3::new(-2.6, 4.0, -4.6), Rc::clone(&chiseled_stone_bricks)),
        (Vec3::new(-2.2, 4.0, -4.6), Rc::clone(&gold_block)),
        (Vec3::new(-1.8, 4.0, -4.6), Rc::clone(&gold_block)),
        (Vec3::new(-1.4, 4.0, -4.6), Rc::clone(&chiseled_stone_bricks)),

        (Vec3::new(-2.2, 4.2, -4.4), Rc::clone(&chiseled_stone_bricks)),
        (Vec3::new(-1.8, 4.2, -4.4), Rc::clone(&chiseled_stone_bricks)),

        //suelo
        (Vec3::new(-1.0, 0.8, -3.4), Rc::clone(&netherrack)),
        (Vec3::new(-1.4, 0.8, -3.4), Rc::clone(&netherrack)),
        (Vec3::new(-1.8, 0.8, -3.4), Rc::clone(&netherrack)),
        (Vec3::new(-2.2, 0.8, -3.4), Rc::clone(&netherrack)),
        (Vec3::new(-2.6, 0.8, -3.4), Rc::clone(&netherrack)),
        (Vec3::new(-3.0, 0.8, -3.4), Rc::clone(&netherrack)),
    ];

    let mut objects: Vec<Box<dyn RayIntersect>> = Vec::new();

    for &(center, ref material) in &cube_data {
        let cube = Box::new(Cube {
            center,
            size: 0.4,
            material: Rc::clone(material),
        }) as Box<dyn RayIntersect>;
        objects.push(cube);
    }

    // Crear el suelo
    let ground = Box::new(Square {
        center: Vec3::new(-2.0, 0.7, -4.0), // Ajusta la posición del suelo según tu escena
        size: 10.0, // Ajusta el tamaño del suelo según tu escena
        material: Rc::clone(&grass),
    }) as Box<dyn RayIntersect>;

    objects.push(ground);
    

    objects
}