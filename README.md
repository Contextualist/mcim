# Monte Carlo Simulation 2D Ising Model

[**Detailed instructions**](https://github.com/Contextualist/mcim/wiki)

Based on [Ising2D](https://github.com/OpenSourcePhysics/STP/blob/master/src/org/opensourcephysics/stp/ising/ising2d/Ising2D.java) of Open Source Physics STP. This implements the Metropolis algorithm.

### Usage

```bash
./mcim temperature magnetic_field size step
```

* temperature: heat bath temperature, in `J/k_B`, where `J` is the exchange constant
* magnetic field: external magnetic field
* size: linear dimension of lattice
* step: total simulation step to run; measured in terms of Monte Carlo steps per spin

The simulation will start from a paramagnetic (randomized) initial state. Output will be a CSV file in which each row includes the average energy (per spin) and average magnetization (per spin) at that step.
