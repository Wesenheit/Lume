use rand::Rng;


pub struct Matrix {
    pub rows: Vec<u16>
}

pub trait Renderable {
    fn render(&self, matrix: &mut Matrix);
}

impl Matrix {
    pub fn zero(height:usize) -> Self {
        Self{rows:vec![0;height]}
    }

    pub fn random(height:usize) -> Self {
        let mut rng = rand::thread_rng();
        
        let mut rows:Vec<u16> = vec![0;height];
        rng.fill(&mut rows[..]);

        Self{rows:rows}
    }

    pub fn update<T: Renderable>(&mut self, pattern: &T) {
        pattern.render(self);
    }
}
