use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

use bevy::{
    ecs::Command,
    math::{vec2, vec3, Vec3Swizzles},
    prelude::*,
    tasks::ParallelIterator,
};
use rand::{prelude::SliceRandom, Rng};

pub struct GatherEvent(pub Entity, pub NaturalResourceType);

use crate::{
    house::House,
    resource::{NaturalResource, NaturalResourceType},
    Materials, MyStages,
};

const PERSON_SPEED: f32 = 200.;

#[derive(Debug, PartialEq, Eq)]
pub enum PersonTask {
    Idle,
    Gathering(NaturalResourceType, Entity),
    Wandering,
}

impl Default for PersonTask {
    fn default() -> Self {
        Self::Idle
    }
}

pub struct WanderTimer(Timer);
pub struct GatherTimer(Timer);
pub struct TargetPosition(pub Option<Vec2>);

#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub task: PersonTask,
    pub house: Option<Entity>,
}

impl Default for Person {
    fn default() -> Self {
        Self {
            name: "Unnamed Pal".to_string(),
            task: PersonTask::default(),
            house: None,
        }
    }
}

pub const AVAILABLE_PERSON_NAMES: [&str; 7] = [
    "Leon", "Alina", "Elena", "Eduard", "Alexey", "Michael", "Vasya",
];

pub fn colonize_homes(
    commands: &mut Commands,
    materials: Res<Materials>,
    query: Query<(&Transform, Entity), With<House>>,
) {
    let mut rng = rand::thread_rng();
    for (house_transform, house_entity) in query.iter() {
        println!("Spawning person");
        commands
            .spawn(SpriteBundle {
                material: materials.skin.clone(),
                sprite: Sprite::new(Vec2::new(5.0, 11.0)),
                transform: Transform::from_translation(vec3(
                    house_transform.translation.x,
                    house_transform.translation.y,
                    10.,
                )),
                ..Default::default()
            })
            .with(Person {
                name: AVAILABLE_PERSON_NAMES.choose(&mut rng).unwrap().to_string(),
                house: Some(house_entity),
                ..Default::default()
            })
            .with(TargetPosition(None));
    }
}

pub fn move_people(
    commands: &mut Commands,
    time: Res<Time>,
    mut gather_event: ResMut<Events<GatherEvent>>,
    mut query: Query<(&mut Transform, &mut TargetPosition, &mut Person, Entity)>,
) {
    // For every person
    let delta = time.delta_seconds() * PERSON_SPEED;
    for (mut trans, mut optional_target, mut person, person_entity) in query.iter_mut() {
        // println!("Moving pal named: {:?}", person.1.name.clone());
        // If there is a place they want to go
        if let Some(target) = optional_target.0 {
            // MAKE THE NORMALIZED DIR VEC
            let dir = vec3(
                target.x - trans.translation.x,
                target.y - trans.translation.y,
                0.,
            );
            if dir.length_squared() < 4. {
                optional_target.0 = None;
                println!(
                    "{} {:?} has reached destination",
                    person.name, person_entity
                );
                match person.task {
                    PersonTask::Gathering(nr_type, nr_entity) => {
                        println!("{} gathered some {:?}", person.name, nr_type);
                        gather_event.send(GatherEvent(person_entity, nr_type));
                        commands.despawn_recursive(nr_entity);
                    }
                    _ => {}
                };

                person.task = PersonTask::Idle;

                continue;
            }
            let dir = delta * dir.normalize();

            trans.translation += dir;
        }
    }
}

fn handle_gather_events(
    events: Res<Events<GatherEvent>>,
    mut event_reader: Local<EventReader<GatherEvent>>,
    mut houses_query: Query<&mut Sprite, With<House>>,
    mut people_query: Query<&Person>,
) {
    for ev in event_reader.iter(&events) {
        let maybe_person = people_query.get(ev.0);
        if let Ok(person) = maybe_person {
            let maybe_house = person.house;
            if let Some(house) = maybe_house {
                let maybe_sprite = houses_query.get_mut(house);
                if let Ok(mut sprite) = maybe_sprite {
                    println!("Enlarging {}'s house!", person.name);
                    sprite.size.y += 10.;
                }
            }
        }
    }
}

pub fn make_people_wander(
    time: Res<Time>,
    mut timer: ResMut<WanderTimer>,
    mut query: Query<(&mut TargetPosition, &mut Person)>,
    query_houses: Query<&GlobalTransform, With<House>>,
) {
    // update our timer with the time elapsed since the last update
    // if the timer hasn't finished yet, we return
    if !timer.0.tick(time.delta_seconds()).just_finished() {
        return;
    }

    let mut rng = rand::thread_rng();

    for (mut target, mut person) in query.iter_mut() {
        match person.task {
            PersonTask::Idle => {}
            _ => continue,
        }
        person.task = PersonTask::Wandering;
        let mut anchor: (f32, f32) = (0., 0.);
        // let home = person.house;
        if let Some(home) = person.house {
            if let Ok(aaa) = query_houses.get(home) {
                println!(
                    "{} has home, so will wander there {}; {}",
                    person.name, aaa.translation.x as i32, aaa.translation.y as i32
                );
                anchor = (aaa.translation.x, aaa.translation.y);
            }
        }
        let x: f32 = anchor.0 + rng.gen_range(-100.0..=100.0);
        let y: f32 = anchor.1 + rng.gen_range(-100.0..=100.0);
        // rng.gen_range(0..1);
        // println!("{} wants to go to {};{}!", person.1.name, x, y);
        *target = TargetPosition(Some(vec2(x, y)))
    }
}

pub fn make_people_gather(
    time: Res<Time>,
    mut timer: ResMut<GatherTimer>,
    mut person_q: Query<(&Transform, &mut Person, &mut TargetPosition)>,
    nr_q: Query<(&Transform, &NaturalResource, Entity)>,
    pool: Res<bevy::tasks::ComputeTaskPool>,
) {
    if !timer.0.tick(time.delta_seconds()).just_finished() {
        return;
    }

    let mut rng = rand::thread_rng();

    for (person_t, mut per, mut tar) in person_q.iter_mut() {
        if !(per.task == PersonTask::Idle || per.task == PersonTask::Wandering) {
            continue;
        }
        let person_tr_v2 = vec2(person_t.translation.x, person_t.translation.y);
        if rng.gen_bool(0.5) {
            continue;
        }
        let min_dis = Arc::new(Mutex::new(f32::INFINITY)); // f32::INFINITY;
        let min_nr: Arc<Mutex<Option<(Entity, &NaturalResource, Vec2)>>> =
            Arc::new(Mutex::new(None)); // f32::INFINITY;

        nr_q.par_iter(32)
            .for_each(&pool, |(nr_transform, nr, nr_entity)| {
                let nr_pos = nr_transform.translation.xy();
                let dist = (person_tr_v2.x - nr_pos.x).abs() + (person_tr_v2.y - nr_pos.y).abs();
                let mut local_min = min_dis.lock().unwrap();
                if dist < *local_min {
                    *local_min = dist;
                    *min_nr.lock().unwrap() = Some((nr_entity, nr, nr_pos));
                }
            });

        let min_dis = *min_dis.lock().unwrap();
        let min_nr = *min_nr.lock().unwrap();

        if let Some(nearest_resource) = min_nr {
            let target_name = {
                let t = nr_q.get_component::<NaturalResource>(nearest_resource.0);
                if let Ok(t_r) = t {
                    format!("{:?}", *t_r)
                } else {
                    "Unknown target".to_string()
                }
            };
            println!("Found nearest dist {} to {}", min_dis, target_name);
            per.task = PersonTask::Gathering(nearest_resource.1 .0, nearest_resource.0);
            *tar = TargetPosition(Some(nearest_resource.2));
        }

        // for (nr_transform, nr, nr_entity) in nr_q.iter() {
        //     let mr_tr_v2 = vec2(nr_transform.translation.x, nr_transform.translation.y);
        //     if (person_tr_v2.x - mr_tr_v2.x).abs() + (person_tr_v2.y - mr_tr_v2.y).abs() < 200. {
        //         println!("{} wants to gather some {:?}", per.name, nr.0);
        //         per.task = PersonTask::Gathering(nr.0, nr_entity);
        //         *tar = TargetPosition(Some(mr_tr_v2));
        //         break;
        //     }
        // }
    }
}

pub struct PeoplePlugin;
impl Plugin for PeoplePlugin {
    fn build(&self, app: &mut AppBuilder) {
        // the reason we call from_seconds with the true flag is to make the timer repeat itself
        app.add_resource(WanderTimer(Timer::from_seconds(2.0, true)))
            .add_resource(GatherTimer(Timer::from_seconds(1.0, true)))
            .add_event::<GatherEvent>()
            // .add_startup_system(add_people.system())
            .add_startup_system_to_stage(MyStages::People.to_str(), colonize_homes.system())
            .add_system(make_people_wander.system())
            .add_system(move_people.system())
            .add_system(make_people_gather.system())
            .add_system(handle_gather_events.system());
    }
}
