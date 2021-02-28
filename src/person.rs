use bevy::{
    math::{vec2, vec3, Vec3Swizzles},
    prelude::*,
};
use rand::{prelude::SliceRandom, Rng};

use crate::{
    house::House,
    island::Island,
    resource::{NaturalResource, NaturalResourceType},
    Materials,
};

const PERSON_SPEED: f32 = 10.;

#[derive(Debug, PartialEq, Eq)]
enum PersonTask {
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
pub struct TargetPosition(Option<Vec2>);

#[derive(Debug)]
pub struct Person {
    name: String,
    task: PersonTask,
    house: Option<Entity>,
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

const AVAILABLE_PERSON_NAMES: [&str; 7] = [
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
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut TargetPosition, &mut Person, Entity), With<Person>>,
) {
    // For every person
    let delta = time.delta_seconds() * PERSON_SPEED;
    for (mut trans, mut optional_target, mut person, entity) in query.iter_mut() {
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
                person.task = PersonTask::Idle;
                // println!("{} {:?} has reached destination", person.name, entity);
                continue;
            }
            let dir = delta * dir.normalize();

            trans.translation += dir;
        }
    }
}

pub fn make_people_wander(
    time: Res<Time>,
    mut timer: ResMut<WanderTimer>,
    mut query: Query<(&mut TargetPosition, &mut Person), With<Person>>,
    query_houses: Query<&Transform, With<House>>,
) {
    // update our timer with the time elapsed since the last update
    // if the timer hasn't finished yet, we return
    if !timer.0.tick(time.delta_seconds()).just_finished() {
        return;
    }

    let mut rng = rand::thread_rng();

    for (mut target, mut person) in query.iter_mut() {
        if person.task != PersonTask::Idle {
            continue;
        }
        person.task = PersonTask::Wandering;
        let mut anchor: (f32, f32) = (0., 0.);
        // let home = person.house;
        if let Some(home) = person.house {
            if let Ok(aaa) = query_houses.get(home) {
                println!("{} has home, so will wander there", person.name);
                anchor = (aaa.translation.x, aaa.translation.y);
            }
        }
        let x: f32 = anchor.0 + 0.; //rng.gen_range(-100.0..=100.0);
        let y: f32 = anchor.1 + 0.; //rng.gen_range(-100.0..=100.0);
                                    // rng.gen_range(0..1);
                                    // println!("{} wants to go to {};{}!", person.1.name, x, y);
        *target = TargetPosition(Some(vec2(x, y)))
    }
}

pub fn make_people_gather(
    time: Res<Time>,
    mut timer: ResMut<WanderTimer>,
    mut person_q: Query<(&Transform, &mut Person, &mut TargetPosition)>,
    nr_q: Query<(&Transform, &NaturalResource, Entity)>,
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
        if rng.gen_ratio(60, 100) {
            continue;
        }
        for (nr_transform, nr, nr_entity) in nr_q.iter() {
            let mr_tr_v2 = vec2(nr_transform.translation.x, nr_transform.translation.y);
            if (person_tr_v2.x - mr_tr_v2.x).abs() + (person_tr_v2.y - mr_tr_v2.y).abs() < 100. {
                println!(
                    "{} wants to gather some {:?}",
                    per.name,
                    nr.0
                );
                per.task = PersonTask::Gathering(nr.0, nr_entity);
                *tar = TargetPosition(Some(mr_tr_v2));
                break;
            }
        }
    }
}

pub struct PeoplePlugin;
impl Plugin for PeoplePlugin {
    fn build(&self, app: &mut AppBuilder) {
        // the reason we call from_seconds with the true flag is to make the timer repeat itself
        app.add_resource(WanderTimer(Timer::from_seconds(2.0, true)))
            // .add_startup_system(add_people.system())
            .add_system(make_people_wander.system())
            .add_system(move_people.system())
            .add_system(make_people_gather.system());
        return;
    }
}
