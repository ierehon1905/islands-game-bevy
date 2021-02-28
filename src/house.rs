use crate::{
    island::{Island, Size},
    Materials,
};
use bevy::{math::vec3, prelude::*};
use rand::{self, Rng};

pub struct House;

pub fn build_house(
    commands: &mut Commands,
    materials: Res<Materials>,
    query: Query<(Entity, &Size), With<Island>>,
) {
    let mut rng = rand::thread_rng();
    for (island_entity, island_size) in query.iter() {
        let number_of_houses: usize = rng.gen_range(1..10);
        let mut houses: Vec<Entity> = Vec::with_capacity(number_of_houses);
        let radius = island_size.width / 2.;

        for i in 0..number_of_houses {
            let angle = i as f32 * std::f32::consts::TAU / number_of_houses as f32;
            let x = angle.cos() * radius;
            let y = angle.sin() * radius;
            println!("Building house");
            let house = commands
                .spawn(SpriteBundle {
                    material: materials.house_material.clone(),
                    sprite: Sprite::new(Vec2::new(10.0, 10.0)),
                    transform: Transform::from_translation(vec3(x, y, 0.)),
                    ..Default::default()
                })
                .with(House)
                .current_entity()
                .unwrap();
            houses.push(house);
        }
        commands.push_children(island_entity, houses.as_slice());
    }
}
