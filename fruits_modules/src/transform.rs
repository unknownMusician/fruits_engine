mod components;
mod systems;

pub use self::{
    components::*,
    systems::*,
};

use fruits_ecs_schedule::Schedule;
use fruits_ecs_world::WorldBuilder;

pub fn add_module_to(world: &mut WorldBuilder) {
    let update = world.behavior_mut().get_mut(Schedule::Update);

    update.add_system(adjust_component_sets);
    update.add_system(update_parents_remove_invalid_children);
    update.add_system(update_parents_add_missing_children);
    update.add_system(update_parents_destroy_empty_parents);
    update.add_system(calculate_global_transform);
    
    update.order_systems(adjust_component_sets, update_parents_remove_invalid_children);
    update.order_systems(update_parents_remove_invalid_children, update_parents_add_missing_children);
    update.order_systems(update_parents_add_missing_children, update_parents_destroy_empty_parents);
    update.order_systems(update_parents_destroy_empty_parents, calculate_global_transform);
}