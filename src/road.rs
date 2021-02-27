use crate::*;
use bevy::{
    math::{vec2, vec3},
    prelude::*,
};
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};

struct Road {
    pub from: Entity,
    pub to: Entity,
}

fn create_road(
    commands: &mut Commands,
    materials: Res<Materials>,
    query: QuerySet<(Query<&Road>, Query<(Entity, &Transform), With<Island>>)>,
) {
    let current_roads = query.q0().iter();

    for pair in query.q1().iter().collect::<Vec<_>>().chunks(2) {
        println!("Spawning road");

        let path = shapes::Line(pair[0].1.translation.into(), pair[1].1.translation.into());

        // if let Some(ex) = current_roads.find(|&&r| {
        //     r.from == pair[0] || r.to == pair[0] || r.to == pair[1] || r.from == pair[1]
        // })
        // {
        commands
            .spawn((Road {
                from: pair[0].0,
                to: pair[1].0,
            },))
            .with_children(|p| {
                p.spawn(GeometryBuilder::build_as(
                    &path,
                    materials.soil_material.clone(),
                    TessellationMode::Stroke(StrokeOptions::default()),
                    Transform::default(),
                ))
                .with(GlobalTransform::default());
            });
        // } else {
        // }
    }
}
