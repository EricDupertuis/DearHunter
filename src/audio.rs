use amethyst::{
    assets::{Loader},
    audio::{AudioSink, OggFormat, Source, SourceHandle},
    ecs::prelude::World,
};
use std::{iter::Cycle, vec::IntoIter};

const AUDIO_MUSIC: &'static [&'static str] = &["audio/Kevin_MacLeod-Hot_Pursuit.ogg"];

pub struct Music {
    pub music: Cycle<IntoIter<SourceHandle>>,
}

fn load_audio_track(loader: &Loader, world: &World, file: &str) -> SourceHandle {
    loader.load(file, OggFormat, (), (), &world.read_resource())
}

pub fn initialise_audio(world: &mut World) {
    let music = {
        let loader = world.read_resource::<Loader>();

        let mut sink = world.write_resource::<AudioSink>();
        sink.set_volume(0.25); // Music is a bit loud, reduce the volume.

        let music = AUDIO_MUSIC
            .iter()
            .map(|file| load_audio_track(&loader, &world, file))
            .collect::<Vec<_>>()
            .into_iter()
            .cycle();
        let music = Music { music };

        music
    };

    world.add_resource(music);
}
