use super::schedule_behavior::{ScheduleBehavior, ScheduleBehaviorBuilder};

#[derive(Clone, Copy)]
pub enum Schedule {
    Start = 0,
    Update = 1,
}

impl Schedule {
    pub const COUNT: usize = 2;
    pub const fn index(self) -> usize { self as usize }
}

pub struct WorldBehaviorBuilder {
    schedule_behaviors: [ScheduleBehaviorBuilder; Schedule::COUNT],
}

impl WorldBehaviorBuilder {
    pub fn new() -> Self {
        Self {
            schedule_behaviors: core::array::from_fn(|_| ScheduleBehaviorBuilder::new()),
        }
    }

    pub fn get_mut(&mut self, schedule: Schedule) -> &mut ScheduleBehaviorBuilder {
        &mut self.schedule_behaviors[schedule.index()]
    }

    pub fn build(self) -> WorldBehavior {
        WorldBehavior {
            schedule_behaviors: self.schedule_behaviors.map(|b| b.build()),
        }
    }
}

pub struct WorldBehavior {
    schedule_behaviors: [ScheduleBehavior; Schedule::COUNT],
}

impl WorldBehavior {
    pub fn get(&self, schedule: Schedule) -> &ScheduleBehavior {
        &self.schedule_behaviors[schedule.index()]
    }
}
