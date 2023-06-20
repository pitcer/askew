# askew

Toy curves editor made for Curves and Surfaces in Computer Graphics course at university.
It features CPU-based computations and rendering, very low memory footprint (13M after start) and IPC (see [scripts/](https://github.com/pitcer/askew/tree/master/scripts)).

## How to compile and run

First, you need to install the latest stable [Rust toolchain](https://www.rust-lang.org/tools/install). Then follow these instructions:

* `git clone https://github.com/pitcer/askew.git`
* `cd askew`
* (download)[https://download.jetbrains.com/fonts/JetBrainsMono-2.304.zip] and unzip here a `JetBrainsMonoNL-Regular.ttf` font file
* `cargo run --release -- run`

Initial settings are available under `cargo run --release -- run --help`.

You can also directly use compiled binary at `./target/release/askew`.

## Keybinds

* `:` - enter command mode
* `enter` - execute command
* `escape` - exit mode
* `p` - change mode to point
* `s` - change mode to point select
* `a` - change mode to point add or add curve
* `d` - delete curve or point
* `j` - previous curve or point
* `k` - next curve or point
* `i` - increase rotation or weight
* `o` - decrease rotation or weight
* `h` - show convex hull
* `up|down|left|right` - move curve or point

## In-editor commands

* `:get show_convex_hull`
* `:get interpolation_nodes`
* `:get samples`
* `:set show_convex_hull <bool>`
* `:set interpolation_nodes <chebyshev|equally_spaced>`
* `:set samples <uint>`
* `:toggle show_convex_hull`
* `:toggle control_line`
* `:rotate <deg> [curve id]`
* `:move <x shift> <y shift>`
* `:save [path]`
* `:open [path]`
* `:set_curve_type <polyline|convex_hull|interpolation|bezier|rational_bezier>`
* `:get_curves_length`
* `:get_length <curve id>`
* `:get_point <curve id> <point id>`
* `:move_point <curve id> <point id> <x> <y>`
* `:trochoid_properties <range_start,range_end,r_1,r_2,w_1,w_2>`
