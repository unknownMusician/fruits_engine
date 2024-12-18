mod test_json;

use fruits_prelude::*;

fn main() {
    //test_json();
    test_fruits_prelude();
}

#[derive(Resource)]
pub struct SomeResource;

fn test_fruits_prelude() {
    let mut app = App::new();

    let ecs = app.ecs_mut();
    let data = ecs.data_mut();
    
    data.resources_mut().insert(SomeResource);
    data.entities_components_mut().create_entity();
    
    let behavior = ecs.behavior_mut();

    behavior.get_mut(Schedule::Start).add_system(some_init_system);
    behavior.get_mut(Schedule::Update).add_system(some_early_system);
    behavior.get_mut(Schedule::Update).add_system(some_late_system);
    behavior.get_mut(Schedule::Update).order_systems(some_early_system, some_late_system);

    app.run();
}

fn some_init_system() {

}

fn some_early_system() {

}

fn some_late_system() {

}
