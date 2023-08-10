# color-swapper

This is a simple program written in Rust that, as the name suggests, swaps the colors of an SVG file, in theory.

The reality is that, as of right now, this accepts any UTF8 text file and replaces a given text with another one.

### Why ?

This could probably be done in 1 command in UNIX systems, but I wanted to learn Rust and this seemed like a good little project to start with.

### How does it work?

It reads your input folder for any file and **assumes they are `.svg` files** (will be fixed soon). It then it reads each file individually and replaces the given text with the new one.

After the replacement, it saves the result in the output folder, as `.svg`.

Once this is done, it will ask you if you wish to convert your new files to an image. If you do, it will use [librsvg](https://gitlab.gnome.org/GNOME/librsvg) to convert the files to a 512x512 transparent `.png`.

### How to use

Just run the program. It will ask you what to do.

### TODO

- [ ] Check file extensions before processing
- [ ] Print configuration before swapping
- [ ] Handle first-time run correctly
