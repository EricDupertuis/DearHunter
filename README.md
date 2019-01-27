# Dear Hunter

A project for the 2019's Global Game Jam.

## Gameplay

You play a hunter, you've been hunting for your family in the forest.
Now it's late, you have to get back home to feed your family.
But the forest is dark, you lost your path, and there are hogs around chasing you.
You have to get home without hitting on hogs and before your time is up (and your family dies of hunger!).

To move around use `W`,`A`,`S`,`D` keys.
To pass screens use `<Space>`.

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

## Licenses for audio assets

 - [`Kevin_MacLeod-Hot_Pursuit.ogg` (BY)](https://incompetech.com/music/royalty-free/index.html?isrc=USUAN1700084)
 - `score.ogg`: Contributed by Richard Dodd, released into the Public Domain (CC0)
