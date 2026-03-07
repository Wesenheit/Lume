use crate::core::{Renderable,Structure,Region};


pub struct CM5;

fn cmp5_org(x: u16) -> u16 {
    ((x >> 0) ^ (x >> 4) ^ (x >> 13) ^ (x >> 15)) & 1
}

impl Renderable for CM5 {
    fn render_region(&mut self, matrix: &mut [u16],region:Region) {
        for (i,row) in region.iter().zip(matrix.iter_mut()){
            let new = cmp5_org(*row);
            if (i & 4) != 0 {
                *row = (*row << 1) | new;
            } else {
                *row = (*row >> 1) | (new << 15);
            }
        }
    }
    fn get_structure(&self)->Structure {
        Structure::Sliding(4)
    }
}
