use std::collections::VecDeque;

use fruits_ecs_component::Entity;
use fruits_ecs_system_params::{ExclusiveWorldAccess, WorldQuery};

use super::{ChildComponent, GlobalTransform, LocalTransform, ParentComponent};

pub fn adjust_component_sets(
    mut world: ExclusiveWorldAccess,
) {
    #[derive(Clone, Copy)]
    enum Change {
        ParentAdd,
        ParentRemove,
        ChildAdd,
        ChildRemove,
        GlobalAdd,
    }

    let mut changes = Vec::new();

    let entities_components = world.entities_components();
    for e in entities_components.query::<Entity>().iter() {
        // +-Parent +-Child +Global

        // - -> -Parent -Child
        // Local -> +Global +Child +Parent
        // Global -> +Parent -Child
        // Local, Global -> +Child +Parent
        let contains_global = entities_components.get_component::<GlobalTransform>(e).is_some();
        let contains_local = entities_components.get_component::<LocalTransform>(e).is_some();
        let contains_child = entities_components.get_component::<ChildComponent>(e).is_some();
        let contains_parent = entities_components.get_component::<ParentComponent>(e).is_some();

        match (contains_global, contains_local) {
            (false, false) => {
                if contains_parent { changes.push((e, Change::ParentRemove)) };
                if contains_child { changes.push((e, Change::ChildRemove)) };
            },
            (false, true) => {
                changes.push((e, Change::GlobalAdd));
                if !contains_parent { changes.push((e, Change::ParentAdd)) };
                if !contains_child { changes.push((e, Change::ChildAdd)) };
            },
            (true, false) => {
                if !contains_parent { changes.push((e, Change::ParentAdd)) };
                if contains_child { changes.push((e, Change::ChildRemove)) };
            },
            (true, true) => {
                if !contains_parent { changes.push((e, Change::ParentAdd)) };
                if !contains_child { changes.push((e, Change::ChildAdd)) };
            },
        }
    }

    let entities_components = world.entities_components_mut();
    for (e, change) in changes {
        match change {
            Change::ChildAdd => { entities_components.add_component(e, ChildComponent { parent: Entity::EMPTY }); },
            Change::ChildRemove => { entities_components.remove_component::<ChildComponent>(e); },
            Change::ParentAdd => { entities_components.add_component(e, ParentComponent { children: Vec::new() }); },
            Change::ParentRemove => { entities_components.remove_component::<ParentComponent>(e); },
            Change::GlobalAdd => { entities_components.add_component(e, GlobalTransform::IDENTITY); },
        }
    }
}

// - Update ParentComponents according to ChildComponents
//     - Remove children from parent components
//     - Add missing children to parent components with creation if needed
//     - Destroy existing empty parent components


// - Update ParentComponents according to ChildComponents
//     - Remove children from parent components
pub fn update_parents_remove_invalid_children(
    mut parents: WorldQuery<(Entity, &mut ParentComponent)>,
    children: WorldQuery<&ChildComponent>,
) {
    let mut indices_to_remove = Vec::new();
    
    for (parent_entity, parent) in parents.iter_mut() {
        for (index, &child_entity) in parent.children.iter().enumerate().rev() {
            if children.get(child_entity).map(|c| c.parent != parent_entity).unwrap_or(true) {
                indices_to_remove.push(index);
            }
        }

        for &index in indices_to_remove.iter() {
            parent.children.remove(index);
        }

        indices_to_remove.clear();
    }
}

// - Update ParentComponents according to ChildComponents
//     - Add missing children to parent components with creation if needed
pub fn update_parents_add_missing_children(
    mut world: ExclusiveWorldAccess,
) {
    let children = world
        .entities_components()
        .query::<(Entity, &ChildComponent)>()
        .iter()
        .map(|(e, c)| (e, c.parent))
        .filter(|(_, pe)| world.entities_components().get_component::<ParentComponent>(*pe).is_some())
        .collect::<Vec<_>>();

    for (child_entity, parent_entity) in children.into_iter() {
        let parent = world.entities_components_mut().get_component_mut::<ParentComponent>(parent_entity).unwrap();

        if !parent.children.contains(&child_entity) {
            parent.children.push(child_entity);
        }
    }
}


// - Update ParentComponents according to ChildComponents
//     - Destroy existing empty parent components
pub fn update_parents_destroy_empty_parents(
    mut world: ExclusiveWorldAccess,
) {
    let empty_parents = world
        .entities_components()
        .query::<(Entity, &ParentComponent)>()
        .iter()
        .filter(|(_, p)| p.children.len() == 0)
        .map(|(e, _)| e)
        .collect::<Vec<_>>();

    for parent in empty_parents {
        world.entities_components_mut().remove_component::<ParentComponent>(parent);
    }
}

// - Calculate GlobalTransform from LocalTransform and child-parent relation with tree-ordering from a root parent to all the child leaves.
pub fn calculate_global_transform(
    mut world: ExclusiveWorldAccess,
) {
    let entities_components = world.entities_components_mut();

    let mut transforms_to_calc = entities_components
        .query::<(Entity, &GlobalTransform)>()
        .iter()
        .filter(|(e, _)| {
            let Some(child_component) = entities_components.get_component::<ChildComponent>(*e) else {
                return true;
            };

            !entities_components.contains_entity(child_component.parent)
        })
        .map(|(e, _)| e)
        .collect::<VecDeque<_>>();

    while let Some(transform) = transforms_to_calc.pop_front() {
        let parent_global_transform = match entities_components.get_component::<ChildComponent>(transform) {
            None => GlobalTransform::IDENTITY,
            Some(child_component) => match entities_components.get_component::<GlobalTransform>(child_component.parent) {
                None => GlobalTransform::IDENTITY,
                Some(&parent_global_transform) => parent_global_transform,
            }
        };

        // todo: Check geometry operations
        // {
        let Some(&local_transform) = entities_components.get_component::<LocalTransform>(transform) else {
            continue;
        };
        let Some(global_transform) = entities_components.get_component_mut::<GlobalTransform>(transform) else {
            continue;
        };
        global_transform.position = parent_global_transform.scale_rotation * local_transform.position + parent_global_transform.position;
        global_transform.scale_rotation = parent_global_transform.scale_rotation * (local_transform.rotation.to_matrix() * fruits_math::scale_matrix_3d(local_transform.scale));
        // }

        let Some(children) = entities_components.get_component::<ParentComponent>(transform) else {
            continue;
        };

        for &child in children.children.iter() {
            transforms_to_calc.push_back(child);
        }
    }
}