use sysinfo::{System, RefreshKind, CpuRefreshKind};

use crate::core::{Matrix,Renderable};
enum CpuVisType {
    Simple,
}

pub struct Cpu{
    vis: CpuVisType,
    sys: System,
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
    pub fn new() -> Cpu {
        let sys =System::new_with_specifics(
            RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()),
        );
        return Cpu { vis: CpuVisType::Simple, sys:sys  }
    }
    pub fn count(&self) -> usize {
        return self.sys.cpus().len();
    }
}

impl Renderable for Cpu {
    fn render(&mut self, matrix: &mut Matrix) {
        self.sys.refresh_cpu_all();
        for (i, cpu) in self.sys.cpus().iter().enumerate() {
            match self.vis {
                CpuVisType::Simple => {
                    matrix.rows[i] = usage_to_u16_simple(cpu.cpu_usage())
                }
            }
        }
    }
}
