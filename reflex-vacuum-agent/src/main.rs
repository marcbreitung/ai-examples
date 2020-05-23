extern crate rand;

use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let mut world = World::new(Location::A);
    let mut perceptions = vec![];
    for _ in 0..100 {
        perceptions.push(world.percept());
        world.update(reflex_vacuum_agent(world.percept()));
        match rng.gen_range(0, 100) {
            x if x < 25 => world.location_right = Status::Dirty,
            x if x > 75 => world.location_left = Status::Dirty,
            _ => {}
        }
    }
    println!("Perceptions: {:?}", perceptions);
}

#[derive(Debug)]
struct World {
    location_left: Status,
    location_right: Status,
    cleaner_position: Location,
}

impl World {
    fn new(start: Location) -> Self {
        Self {
            location_left: Status::Dirty,
            location_right: Status::Dirty,
            cleaner_position: start,
        }
    }

    fn update(&mut self, action: Action) {
        let position = &self.cleaner_position;
        match (action, position) {
            (Action::Clean, Location::A) => self.location_left = Status::Clean,
            (Action::Clean, Location::B) => self.location_right = Status::Clean,
            (Action::Right, Location::A) => self.cleaner_position = Location::B,
            (Action::Left, Location::B) => self.cleaner_position = Location::A,
            (_, _) => {}
        }
    }

    fn percept(&self) -> Perception {
        match (&self.cleaner_position, &self.location_left, &self.location_right) {
            (Location::A, Status::Dirty, _) => Perception(Location::A, Status::Dirty),
            (Location::A, Status::Clean, _) => Perception(Location::A, Status::Clean),
            (Location::B, _, Status::Dirty) => Perception(Location::B, Status::Dirty),
            (Location::B, _, Status::Clean) => Perception(Location::B, Status::Clean),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Status {
    Dirty,
    Clean,
}

#[derive(Debug, PartialEq)]
enum Location {
    A,
    B,
}

#[derive(Debug, PartialEq)]
enum Action {
    Left,
    Right,
    Clean,
}

#[derive(Debug, PartialEq)]
struct Perception(
    Location,
    Status,
);

// Returns an action based on the given perception
fn reflex_vacuum_agent(perception: Perception) -> Action {
    match perception {
        Perception(_, Status::Dirty) => {
            Action::Clean
        }
        Perception(Location::A, Status::Clean) => {
            Action::Right
        }
        Perception(Location::B, Status::Clean) => {
            Action::Left
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Action, Location, Perception, reflex_vacuum_agent, Status, World};

    #[test]
    fn should_return_action_clean() {
        let start = Perception(Location::A, Status::Dirty);
        let action = reflex_vacuum_agent(start);
        assert_eq!(action, Action::Clean);
    }

    #[test]
    fn should_return_action_right() {
        let start = Perception(Location::A, Status::Clean);
        let action = reflex_vacuum_agent(start);
        assert_eq!(action, Action::Right);
    }

    #[test]
    fn should_return_action_left() {
        let start = Perception(Location::B, Status::Clean);
        let action = reflex_vacuum_agent(start);
        assert_eq!(action, Action::Left);
    }

    #[test]
    fn should_return_initial_perception() {
        let start = Perception(Location::A, Status::Dirty);
        let world = World::new(Location::A);
        assert_eq!(start, world.percept());
    }

    #[test]
    fn should_return_next_perception() {
        let next = Perception(Location::A, Status::Clean);
        let mut world = World::new(Location::A);
        world.update(reflex_vacuum_agent(world.percept()));
        assert_eq!(next, world.percept());
    }
}
