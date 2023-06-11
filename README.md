# [Pronto Graphics](https://github.com/Garbaz/pronto-graphics) for [Roc](https://github.com/roc-lang/roc)

A Roc _platform_ for my simple 2D graphics crate [Pronto Graphics](https://github.com/Garbaz/pronto-graphics).

## Usage

For the time being, before you can use this plaform, you have to copy or symbolic link in the folder of the `roc_std` crate. So if you have the [Roc git repository](https://github.com/roc-lang/roc) cloned at `/my/path/to/roc`, then run in this repository:

```sh
ln -s /my/path/to/roc/crates/roc_std
```

Once `roc_std` is on [crates.io](https://crates.io/) this will no longer be necessary.

Once that is done, create a new Roc application like this:

```elm
app "hello-pronto"
    packages { pf: "pronto/main.roc" } # Or whatever the path is in your case
    imports []
    provides [init, draw] to pf

init = Windowed 800 600 "Hello Pronto"

draw = \pg ->
    pg.circle((100, 100), 50)
```