# Dear Hunter

## Run

```bash
cargo run +nightly
```

To only display warning/error logs
```bash
AMETHYST_LOG_LEVEL_FILTER=WARN cargo run
```

# Coordinate system for set_xyz()
* Is expressed in arena units
* Origin is as follow

y
^
|
|
|----*
|    |
0-------> x
