use crate::{
    draw_hovered_islands, house::House, person::*, resource::NaturalResourceType, Materials,
    MyStages,
};
use bevy::{
    ecs::Stage,
    math::{vec2, vec3},
    prelude::*,
    utils::HashMap,
};
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};
use rand::{prelude::SliceRandom, Rng};

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

#[derive(Debug, Default)]
pub struct IslandNR(pub HashMap<NaturalResourceType, u32>);

#[derive(Bundle)]
pub struct IslandBundle {
    pub title: Title,
    pub size: Size,
    pub _i: Island,
    pub natural_resources: IslandNR,
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

    let width: f32 = (translation.x.cos().abs() * 150.) + 50.;
    let triangle = shapes::Circle {
        radius: width / 2.,
        center: Vec2::zero(),
    };

    let bbox = shapes::Rectangle {
        width,
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
            title: Title(new_name),
            size: Size {
                width,
                height: width,
            },
            _i: Island,
            natural_resources: Default::default(),
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

fn handle_gather_events(
    events: Res<Events<GatherEvent>>,
    mut event_reader: Local<EventReader<GatherEvent>>,
    houses_query: Query<&House>,
    people_query: Query<&Person>,
    mut islands_query: Query<&mut IslandNR>,
) {
    for ev in event_reader.iter(&events) {
        let maybe_person = people_query.get(ev.0);
        if let Ok(person) = maybe_person {
            let maybe_house = person.house;
            if let Some(house) = maybe_house {
                let island_entity = houses_query.get(house).unwrap().island;
                // let isl /z= islands_query.get(h)
                let mut nr_isl = islands_query.get_mut(island_entity).unwrap();
                // nr_isl.entry(ev.1).or_insert(default);
                let count = nr_isl.0.entry(ev.1).or_insert(0);
                *count += 1;
                println!("Inlands resources {:?}", &nr_isl.0);
            }
        }
    }
}

fn handle_resources_changes(
    commands: &mut Commands,
    materials: Res<Materials>,
    mut query: Query<(&mut IslandNR, &Transform, &Size, Entity), Mutated<IslandNR>>,
) {
    let mut rng = rand::thread_rng();
    for (mut res, tr, size, en) in query.iter_mut() {
        println!("Building a new house");
        if let Some(mut wood) = res.0.get_mut(&NaturalResourceType::Wood) {
            while *wood >= 2 {
                let x = tr.translation.x + rng.gen_range(-size.width / 2.0..size.width / 2.0);
                let y = tr.translation.y + rng.gen_range(-size.height / 2.0..size.height / 2.0);
                let house = commands
                    .spawn(SpriteBundle {
                        material: materials.house_material.clone(),
                        sprite: Sprite::new(Vec2::new(10.0, 10.0)),
                        transform: Transform::from_translation(vec3(x, y, 0.)),
                        ..Default::default()
                    })
                    .with(House { island: en })
                    .current_entity()
                    .unwrap();

                commands
                    .spawn(SpriteBundle {
                        material: materials.skin.clone(),
                        sprite: Sprite::new(Vec2::new(5.0, 11.0)),
                        transform: Transform::from_translation(vec3(x, y, 10.)),
                        ..Default::default()
                    })
                    .with(Person {
                        name: AVAILABLE_PERSON_NAMES.choose(&mut rng).unwrap().to_string(),
                        house: Some(house),
                        ..Default::default()
                    })
                    .with(TargetPosition(None));

                *wood -= 2;
            }
        }
    }
}

pub struct IslandsPlugin;
impl Plugin for IslandsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(IslandsResources {
            available_names: vec!["Land #1".to_string(), "Ocor".to_string()],
        })
        .add_startup_system_to_stage(MyStages::Islands.to_str(), spawn_islands.system())
        .add_system(draw_hovered_islands.system())
        .add_stage_after(stage::UPDATE, "changes", SystemStage::parallel())
        .add_system_to_stage("changes", handle_resources_changes.system())
        .add_system(handle_gather_events.system());
    }
}
