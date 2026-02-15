use rand::Rng;
use crate::utils::{take_lower,take_upper,take_even};

pub struct MatrixConfig{
    pub size:usize,
    pub reduce:bool
}

pub enum Structure {
    Static,
    Sliding(usize)
}

pub struct Matrix {
    pub rows: Vec<u16>,
    pub reduce:bool,
    pub rows_u8: Vec<u8>,
}

pub trait Renderable {
    fn render(&mut self, matrix: &mut Matrix);
    fn get_structure(&self)->Structure;
}

impl Matrix {
    fn allocate(config:MatrixConfig) -> Self{
        let base:Vec<u16> = vec![0;config.size];
        let reduced_version:Vec<u8> = if config.reduce {
            vec![0;config.size]
        } else {
            Vec::new()
        };
        return Self{rows:base,reduce:config.reduce,rows_u8:reduced_version}
    }
    
    pub fn zero(config: MatrixConfig) -> Self {
        let mut matrix  = Matrix::allocate(config);
        matrix.rows.fill(0);
        if matrix.reduce {
            matrix.rows_u8.fill(0);  
        }
        return matrix
    }

    pub fn random(config:MatrixConfig) -> Self {
        let mut rng = rand::thread_rng();

        let mut matrix  = Matrix::allocate(config);
        rng.fill(&mut matrix.rows[..]);
        return matrix
    }

    pub fn update(&mut self, pattern: &mut dyn Renderable) {
        pattern.render(self);
        if self.reduce {
            self.rows_u8.iter_mut().zip(self.rows.iter()).enumerate()
                .for_each(|(i,(red,full))| {
                    
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
