use crate::*;
use bevy::{
    math::{vec2, vec3},
    prelude::*,
};
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};
use rand::prelude::SliceRandom;

pub struct IslandsResources {
    pub available_names: Vec<String>,
}

pub struct Title(pub String);

#[derive(Debug)]
pub struct Hovered(pub bool);

pub struct Size {
    pub width: f32,
    pub height: f32,
}

pub struct Bbox;

pub struct Soil;
pub struct Grass;

pub struct Island;

#[derive(Bundle)]
pub struct IslandBundle {
    pub position: Vec2,
    pub title: Title,
    pub size: Size,
    pub _i: Island,
    // soil: ShapeBundle,
    // grass: ShapeBundle,
}

pub fn spawn_island_at(
    commands: &mut Commands,
    materials: &Res<Materials>,
    island_mat: &Res<IslandsResources>,
    translation: Vec3,
) {
    println!("Spawning island");

    let mut rng = rand::thread_rng();

    let width: f32 = (translation.x.cos() * 150.) + 50.;
    let triangle = shapes::Circle {
        radius: width / 2.,
        center: Vec2::zero(),
    };

    let bbox = shapes::Rectangle {
        width: width,
        height: width,
        origin: shapes::RectangleOrigin::Center,
    };
    let new_name = island_mat
        .available_names
        .choose(&mut rng)
        .unwrap()
        .to_string();

    commands
        .spawn(IslandBundle {
            position: vec2(0., 0.),
            title: Title(new_name),
            size: Size {
                width: width,
                height: 10.,
            },
            _i: Island,
        })
        .with(Transform::from_translation(translation))
        .with(GlobalTransform::default())
        .with_children(|parent| {
            parent
                .spawn(GeometryBuilder::build_as(
                    &triangle,
                    materials.soil_material.clone(),
                    TessellationMode::Fill(FillOptions::default()),
                    Transform::default(),
                ))
                .with(GlobalTransform::default())
                .spawn(GeometryBuilder::build_as(
                    &bbox,
                    materials.transparent.clone(),
                    TessellationMode::Stroke(StrokeOptions::default()),
                    Transform::default(),
                ))
                .with(Bbox)
                .with(Hovered(false))
                .with(GlobalTransform::default());
        });
}

pub fn spawn_islands(
    commands: &mut Commands,
    materials: Res<Materials>,
    island_mat: Res<IslandsResources>,
) {
    spawn_island_at(commands, &materials, &island_mat, vec3(0., 0., 0.));

    for i in 0..1 {
        let x: f32 = 500. * (i as f32).cos();
        let y: f32 = 500. * (i as f32).sin();
        spawn_island_at(commands, &materials, &island_mat, vec3(x, y, 0.));
    }
}

pub struct IslandsPlugin;
impl Plugin for IslandsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(IslandsResources {
            available_names: vec!["Land #1".to_string(), "Ocor".to_string()],
        })
        .add_startup_system_to_stage(MyStages::Islands.to_str(), spawn_islands.system())
        .add_system(draw_hovered_islands.system());
    }
}
