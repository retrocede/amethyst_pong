use amethyst::{
    assets::{Loader, AssetStorage},
    audio::{output::Output, OggFormat, Source, SourceHandle},
    ecs::{World, WorldExt}
};

const BOUNCE_SOUND: &str = "audio/bounce.ogg";
const SCORE_SOUND: &str = "audio/score.ogg";

pub struct Sounds {
    pub score_sfx: SourceHandle,
    pub bounce_sfx: SourceHandle,
}

/// Loads an ogg audio track
fn load_audio_track(loader: &Loader, world: &World, file: &str) -> SourceHandle {
    loader.load(file, OggFormat, (), &world.read_resource())
}

/// Initialize audio in the world. Just sound effects for now
pub fn initialize_audio(world: &mut World) {
    let sound_effects = {
        let loader = world.read_resource::<Loader>();

        let sound = Sounds {
            bounce_sfx: load_audio_track(&loader, &world, BOUNCE_SOUND),
            score_sfx: load_audio_track(&loader, &world, SCORE_SOUND),
        };

        sound
    };

    // Add sound effects to the world
    world.insert(sound_effects);
}

pub fn play_bounce_sound(sounds: &Sounds, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(&sounds.bounce_sfx) {
            output.play_once(sound, 1.0);
        }
    }
}

pub fn play_score_sound(sounds: &Sounds, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(&sounds.score_sfx) {
            output.play_once(sound, 1.0);
        }
    }
}