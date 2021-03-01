use std::iter::Enumerate;

use bevy::{
    ecs::WorldBuilder,
    math::{vec2, vec3},
    prelude::*,
};
use rand::Rng;

use crate::MyStages;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum NaturalResourceType {
    Coal,
    Iron,
    Gold,
    Wood,
    Water,
}

impl NaturalResourceType {
    pub fn choose<R>(rng: &mut R) -> Self
    where
        R: Rng + ?Sized,
    {
        let i: usize = rng.gen_range(0..5);
        match i {
            0 => Self::Coal,
            1 => Self::Iron,
            2 => Self::Gold,
            3 => Self::Wood,
            4 => Self::Wood,
            _ => {
                panic!("Unexpected random resource");
            }
        }
    }
}

pub struct NaturalResourceMaterials {
    pub coal: Handle<ColorMaterial>,
    pub iron: Handle<ColorMaterial>,
    pub gold: Handle<ColorMaterial>,
    pub wood: Handle<ColorMaterial>,
    pub water: Handle<ColorMaterial>,
}

#[derive(Debug, Clone, Copy)]
pub struct NaturalResource(pub NaturalResourceType);
// pub struct

pub fn make_resource_materials(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("Creating materials for resourcess");
    commands.insert_resource(NaturalResourceMaterials {
        coal: materials.add(ColorMaterial::color(Color::BLACK.into())),
        iron: materials.add(ColorMaterial::color(Color::SILVER.into())),
        gold: materials.add(ColorMaterial::color(Color::rgb_u8(255, 215, 0).into())),
        wood: materials.add(ColorMaterial::color(Color::rgb_u8(150, 70, 0).into())),
        water: materials.add(ColorMaterial::color(Color::CYAN.into())),
    });
}

pub fn plant_resources(commands: &mut Commands, mats: Res<NaturalResourceMaterials>) {
    let mut rng = rand::thread_rng();
    for x in -1000..=1000 {
        for y in -1000..=1000 {
            if rng.gen_ratio(1, 100 * 100) {
                // mats.
                let r = NaturalResourceType::choose(&mut rng);
                let mat = match r {
                    NaturalResourceType::Coal => mats.coal.clone(),
                    NaturalResourceType::Iron => mats.iron.clone(),
                    NaturalResourceType::Gold => mats.gold.clone(),
                    NaturalResourceType::Wood => mats.wood.clone(),
                    NaturalResourceType::Water => mats.water.clone(),
                };
                commands
                    .spawn(SpriteBundle {
                        sprite: Sprite::new(vec2(10., 10.)),
                        material: mat,
                        transform: Transform::from_translation(vec3(x as f32, y as f32, 100.)),
                        ..Default::default()
                    })
                    .with(NaturalResource(r));
            }
        }
    }
}
pub struct ResourcesPlugin;
impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_stage_after(
            MyStages::PreSetup.to_str(),
            "resources",
            SystemStage::single(make_resource_materials.system()),
        )
        .add_startup_stage_after("resources", "planting_resources", SystemStage::parallel())
        .add_startup_system_to_stage("planting_resources", plant_resources.system());
    }
}
