use amethyst::{
    assets::Loader,
    audio::{AudioSink, OggFormat, Source, SourceHandle},
    ecs::prelude::World,
};
use std::{iter::Cycle, vec::IntoIter};

const AUDIO_MUSIC_SILENT: &'static [&'static str] = &[];
const AUDIO_MUSIC_START: &'static [&'static str] = &["audio/score.ogg"];
const AUDIO_MUSIC_GAME: &'static [&'static str] = &["audio/Kevin_MacLeod-Hot_Pursuit.ogg"];

pub struct Music {
    pub music: Cycle<IntoIter<SourceHandle>>,
}

fn load_audio_track(loader: &Loader, world: &World, file: &str) -> SourceHandle {
    loader.load(file, OggFormat, (), (), &world.read_resource())
}

pub enum MusicTracks {
    Silent,
    Start,
    Game,
}

pub fn initialise_audio(world: &mut World) {
    let music = {
        let loader = world.read_resource::<Loader>();

        let mut sink = world.write_resource::<AudioSink>();
        sink.set_volume(0.5);

        let music = AUDIO_MUSIC_SILENT
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

pub fn change_track(world: &mut World, track: MusicTracks) {
    let mut sink = world.write_resource::<AudioSink>();

    let loader = world.read_resource::<Loader>();
    let mut music = world.write_resource::<Music>();
    music.music = match track {
        MusicTracks::Silent => AUDIO_MUSIC_SILENT,
        MusicTracks::Start => AUDIO_MUSIC_START,
        MusicTracks::Game => AUDIO_MUSIC_GAME,
    }
    .iter()
    .map(|file| load_audio_track(&loader, &world, file))
    .collect::<Vec<_>>()
    .into_iter()
    .cycle();

    sink.pause();
    music.music.next();
    sink.play();

    match track {
        MusicTracks::Silent => sink.set_volume(0.7),
        MusicTracks::Start => sink.set_volume(0.9),
        MusicTracks::Game => sink.set_volume(0.5),
    };
}
