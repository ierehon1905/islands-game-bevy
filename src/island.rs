use crate::*;
use bevy::{
    math::{vec2, vec3},
    prelude::*,
};
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};

pub struct Title(String);

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

pub fn spawn_island_at(commands: &mut Commands, materials: &Res<Materials>, translation: Vec3) {
    println!("Spawning island");
    let width: f32 = (translation.x.cos() * 150.) + 50.;
    // let triangle = shapes::Polygon {
    //     points: vec![vec2(-width / 2., 0.), vec2(width / 2., 0.), vec2(0., -100.)],
    //     closed: true,
    // };

    let triangle = shapes::Circle {
        radius: width / 2.,
        center: Vec2::zero(),
    };

    let bbox = shapes::Rectangle {
        width: width,
        height: width,
        origin: shapes::RectangleOrigin::Center,
    };

    commands
        .spawn(IslandBundle {
            position: vec2(0., 0.),
            title: Title("Land".to_string()),
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

pub fn spawn_islands(commands: &mut Commands, materials: Res<Materials>) {
    spawn_island_at(commands, &materials, vec3(0., 0., 0.));

    for i in 0..15 {
        let x: f32 = 500. * (i as f32).cos();
        let y: f32 = 500. * (i as f32).sin();
        spawn_island_at(commands, &materials, vec3(x, y, 0.));
    }
}
