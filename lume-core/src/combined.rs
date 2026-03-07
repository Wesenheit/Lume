use crate::core::{Matrix, Region, Renderable, Structure};
use crate::cpu::Cpu;
use crate::nvidia::NVIDIA;
use crate::random::CM5;
use serde::Deserialize;

pub struct CombinedPattern {
    patterns: Vec<Box<dyn Renderable>>,
    ranges: Vec<Region>,
    sliding: usize,
    size: usize,
}

#[derive(Deserialize)]
pub struct RegionConfig {
    pub start: usize,
    pub end: usize,
}

#[derive(Deserialize)]
pub struct ModuleEntry {
    pub name: String,
    pub region: RegionConfig,
    pub config: serde_yaml::Value,
}

#[derive(Deserialize)]
pub struct CombinedConfig {
    pub modules: Vec<ModuleEntry>,
    pub step: usize,
}

impl Renderable for CombinedPattern {
    fn render_region(&mut self, _rows: &mut [u16], _region: Option<&Region>) {
        panic!("Combined Pattern cannot be called with render_region!")
    }
    fn get_structure(&self) -> Structure {
        return Structure::Sliding(self.sliding);
    }
    fn render(&mut self, matrix: &mut Matrix) {
        for (pattern, region) in self.patterns.iter_mut().zip(self.ranges.iter()) {
            pattern.render_region(
                &mut matrix.rows[region.lower..region.upper],
                Option::Some(region),
            );
        }
    }
}

impl CombinedPattern {
    pub fn from_yaml(path: &str) -> Self {
        let contents = std::fs::read_to_string(path).expect("failed to read config");
        let config: CombinedConfig = serde_yaml::from_str(&contents).expect("failed to parse yaml");

        let (ranges, patterns): (Vec<Region>, Vec<Box<dyn Renderable>>) = config
            .modules
            .into_iter()
            .map(|entry| {
                let region = Region {
                    lower: entry.region.start,
                    upper: entry.region.end,
                };
                let pattern: Box<dyn Renderable> = match entry.name.as_str() {
                    "cpu" => Box::new(Cpu::from_config(entry.config, config.step)),
                    "random" => Box::new(CM5 {
                        sliding: config.step,
                    }),
                    "nvidia" => Box::new(NVIDIA::from_config(entry.config, config.step)),
                    other => panic!("unknown module: {}", other),
                };
                (region, pattern)
            })
            .collect();
        let mut ranges_copy: Vec<Region> = ranges.iter().map(|r| r.clone()).collect();
        ranges_copy.sort_by_key(|r| r.lower);

        for window in ranges_copy.windows(2) {
            let (a, b) = (&window[0], &window[1]);
            assert!(a.upper <= b.lower,);
        }
        let min = ranges.iter().map(|r| r.lower).min();
        let max = ranges.iter().map(|r| r.upper).max();
        match min {
            Option::Some(value) => assert!(value == 0),
            Option::None => panic!("No ranges specified"),
        }
        return Self {
            patterns,
            ranges,
            sliding: config.step,
            size: max.expect("no ranges specified!"),
        };
    }
    pub fn get_size(&self) -> usize {
        return self.size;
    }
}
