# survex-dist
A command line tool to calculate the distance between two points in a cave survey using
the output from the Survex `dump3d` command. Please note that this application is not
affiliated with the Survex project.

## Features
### Implemented
* Process Survex `dump3d` files to create a graph of the cave system.
* Calculate the shortest walking distance between any two stations in the cave system
  using A* pathfinding.
* Output to different formats: plain text, table, JSON.
* Avoid certain passages by specifying a list of stations to avoid.

### Planned
* Process Survex `.3d` files directly.
* Produce a graphical representation of the cave system and path.
* Allow a 'via' option, forcing the pathfinding algorithm to take a particular route.
* A web interface to allow the application to be used without installing it locally.

## Usage
The application is currently in early development, but it is fully functional and can
successfully parse the output of a Survex `dump3d` and calculate the shortest walking
distance between any two stations using A* pathfinding.

### Requirements
* Rust compiler - see [https://rustup.rs/](https://rustup.rs/).
* Survex `dump3d` command output for your chosen survey data -
  see [the Survex website](https://survex.com/).

### Installation
At the moment, no binaries are provided. You must compile the program from source
using the Rust compiler. Clone the repository and change into the directory:

```bash
git clone git@github.com:anorthall/survex-dist.git
cd survex-dist
```

Then compile and install the program using Cargo:

```bash
cargo install --path .
```

The program will now be installed into your chosen Rust installation's `bin`
direction. In most cases, on *nix systems, this will be `~/.cargo/bin`. You may
need to add this directory to your `PATH` environment variable.

### Running the program
The program takes three arguments: the path to the Survex `dump3d` output file,
the name of the start station, and the name of the end station. A sample `dump3d`
output file for [Notts II](https://cncc.org.uk/cave/notts-2) from the
[British Caving Association Cave Registry](https://cave-registry.org.uk/) is
provided in the repository as `dump3d/notts_ii_with_entrance.txt`.

The entrance survey station for Notts II is called `nottsii.entrance`. We will
calculate the distance between the entrance and the point at which the entrance
passage meets the main streamway, for which the station name
is `nottsii.mainstreamway.mainstreamway3.32`.

```bash
survex-dist dump3d/notts_ii_with_entrance.txt nottsii.entrance mainstreamway3.32
```

Note that the name of the second station has been truncated to `mainstreamway3.32`.
The program will match shorter variants of a station name, as long as the name given
is unambiguous.

The output from the command is as follows:

```
+----------------------------------------+----------------------------+----------+------------+
| Station label                          | Coords                     | Leg Dist | Total Dist |
+----------------------------------------+----------------------------+----------+------------+
| entrance                               | 66668.00, 78303.00, 319.00 | 0.00m    | 0.00m      |
| committeepotentrance.entranceshaft.002 | 66668.09, 78302.82, 319.00 | 0.20m    | 0.20m      |
| committeepotentrance.entranceshaft.003 | 66668.09, 78302.82, 317.93 | 1.07m    | 1.27m      |
| committeepotentrance.entranceshaft.004 | 66667.99, 78303.07, 317.93 | 0.27m    | 1.54m      |
| committeepotentrance.entranceshaft.005 | 66667.99, 78303.07, 306.40 | 11.53m   | 13.07m     |
| committeepotentrance.entranceshaft.006 | 66668.52, 78305.06, 305.16 | 2.40m    | 15.47m     |
| committeepotentrance.entranceshaft.007 | 66668.04, 78305.88, 304.03 | 1.48m    | 16.95m     |
| committeepotentrance.entranceshaft.008 | 66668.04, 78305.88, 286.11 | 17.92m   | 34.87m     |
| committeepotentrance.entranceshaft.009 | 66668.89, 78307.05, 284.85 | 1.92m    | 36.79m     |
| committeepotentrance.entranceshaft.010 | 66671.42, 78307.87, 283.73 | 2.89m    | 39.67m     |
| committeepotentrance.entranceshaft.011 | 66672.18, 78309.32, 282.09 | 2.32m    | 41.99m     |
| committeepotentrance.entranceshaft.012 | 66674.79, 78310.70, 280.71 | 3.26m    | 45.25m     |
| committeepotentrance.entranceshaft.013 | 66676.70, 78312.82, 280.71 | 2.85m    | 48.10m     |
| committeepotentrance.entranceshaft.014 | 66679.30, 78313.52, 278.69 | 3.37m    | 51.47m     |
| committeepotentrance.entranceshaft.015 | 66681.27, 78310.24, 278.15 | 3.86m    | 55.33m     |
| committeepotentrance.entranceshaft.016 | 66680.33, 78307.35, 276.33 | 3.54m    | 58.88m     |
| committeepotentrance.entranceshaft.022 | 66682.02, 78307.50, 276.30 | 1.70m    | 60.57m     |
| committeepotentrance.entranceshaft.023 | 66682.02, 78307.50, 270.39 | 5.91m    | 66.48m     |
| committeepotentrance.entranceshaft.024 | 66680.23, 78305.84, 268.34 | 3.19m    | 69.67m     |
| committeepotentrance.entranceshaft.025 | 66680.83, 78305.07, 268.61 | 1.01m    | 70.68m     |
| committeepotentrance.inlet13.001       | 66681.97, 78303.17, 268.69 | 2.22m    | 72.90m     |
| committeepotentrance.inlet13.002       | 66681.97, 78303.17, 263.86 | 4.83m    | 77.73m     |
| committeepotentrance.inlet13.003       | 66684.59, 78301.84, 263.34 | 2.98m    | 80.72m     |
| committeepotentrance.inlet13.004       | 66684.59, 78301.84, 260.37 | 2.97m    | 83.69m     |
| committeepotentrance.mincemeataven.1   | 66696.40, 78308.12, 260.61 | 13.38m   | 97.06m     |
| committeepotentrance.inlet13.006       | 66704.50, 78309.11, 259.32 | 8.26m    | 105.33m    |
| committeepotentrance.inlet13.008       | 66708.21, 78314.61, 255.93 | 7.45m    | 112.78m    |
| committeepotentrance.inlet13.009       | 66722.16, 78330.38, 253.72 | 21.17m   | 133.95m    |
| committeepotentrance.inlet13.010       | 66719.31, 78340.33, 252.45 | 10.43m   | 144.37m    |
| committeepotentrance.inlet13.012       | 66713.87, 78337.62, 251.61 | 6.14m    | 150.51m    |
| committeepotentrance.inlet13.013       | 66711.68, 78340.71, 251.41 | 3.79m    | 154.30m    |
| committeepotentrance.inlet13.014       | 66708.59, 78347.20, 250.79 | 7.21m    | 161.52m    |
| committeepotentrance.inlet13.015       | 66707.85, 78352.04, 249.96 | 4.97m    | 166.48m    |
| committeepotentrance.inlet13.016       | 66697.75, 78351.53, 249.63 | 10.12m   | 176.60m    |
| committeepotentrance.inlet13.017       | 66697.41, 78360.45, 248.53 | 8.99m    | 185.59m    |
| committeepotentrance.inlet13.018       | 66693.53, 78357.89, 248.05 | 4.67m    | 190.27m    |
| committeepotentrance.inlet13.019       | 66680.85, 78357.64, 245.62 | 12.91m   | 203.18m    |
| committeepotentrance.inlet13.020       | 66676.05, 78360.58, 244.84 | 5.68m    | 208.86m    |
| committeepotentrance.inlet13.021       | 66676.63, 78367.22, 244.26 | 6.69m    | 215.55m    |
| committeepotentrance.inlet13.022       | 66674.23, 78372.29, 242.64 | 5.84m    | 221.39m    |
| mainstreamway.mainstreamway3.32        | 66675.36, 78372.90, 242.68 | 1.28m    | 222.68m    |
+----------------------------------------+----------------------------+----------+------------+

+-------------------------+-----------------------------------------+
| Metadata                | Value                                   |
+-------------------------+-----------------------------------------+
| Origin station          | nottsii.entrance                        |
| Destination station     | nottsii.mainstreamway.mainstreamway3.32 |
| Path length             | 41 stations                             |
| Average leg length      | 5.43m                                   |
| Straight line distance  | 103.75m                                 |
| Walking/survey distance | 222.68m                                 |
| Time taken              | 29.59ms                                 |
+-------------------------+-----------------------------------------+
```

The output starts with a table of the stations in the path, including the distance
from the previous station and the cumulative distance from the origin station. The
table is followed by a summary of the path.

The pathfinding can take some time for larger surveys. The program has been tested on
a survey with 70,000 datapoints which took around three seconds to find the shortest route
on each run. For smaller surveys, the time taken is negligible. If you wish to see a more
verbose output of the processing steps, you can enable logging by setting `RUST_LOG=trace`
or `RUST_LOG=info` in the environment before running the program.

### Generating `dump3d` files
The `dump3d` files which are used as input to the program can be generated by the
`dump3d` command which is provided with Survex. The program cannot parse Survex
`.3d` files directly - they must be converted using `dump3d` before use.

If you do not have Survex installed, visit the [Survex website](https://survex.com/)
to obtain it. Once installed, `dump3d` can be run from the command line.
For example, the following command was used to generate the `dump3d` file for the
Notts II survey data:

```bash
dump3d -l NottsIIWithEntrance.3d > notts_ii_with_entrance.txt
```

Note the use of the `-l` flag to include survey leg information in the output. This
is required and `dump3d` files without leg information cannot be used with `survex-dist`.
The pipe operator (`>`) is used to redirect the output to a file, otherwise it will
be printed to the terminal.

### Help
If you need any assistance using the program, please either raise an issue on GitHub
or contact me directly via email at `andrew@caver.dev`. For more information about
program options, run `survex-dist --help`.

## Contributing
Pull requests are welcome.

## License
This project is licensed under the GNU General Public License v3.0 - see the LICENCE
file for details.

## Links
* [Survex project](https://survex.com/)
* [Survex GitHub](https://github.com/ojwb/survex)
* [Survex dump3d command output](https://github.com/ojwb/survex/blob/master/src/dump3d.c)
