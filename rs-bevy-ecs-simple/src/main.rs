use bevy::{
    app::{App, Startup, Update},
    ecs::{
        component::Component,
        entity::Entity,
        query::With,
        schedule::IntoScheduleConfigs,
        system::{Commands, Query},
    },
};

#[derive(Component)]
struct EntityName(String);

#[derive(Component)]
struct Health(f32);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

fn spawn_entities(mut commands: Commands) {
    commands.spawn((EntityName("Player".to_string()), Health(100.0), Player));

    commands.spawn((EntityName("Goblin".to_string()), Health(60.0), Enemy));

    commands.spawn((EntityName("Skeleton".to_string()), Health(40.0), Enemy));

    commands.spawn((EntityName("Training Dummy".to_string()), Health(200.0)));
}

fn print_all_named_entities(query: Query<(Entity, &EntityName, &Health)>) {
    println!("All entities:");

    for (entity, name, health) in &query {
        println!("{:?} - {} (Health: {:.0})", entity, name.0, health.0);
    }
}

fn print_enemies(query: Query<&EntityName, With<Enemy>>) {
    println!("Enemies:");

    for name in &query {
        println!("{}", name.0);
    }
}

fn main() {
    App::new()
        .add_systems(Startup, spawn_entities)
        // .chain() makes execution order deterministic
        .add_systems(Update, (print_all_named_entities, print_enemies).chain())
        .update(); // Using instead of run because we only want 1 execution
}
