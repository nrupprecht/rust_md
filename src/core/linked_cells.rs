use crate::core::simdata::{Bounds, SimData};
use crate::core::vector::Position;
use std::cmp::max;

#[derive(Debug, Clone)]
pub struct Cell {
    pub particle_ids: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct LinkedCells {
    num_x: usize,
    num_y: usize,
    cells: Vec<Cell>,

    /// The low and high bounds in each dimension.
    pub bounds: Bounds,

    cell_width: f32,
    cell_height: f32,
}

impl LinkedCells {
    pub fn get_num_x(&self) -> usize {
        self.num_x
    }

    pub fn get_num_y(&self) -> usize {
        self.num_y
    }

    /// Create a new set of linked cells object.
    pub fn new(bounds: Bounds, target_size: f32) -> Self {
        // Calculate the number of x and y cells
        if target_size <= 0. {
            panic!("target size cannot be less than or equal to zero");
        }

        let num_x = max(1, f32::floor(bounds.width() / target_size) as usize);
        let num_y = max(1, f32::floor(bounds.height() / target_size) as usize);
        let num_cells = num_x * num_y;

        let cell_width = bounds.width() / (num_x as f32);
        let cell_height = bounds.height() / (num_y as f32);

        LinkedCells {
            num_x,
            num_y,
            cells: vec![
                Cell {
                    particle_ids: vec![]
                };
                num_cells as usize
            ],
            bounds,
            cell_width,
            cell_height,
        }
    }

    /// Create a new LinkedCells, taking its particle data from SimData.
    pub fn new_for_simdata(sim_data: &SimData, target_size: f32) -> Self {
        LinkedCells::new(sim_data.bounds, target_size)
    }

    /// Get a cell given the x and y indices of the cell.
    pub fn get_cell(&self, x: usize, y: usize) -> Option<&Cell> {
        if self.num_x <= x || self.num_y <= y {
            return None;
        }
        let index = self.num_x * y + x;
        Some(self.cells.get(index as usize).expect("Could not get cell"))
    }

    pub fn get_adjusted_cell(&self, x: usize, y: usize, dx: i32, dy: i32) -> Option<&Cell> {
        if (dx < 0 && x < -dx as usize)
            || (dy < 0 && y < -dy as usize)
            || (0 < dx && self.num_x < x + dx as usize)
            || (0 < dy && self.num_y < y + dy as usize)
        {
            return None;
        }

        let adjx = (x as i32) + dx;
        let adjy = (y as i32) + dy;
        self.get_cell(adjx as usize, adjy as usize)
    }

    /// Get a cell given the x and y indices of the cell.
    pub fn get_mut_cell(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        if self.num_x <= x || self.num_y <= y {
            return None;
        }
        let index = self.num_x * y + x;
        Some(
            self.cells
                .get_mut(index as usize)
                .expect("Could not get cell"),
        )
    }

    /// Get what cell a position falls inside.
    pub fn get_cell_indices(&self, x: f32, y: f32) -> (usize, usize) {
        let ix = ((x - self.bounds.xlo) / self.cell_width) as usize;
        let iy = ((y - self.bounds.ylo) / self.cell_height) as usize;
        (ix, iy)
    }

    /// Add a particle into the linked cells object.
    ///
    /// Returns the cell into which the particle was added.
    pub fn add_particle(&mut self, position: &Position, id: usize) -> &mut Cell {
        let (ix, iy) = self.get_cell_indices(position.x, position.y);
        let mut cell = self
            .get_mut_cell(ix, iy)
            .expect("A particle must belong to some cell");
        cell.particle_ids.push(id);
        cell
    }
}

// =================================================================================================
//  Unit Tests.
// =================================================================================================
