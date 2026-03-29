use crate::utils::{take_even, take_lower, take_upper};
use rand::Rng;

pub struct MatrixConfig {
    pub size: usize,
    pub reduce: bool,
}

pub enum Structure {
    Static,
    Sliding(usize),
}

pub struct Matrix {
    pub rows: Vec<u16>,
    pub reduce: bool,
    pub rows_u8: Vec<u8>,
}

#[derive(Clone)]
pub struct Region {
    pub lower: usize,
    pub upper: usize,
}

impl Region {
    pub fn iter(&self) -> impl Iterator<Item = usize> + '_ {
        self.lower..self.upper
    }
}

pub trait Renderable {
    fn render_region(&mut self, rows: &mut [u16], region: Option<&Region>);
    fn render(&mut self, matrix: &mut Matrix) {
        self.render_region(&mut matrix.rows, Option::None);
    }
    fn get_structure(&self) -> Structure;
}

impl Matrix {
    fn allocate(config: MatrixConfig) -> Self {
        let base: Vec<u16> = vec![0; config.size];
        let reduced_version: Vec<u8> = if config.reduce {
            vec![0; config.size]
        } else {
            Vec::new()
        };
        Self {
            rows: base,
            reduce: config.reduce,
            rows_u8: reduced_version,
        }
    }

    pub fn zero(config: MatrixConfig) -> Self {
        let mut matrix = Matrix::allocate(config);
        matrix.rows.fill(0);
        if matrix.reduce {
            matrix.rows_u8.fill(0);
        }
        matrix
    }

    pub fn random(config: MatrixConfig) -> Self {
        let mut rng = rand::thread_rng();

        let mut matrix = Matrix::allocate(config);
        rng.fill(&mut matrix.rows[..]);
        matrix
    }

    pub fn update(&mut self, pattern: &mut dyn Renderable) {
        pattern.render(self);
        if self.reduce {
            self.rows_u8
                .iter_mut()
                .zip(self.rows.iter())
                .enumerate()
                .for_each(|(i, (red, full))| {
                    *red = match pattern.get_structure() {
                        Structure::Static => take_even(*full),
                        Structure::Sliding(step) => {
                            if (i & step) != 0 {
                                take_lower(*full)
                            } else {
                                take_upper(*full)
                            }
                        }
                    }
                });
        }
    }
}
