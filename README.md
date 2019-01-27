# Dear Hunter

## Quickstart

```bash
cargo run --release
```

To only display warning/error logs
```bash
AMETHYST_LOG_LEVEL_FILTER=WARN cargo run --release
```

## Notes
### Coordinate system for `Transform::set_xyz()`
* Is expressed in arena units
* Origin is as follow

```
y
^
|
|
|----*
|    |
0-------> x
```
