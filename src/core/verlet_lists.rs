use crate::core::linked_cells::LinkedCells;
use crate::core::simdata::SimData;

/// A verlet lists structure, that stores particles that are "close" to one another.
pub struct VerletLists {
    /// Each entry is a vector of a particle ID, and the IDs of all particles that are "close" to
    /// that particle.
    pub verlet_lists: Vec<(usize, Vec<usize>)>,

    /// The number of potential pairs in the verlet lists object.
    num_pairs: usize,
}

impl From<Vec<(usize, Vec<usize>)>> for VerletLists {
    fn from(value: Vec<(usize, Vec<usize>)>) -> Self {
        let num_pairs = value.iter().fold(0, |sum, x| sum + x.1.len());
        VerletLists {
            verlet_lists: value,
            num_pairs,
        }
    }
}

pub fn check_neighbors(
    id1: usize,
    ids_to_check: &[usize],
    sim_data: &SimData,
    neighbors: &mut Vec<usize>,
    cutoff: f32,
) {
    for id2 in ids_to_check.iter().copied() {
        let rsqr = sim_data.distance_sqr_between(id1, id2);
        let rdiff = sim_data.radii[id1] + sim_data.radii[id2] + cutoff;

        // This is a neighbor of
        if rsqr < rdiff * rdiff {
            neighbors.push(id2);
        }
    }
}

pub fn create_verlet_lists(sim_data: &SimData, cutoff: f32) -> VerletLists {
    let mut verlet_lists = Vec::new(); // : Vec<(i32, Vec<i32>)>
                                       // If there are no particles, there is nothing to do.
    if sim_data.is_empty() {
        return VerletLists::from(verlet_lists);
    }

    // Bin particles into sectors.
    // NOTE: We need a characteristic radius to create bins. For systems with roughly equally sized
    // particles, we can use the max particle radius.
    let max_radius = sim_data.radii.iter().copied().fold(f32::NAN, f32::max);

    // Bin particles in the linked cells structure.
    let mut linked_cells = LinkedCells::new_for_simdata(&sim_data, max_radius);
    for id in 0..sim_data.num_particles() {
        linked_cells.add_particle(&sim_data.positions.get(id).unwrap(), id);
    }

    // Create verlet lists from the linked cells.
    for ix in 0..linked_cells.get_num_x() {
        for iy in 0..linked_cells.get_num_y() {
            let cell = linked_cells.get_cell(ix, iy).unwrap();

            for i in 0..cell.particle_ids.len() {
                let id1 = cell.particle_ids[i];

                let mut neighbors = Vec::new();

                // Top row.
                if let Some(cell) = linked_cells.get_adjusted_cell(ix, iy, -1, 1) {
                    check_neighbors(
                        id1,
                        cell.particle_ids.as_slice(),
                        &sim_data,
                        &mut neighbors,
                        cutoff,
                    );
                }
                if let Some(cell) = linked_cells.get_adjusted_cell(ix, iy, 0, 1) {
                    check_neighbors(
                        id1,
                        cell.particle_ids.as_slice(),
                        &sim_data,
                        &mut neighbors,
                        cutoff,
                    );
                }
                if let Some(cell) = linked_cells.get_adjusted_cell(ix, iy, 1, 1) {
                    check_neighbors(
                        id1,
                        cell.particle_ids.as_slice(),
                        &sim_data,
                        &mut neighbors,
                        cutoff,
                    );
                }
                if let Some(cell) = linked_cells.get_adjusted_cell(ix, iy, -1, 0) {
                    check_neighbors(
                        id1,
                        cell.particle_ids.as_slice(),
                        &sim_data,
                        &mut neighbors,
                        cutoff,
                    );
                }

                // Same cells.
                check_neighbors(
                    id1,
                    &cell.particle_ids[i + 1..],
                    &sim_data,
                    &mut neighbors,
                    cutoff,
                );

                // If any neighbors of id1 were found, add them to the verlet lists.
                if 0 < neighbors.len() {
                    verlet_lists.push((id1, neighbors));
                }
            }
        }
    }

    VerletLists::from(verlet_lists)
}

// =================================================================================================
//  Unit Tests.
// =================================================================================================

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_verlet_list_construction() {
        let mut v = Vec::new();
        v.push((0usize, vec![1usize, 2usize, 3usize]));
        v.push((1usize, vec![2usize, 4usize]));
        v.push((5usize, vec![6usize, 7usize]));

        let vl = VerletLists::from(v);
        assert_eq!(vl.num_pairs, 7);
    }
}
