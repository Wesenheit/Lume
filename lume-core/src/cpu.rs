use sysinfo::{System, RefreshKind, CpuRefreshKind};

use crate::core::{Matrix,Renderable};
enum CpuVisType {
    Simple,
    Random,
}

pub struct Cpu{
    vis: CpuVisType,
    sys: System,
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
                if (i & 4) != 0 {
                    (row << 1) | new_bit
                } else {
                    (row >> 1) | (new_bit << 15)
                }
            }
        }
    }
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
    pub fn new(simple: bool) -> Cpu {
        let sys =System::new_with_specifics(
            RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()),
        );
        let typecpu = if simple {
            CpuVisType::Simple 
        } else {
            CpuVisType::Random
        };
        return Cpu { vis: typecpu, sys:sys  }
    }
    pub fn count(&self) -> usize {
        return self.sys.cpus().len();
    }
}

impl Renderable for Cpu {
    fn render(&mut self, matrix: &mut Matrix) {
        self.sys.refresh_cpu_all();
        for (i, cpu) in self.sys.cpus().iter().enumerate() {
            matrix.rows[i] = self.get_row(matrix.rows[i],i, cpu.cpu_usage())
        }
    }
}
