use crate::core::{Region, Renderable, Structure};

pub struct CM5 {
    pub sliding: usize,
}

fn cmp5_org(x: u16) -> u16 {
    ((x >> 0) ^ (x >> 4) ^ (x >> 13) ^ (x >> 15)) & 1
}

impl Renderable for CM5 {
    fn render_region(&mut self, matrix: &mut [u16], region: Option<&Region>) {
        let (lower, upper) = match region {
            Option::Some(region) => (region.lower, region.upper),
            Option::None => (0, matrix.len()),
        };
        for (i, row) in (lower..upper).zip(matrix.iter_mut()) {
            let new = cmp5_org(*row);
            if (i & self.sliding) != 0 {
                *row = (*row << 1) | new;
            } else {
                *row = (*row >> 1) | (new << 15);
            }
        }
    }
    fn get_structure(&self) -> Structure {
        Structure::Sliding(4)
    }
}
