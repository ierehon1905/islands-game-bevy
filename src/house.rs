use crate::*;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use rand::{self, Rng};

pub struct City;

pub fn build_cities(
    commands: &mut Commands,
    materials: Res<Materials>,
    query: Query<Entity, With<Island>>,
) {
    println!("Trying to build cities");
    
    for t in query.iter() {
        println!("Building a city");
        let city = commands
            .spawn((City, Transform::default(), GlobalTransform::default()))
            .current_entity()
            .unwrap();
        commands.push_children(t, &[city]);
    }
}

pub struct House;

pub fn build_house(
    commands: &mut Commands,
    materials: Res<Materials>,
    query: Query<Entity, With<City>>,
) {
    let mut rng = rand::thread_rng();
    for t in query.iter() {
        let number_of_houses: usize = rng.gen_range(1..10);
        let mut houses: Vec<Entity> = Vec::with_capacity(number_of_houses);
        // houses.capacity()
        for i in 0..number_of_houses {
            let x = (i as f32) * (200. / number_of_houses as f32);
            println!("Building house");
            let house = commands
                .spawn(SpriteBundle {
                    material: materials.house_material.clone(),
                    sprite: Sprite::new(Vec2::new(10.0, 10.0)),
                    transform: Transform::from_translation(vec3(x, 0., 0.)),
                    ..Default::default()
                })
                .with(House)
                .current_entity()
                .unwrap();
            houses.push(house);
        }
        commands.push_children(t, houses.as_slice());
    }
}
