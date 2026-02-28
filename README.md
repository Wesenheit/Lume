# Lume

Lume is a simple library to display current usage of resurces with LEDs only.
It is designed to mimick the look of CM-1 and CM-2 machines from the Thinking Machines Corporation.
The overall goal of the project is to achieve a pleasing and modern look for the visualization while preserving 
functionality.

## Usage
Currently only CPU monitoring is supported. To visualize load for the current machine in TUI run
```
  cargo run cpu
```
Workload is visualized in columns, one column per cpu core. Number of lit lights in the column visualize how much core is occupied (all lit - full load, no lights - no load).
Visualization was designed to have esthetic look while allowing to estimate resource in an eyes glance.

![high_load](data/full_load.png)
*Very high load - all 32 core are working*

![high_load](data/medium_load.png)
*Medium load - roughy half of cores are working, some are utilized in only some percent*

![high_load](data/low_load.png)
*Low load - only few cores are utilized*



## Roadmap
- [ ] RAM utiliation
- [ ] GPU utilization
- [ ] Using real LED matrix for visualization
