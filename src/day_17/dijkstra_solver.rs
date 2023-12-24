use std::collections::BinaryHeap;

use crate::day_17::heat_map::HeatMap;

use super::{direction::Direction, path::Path, position::Position};

pub struct DijsktraSolver {
    map: HeatMap,
    queue: BinaryHeap<State>,
}

impl DijsktraSolver {
    pub fn new(map: HeatMap) -> Self {
        let mut queue = BinaryHeap::new();

        queue.push(State::make_initial());

        return DijsktraSolver { map, queue };
    }

    pub fn solve(&mut self, destination: Position) -> State {
        while self.queue.len() > 0 {
            let state = self.pop_min();

            if state.position == destination {
                return state;
            }

            self.add_neighbors(state);
        }

        panic!("No path could be found.");
    }

    pub fn pop_min(&mut self) -> State {
        return self.queue.pop().unwrap();
    }

    pub fn add_neighbors(&mut self, state: State) {
        let directions = self.get_directions(&state);
        let neighbors = directions
            .iter()
            .map(|d| self.make_neighbor(&state, *d))
            .collect::<Vec<State>>();

        neighbors
            .iter()
            .cloned()
            .for_each(|n| self.add_neighbor_if_better(n));
    }

    fn add_neighbor_if_better(&mut self, state: State) {
        let existing = self.queue.iter().find(|p| p.position == state.position);

        if let Some(existing) = existing {
            if state.loss < existing.loss {
                self.add_neighbor(state);
            }
        } else {
            self.add_neighbor(state);
        }
    }

    fn add_neighbor(&mut self, state: State) {
        // self.remove_at(state.position);

        self.queue.push(state);
    }

    fn remove_at(&mut self, p: Position) {
        self.queue = self
            .queue
            .iter()
            .filter(|s| s.position != p)
            .cloned()
            .collect();
    }

    fn get_directions(&self, state: &State) -> Vec<Direction> {
        return Direction::get_all()
            .iter()
            .filter(|d| state.path.can_move(**d))
            .filter(|d| self.map.is_position_valid(state.position.move_by(**d)))
            .copied()
            .collect();
    }

    fn make_neighbor(&self, state: &State, direction: Direction) -> State {
        let position = state.position.move_by(direction);
        let path = state.path.move_by(direction);

        let loss = state.loss + self.map.get_heat_loss(position);

        return State {
            position,
            loss,
            path,
        };
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct State {
    pub position: Position,
    pub loss: u32,
    pub path: Path,
}

impl State {
    pub fn make_initial() -> Self {
        return State {
            position: Position::new(0, 0),
            loss: 0,
            path: Path::from_positions(vec![Position::new(0, 0)]),
        };
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(other.loss.cmp(&self.loss));
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.partial_cmp(other).unwrap();
    }
}
