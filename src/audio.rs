use crate::loading::AudioAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

pub struct InternalAudioPlugin;

// This plugin is responsible to control the game audio
impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), setup_menu_music)
            .add_systems(OnExit(GameState::Menu), stop_main_menu_music);
    }
}

#[derive(Resource)]
struct MainMenuMusic(Handle<AudioInstance>);

fn setup_menu_music(mut commands: Commands, audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
    // audio.pause();
    let handle = audio
        .play(audio_assets.mierda.clone())
        .looped()
        .with_volume(1.0)
        .handle();
    commands.insert_resource(MainMenuMusic(handle));

    println!("Playing main menu music");
}

fn stop_main_menu_music(
    main_menu_music: Res<MainMenuMusic>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    if let Some(instance) = audio_instances.get_mut(&main_menu_music.0) {
        match instance.state() {
            PlaybackState::Playing { .. } => {
                instance.stop(AudioTween::default());
            }
            _ => {}
        }
    }
}
