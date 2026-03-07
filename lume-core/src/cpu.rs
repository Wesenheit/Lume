use sysinfo::{System, RefreshKind, CpuRefreshKind};
use itertools::Itertools;
use serde_yaml;
use serde::Deserialize;

use crate::core::{Renderable,Structure,Region};
enum CpuVisType {
    Simple,
    Random,
}

#[derive(Deserialize)]
struct CpuConfig {
    simple: bool,
    reduce: usize,
}

pub struct Cpu{
    vis: CpuVisType,
    sys: System,
    reduce: usize,
    step:usize,
}

fn usage_to_u16_simple(usage: f32) -> u16 {
    let clamped_usage = usage.min(100.0).max(0.0);
    
    let num_bits = (clamped_usage / 100.0 * 16.0).round() as u32;

    if num_bits == 0 {
        0
    } else if num_bits >= 16 {
        u16::MAX
    } else {
        (1u16 << num_bits) - 1
    }
}

impl Cpu {
    pub fn get_row(&self,row:u16,i:usize,cpu_usage:f32) -> u16{
        match self.vis {        
            CpuVisType::Simple => {
                usage_to_u16_simple(cpu_usage)
            },
            CpuVisType::Random => {
                let usage = usage_to_u16_simple(cpu_usage);
                let diff = usage.count_ones() as i32 - row.count_ones() as i32;

                let new_bit = if diff > 0 {
                    1
                } else {
                    0
                };
                if (i & self.step) != 0 {
                    (row << 1) | new_bit
                } else {
                    (row >> 1) | (new_bit << 15)
                }
            }
        }
    }
    pub fn new(simple: bool,reduce:usize,step:usize) -> Cpu {
        let sys =System::new_with_specifics(
            RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()),
        );
        let typecpu = if simple {
            CpuVisType::Simple 
        } else {
            CpuVisType::Random
        };
        return Cpu { vis: typecpu, sys:sys ,reduce:reduce ,step:step}
    }
    pub fn from_config(value: serde_yaml::Value,slide:usize) -> Self {
        let config: CpuConfig = serde_yaml::from_value(value)
            .expect("invalid cpu config");
        Self::new(config.simple, config.reduce, slide)
    }
    pub fn count(&self) -> usize {
        return (self.sys.cpus().len() + self.reduce - 1) / self.reduce;
    }
}

impl Renderable for Cpu {
    fn render_region(&mut self, matrix: &mut[u16],region:Option<&Region>) {
        let (lower,upper) = match region{
            Option::Some(region) => (region.lower,region.upper),
            Option::None => (0,matrix.len())
        };
        
        self.sys.refresh_cpu_all();
        for (i, chunk) in (lower..upper).zip(self.sys.cpus().iter().chunks(self.reduce).into_iter()) {
            let (sum, num) = chunk.fold((0.0f32, 0usize), |(s, c), cpu| {
                (s + cpu.cpu_usage(), c + 1)
            });
            matrix[i] = self.get_row(matrix[i],i, sum/num as f32)
        }
    }
    fn get_structure(&self)->Structure {
        match self.vis {
            CpuVisType::Simple => Structure::Static,
            CpuVisType::Random => Structure::Sliding(self.step)
        }
    }
}
