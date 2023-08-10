# survex-dist
A command line utility for calculating the distance between two points in a [Survex](https://survex.com/) cave survey
file. `survex-dist` is not associated with the [Survex project](https://survex.com/), although I am very grateful for
their hard work!

## Features
### Implemented
* Process Survex 3D files to create a graph of the cave system.
* Calculate the shortest walking distance between any two stations in the cave system
  using A* pathfinding.
* Force a specific route through the cave by specifying survey stations to go via or exclude.
* Output to different formats: plain text, table, JSON.

### Planned
* Produce a graphical representation of the cave system and path.
* A web interface to allow the application to be used without installing it locally.
* Provide analysis of the cave system, such as the number of loops, number of entrances, etc.
* Provide analysis of various through trips in the cave system.

## Example
```text
> survex-dist tests/data/nottsii.3d nottsii.entrance entranceshaft.007

+-------------------+----------------------------+----------+------------+
| Station label     | Coords                     | Leg Dist | Total Dist |
+-------------------+----------------------------+----------+------------+
| entrance          | 66668.00, 78303.00, 319.00 | 0.00m    | 0.00m      |
| entranceshaft.002 | 66668.09, 78302.82, 319.00 | 0.20m    | 0.20m      |
| entranceshaft.003 | 66668.09, 78302.82, 317.93 | 1.07m    | 1.27m      |
| entranceshaft.004 | 66667.99, 78303.07, 317.93 | 0.27m    | 1.54m      |
| entranceshaft.005 | 66667.99, 78303.07, 306.40 | 11.53m   | 13.07m     |
| entranceshaft.006 | 66668.52, 78305.06, 305.16 | 2.40m    | 15.47m     |
| entranceshaft.007 | 66668.04, 78305.88, 304.03 | 1.48m    | 16.95m     |
+-------------------+----------------------------+----------+------------+

+------------------------+-----------------------------------------------+
| Metadata               | Value                                         |
+------------------------+-----------------------------------------------+
| Start station          | entrance                                      |
| End station            | committeepotentrance.entranceshaft.007        |
| Path length            | 7                                             |
| Path distance          | 16.95m                                        |
| Straight line distance | 15.24m                                        |
| Time taken             | 22.19ms                                       |
+------------------------+-----------------------------------------------+
```

## Usage
### Installation
`survex-dist` is not presently available as a binary. You can build and install it yourself using `cargo`, the Rust
package manager. If you need to install `cargo`, see [rustup.rs](https://rustup.rs/).

To build and install `survex-dist` in your `cargo` bin directory (usually `~/.cargo/bin`), run the following command:
```text
cargo install survex-dist
```

### Via and avoid
`survex-dist` can ensure that a certain route is taken by specifying via stations. This is done
using the `--via` flag. For example, to ensure that the route passes through the station
`entranceshaft.005`, use the following command:
```text
> survex-dist tests/data/nottsii.3d nottsii.entrance entranceshaft.007 --via entranceshaft.005
```

In exactly the same way, the `--avoid` flag can be used to ensure that a route does not pass
through a certain station. Both `--via` and `--avoid` can be used multiple times. Note that
`--via` stations are processed in the order that they are specified, and may cause a route to
loop back.

### Station matching
You can use partial station names whenever specifying a survey station.

For example, the station `nottsii.committeepotentrance.entranceshaft.007` was specified as `entranceshaft.007` above.
The partial match will be accepted as long as it is unambiguous. If the partial match is ambiguous, `survex-dist` will
print a list of possible matches and exit:

```text
❯ survex-dist tests/data/nottsii.3d nottsii.entrance entranceshaft
There were 35 possible matches for the station name 'entranceshaft'.

The first 20 matches were:

  nottsii.committeepotentrance.entranceshaft_2.15
  nottsii.committeepotentrance.entranceshaft_2.14
  nottsii.committeepotentrance.entranceshaft_2.13
  nottsii.committeepotentrance.entranceshaft_2.12
  ... snip ...
```
### Output format
The output format can be changed using the `--format` flag. The following formats are
supported:
- `table` (default)
- `text`
- `json`

### Survey analysis
`survex-dist` will, in the future, provide analysis of a survey using the `--analyse` flag. At
the moment, this flag does nothing.

### Further options
For a full list of options, run `survex-dist --help`.

## Contributing
Pull requests and issues are more than welcome - do let me know if you find any bugs or have a suggestion for a feature.

## See also
`survex-dist` makes use of the [survex-rs](https://github.com/anorthall/survex-rs) library to read Survex 3D files,
which in turn uses the [Survex](https://survex.com/) `img.c` library directly.
