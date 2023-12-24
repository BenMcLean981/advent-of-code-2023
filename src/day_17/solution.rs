use crate::{
    day_17::{
        dijkstra_solver::{DijsktraSolver, State},
        heat_map::HeatMap,
    },
    utils::file_utils::read_lines,
};

pub fn solve() {
    let filename = "src/day_17/input.txt";

    let binding = read_lines(filename);
    let lines = binding.iter().map(|s| s.as_str()).collect();

    let heat_map = HeatMap::from_lines(lines);
    let destination = heat_map.get_bot_right();

    let mut solver = DijsktraSolver::new(heat_map.clone());

    let state: State = solver.solve(destination);
    heat_map.debug_path(state.path.clone());

    let loss = state.loss;

    println!("Day 17");
    println!("The loss of the shortest path is: {loss}.");
}
