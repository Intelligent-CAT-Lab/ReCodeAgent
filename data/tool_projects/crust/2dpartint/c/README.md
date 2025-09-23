# 2DPartInt

Geotechnical problems have been traditionally tackled as an extension of continuum mechanics theories. Such an approach, although appropriate for systems undergoing small displacements, cannot examine important problems such as landslides, rockfalls, and rainfall-induced landslides. Our implementation is a first step toward analyzing these complex behavior, and, as a starting point, studies systems constituted of geomaterials assumed as particles in a two-dimensional space. With this implementation, we plan to bring to the geotechnical community, academic and practitioners in general, a computational tool that will allow solving challenges involving multiphysics and large displacements effectively and accurately.

![Simulation](https://raw.githubusercontent.com/eafit-apolo/2DPartInt/master/simulacion.gif)

## Usage

Compile the project using make.

```bash
$ make
```

_(Optional)_ run the tests to ensure everything is ok.


```bash
$ make test
```

To debug some simulation step compile like this.

```bash
$ make DEBUG_STEP=true
```

Execute the program passing the `simulation_config.txt` file & the output folder as the arguments.

```bash
$ ./bin/2DPartInt simulation_config.txt out/
```

### Profile the program

Install the package `gperftools` or the equivalent in your operating system.

Compile defining the `PROFILING` flag.

```
$ make PROFILING=true
```

Run defining where to store the profiler output.

```
$ CPUPROFILE=profiler_out.prof ./bin/2DPartInt simulation_config.txt out/
```

Print profiling information.

```
$ pprof --text ./bin/2DPartInt profiler_out.prof
```

More runtime and offline options for the profiler [here](https://gperftools.github.io/gperftools/cpuprofile.html).

## Simulation Config File.

The behaviour of the simulation is determined by the config file. For finding collisions between particles, we use a Grid-like
data structure. The area covered by the Grid will be the only area in space that will be simulated. Particles whose center lies
outside the Grid area will not be taken into account for findind collisions between particles, even though they would mantain
their forces.

In the config file one defines the number of squares on the X coordinate and the number of squares on the Y coordinate, along
with the length of each square. A configuration of x_squares=4, y_squares=3 will yield the following grid:

![OnlyGrid](https://raw.githubusercontent.com/eafit-apolo/2DPartInt/master/img/OnlyGrid.jpg)

This is the structure of the file.
_(All settings are mandatory, but they can be in any order). You should not add comentaries as below._

```
time=[Double] # Total time to simulate, in seconds.
dt=[Double] # Size of the time window used for each step.
x_particles=[Int] # Number of particles along the X coordinate
y_particles=[Int] # Number of particles along the Y coordinate
x_squares=[Int] # Number of squares along the X coordinate
y_squares=[Int] # Number of squares along the X coordinate
square_in_grid_length=[Double] # Length side of each square in the grid
radius=[Int] # Radius of each particle.
kn=[Double] # Normal rigidity of the material.
ks=[Double] # Tangential rigidity of the material.
rho=[Double] # Density of the material.
thickness=[Double] # Thickness of the particles.
v0=[Double] # Initial velocity of the falling particle.
r0=[Double] # Radius of the falling particle.
```
