# survex-dist
A command line tool to calculate the distance between two points  in a cave survey using
the output from the Survex `dump3d` command. The project is in early development and is
not yet functional.

## Usage
The project is barely function, but the following command will run the program so far:

```cargo run -- dump3d/notts_ii_with_entrance.txt a b```

## Links
* [Survex project](https://survex.com/)
* [Survex GitHub](https://github.com/ojwb/survex)
* [Survex dump3d command output](https://github.com/ojwb/survex/blob/master/src/dump3d.c)

## Licence
This project is licensed under the GNU General Public License v3.0 - see the LICENCE
file for details.
