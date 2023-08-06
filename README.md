# Bevy Game of Life 
![demo.gif][demo.gif]
This repository originates from an intriguing project named
[falling_sand_experiment](https://github.com/dfebs/falling_sand_experiment).
While my initial motivation for forking was to develop a logic gate simulator,
the realization of potential intricacies arising from overlapping wires
prompted a shift in focus. Consequently, I pivoted towards creating Conway's
Game of Life as a more feasible endeavor. 

## Dependencies
**This was copied from bevy's [linux_dependencies](https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md)

### [Ubuntu](https://ubuntu.com/)

```bash
sudo apt-get install g++ pkg-config libx11-dev libasound2-dev libudev-dev
```

if using Wayland, you will also need to install

```bash
sudo apt-get install libwayland-dev libxkbcommon-dev
```
### [Fedora](https://getfedora.org/)

```bash
sudo dnf install gcc-c++ libX11-devel alsa-lib-devel systemd-devel
```

if using Wayland, you will also need to install

```bash
sudo dnf install wayland-devel libxkbcommon-devel
```
### Arch / Manjaro

```bash
sudo pacman -S libx11 pkgconf alsa-lib
```

Install `pipewire-alsa` or `pulseaudio-alsa` depending on the sound server you are using.

Depending on your graphics card, you may have to install one of the following:
`vulkan-radeon`, `vulkan-intel`, or `mesa-vulkan-drivers`

### Void

```bash
sudo xbps-install -S pkgconf alsa-lib-devel libX11-devel eudev-libudev-devel
```

### Nix
The project includes a shell.nix file that includes all the necessary packages.
Run nix-shell within the projects directory before building.

```bash
nix-shell
```


## Building and Running

```bash
cargo run --release
```

## Controls
- Scroll: Vertical camera movement.
- Left Shift + Scroll: Horizontal camera Movement.
- Escape: Erase all living cells.
- Left Click: Spawn a cell.
- Right Click: Remove a cell.
- Space: Toggle Pause / Play.

## Points of Interest

### Parallel Processing Using Rayon

Every cell generation is efficiently computed in parallel using Rayon's
par_iter() function. This enables simultaneous processing of all cells, though
occasional memory constraints may temporarily hinder the process. 

### No Grid System Is Actually Used

Conway's game of life is normally calculated using a grid system. Every cell, alive
or dead must be tested for neighbors. This implemenation instead uses a BTreeSet (a form of HashSet)
to instead check for contained neighbors and generate possible newborn cells. This
reduces the time complexity from O(gridsize^2) to O(cells*8) and allows for 
simplistic parallel processing. Furthermore, the distance from each cell has no
affect on the computation time unlike most implemenations and thus gridsize
can have "infinite" size (i.e. i32 x i32).

### Faster Hashset Indexing Using Bitwise Encoding

Rust's BTreeSet encounters limitations when dealing with complex types such as
tuples or Vec2. To address this, a more efficient approach is adopted: two
i32-sized coordinates are compactly stored within a single u64. This
optimization accelerates indexing during BTreeSet::contains() calls,
guaranteeing a constant time complexity of O(1) across all scenarios.

## Original Fork README: 

This experiment was created in 2 work days + an additional evening! It was made
using Rust and the Bevy library.

For those who want to try it, you will need to be capable of compiling and
running rust programs. If so, cloning this repo and doing a `cargo run` should
do the trick. It may take a minute at first since Bevy is a chonker. 
