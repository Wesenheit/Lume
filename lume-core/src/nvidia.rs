use crate::core::{Renderable,Region,Structure};
use crate::utils::usage_to_u16_simple;
use serde::Deserialize;
use serde_yaml;
use nvml_wrapper::Nvml;

#[derive(Deserialize)]
pub enum NVIDIATypes{
    Util,
    Memory,
}

pub struct NVIDIA{
    devices:Vec<u32>,
    measure_type:NVIDIATypes,
    step:usize,
    nvml:Box<Nvml>
}

#[derive(Deserialize)]
struct NVIDIAConfig {
    devices:Vec<u32>,
    measure_type:NVIDIATypes,
}

impl NVIDIA {
    pub fn from_config(value: serde_yaml::Value,slide:usize) -> Self {
        let config: NVIDIAConfig = serde_yaml::from_value(value)
            .expect("invalid nvidia config");
        Self {
        devices: config.devices,
        measure_type: config.measure_type,
        step: slide,
        nvml: Box::new(Nvml::init().ok().unwrap()),
        }
    }

}
impl Renderable for NVIDIA{
    fn render_region(&mut self, rows: &mut [u16], region: Option<&Region>) {
        let (lower,upper) = match region{
            Option::Some(region) => (region.lower,region.upper),
            Option::None => (0,rows.len())
        };
        
        for ((i, device_id),row) in (lower..upper).zip(self.devices.iter()).zip(rows.iter_mut()) {
            let device = self.nvml.device_by_index(*device_id).unwrap();
            let utilization = device.utilization_rates().unwrap();
            let metric = match self.measure_type {
                NVIDIATypes::Util => utilization.gpu,
                NVIDIATypes::Memory => utilization.memory
            };

            let usage = usage_to_u16_simple((metric as f32)/100.);
            let diff = usage.count_ones() as i32 - row.count_ones() as i32;

            let new_bit = if diff > 0 {
                 1
            } else {
                0
            };
            *row = if (i & self.step) != 0 {
                (*row << 1) | new_bit
            } else {
                (*row >> 1) | (new_bit << 15)
            };

        }
    }
    fn get_structure(&self)->Structure {
        Structure::Sliding(self.step)
    }

}
