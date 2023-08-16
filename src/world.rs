use crate::cell::Cell;
use std::collections::HashSet;

pub struct World {
    alive_coords: HashSet<(i64, i64)>,
}

impl World {
    pub fn new(alive_coords: HashSet<(i64, i64)>) -> World {
        World { alive_coords }
    }

    pub fn get_by_xy(&self, x: i64, y: i64) -> Cell {
        let is_alive = self.alive_coords.contains(&(x, y));
        Cell::new(x, y, is_alive)
    }

    pub fn get_relative(&self, cell: &Cell, x: i64, y: i64) -> Cell {
        self.get_by_xy(cell.x + x, cell.y + y)
    }

    pub fn get_alive_cells(&self) -> Vec<Cell> {
        self.alive_coords
            .iter()
            .map(|(x, y)| Cell::new(*x, *y, true))
            .collect()
    }

    fn get_neighbours(&self, cell: &Cell) -> [Cell; 8] {
        let gr = |x, y| self.get_relative(cell, x, y);

        [
            gr(-1, -1),
            gr(-1, 0),
            gr(-1, 1),
            gr(1, -1),
            gr(1, 0),
            gr(1, 1),
            gr(0, -1),
            gr(0, 1),
        ]
    }

    fn get_relevant_dead_cells(&self) -> Vec<Cell> {
        let dead_cells: HashSet<_> = self
            .get_alive_cells()
            .iter()
            .flat_map(|cell| self.get_neighbours(cell))
            .filter(|cell| !cell.is_alive)
            .collect();

        dead_cells.into_iter().collect()
    }

    fn get_relevant_cells(&self) -> Vec<Cell> {
        let mut cells = self.get_alive_cells();
        cells.extend(self.get_relevant_dead_cells());
        cells
    }

    fn will_be_alive(&self, cell: &Cell) -> bool {
        let alive_neighbours = self
            .get_neighbours(cell)
            .iter()
            .filter(|cell| cell.is_alive)
            .count();

        alive_neighbours == 3 || (cell.is_alive && alive_neighbours == 2)
    }

    pub fn next(&self) -> World {
        let alive_coords: HashSet<_> = self
            .get_relevant_cells()
            .iter()
            .filter(|cell| self.will_be_alive(cell))
            .map(|cell| (cell.x, cell.y))
            .collect();

        World { alive_coords }
    }
}
