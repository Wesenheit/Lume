use crate::core::Renderable;


pub struct CM5;

fn cmp5_org(x: u16) -> u16 {
    ((x >> 0) ^ (x >> 4) ^ (x >> 13) ^ (x >> 15)) & 1
}

impl Renderable for CM5 {
    fn render(&self, matrix: &mut crate::core::Matrix) {
        for (i,row) in matrix.rows.iter_mut().enumerate(){
            let new = cmp5_org(*row);
            if (i & 4) != 0 {
                *row = (*row << 1) | new;
            } else {
                *row = (*row >> 1) | (new << 15);
            }
        }
    }
}
