use bevy::{core::FixedTimestep, ecs::component::Component, prelude::*};
use bevy_asset_loader::*;
use bevy_kira_audio::*;
use std::fmt::Debug;
use std::hash::Hash;
use pretty_type_name::pretty_type_name;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum MyState {
    Loading,
    First,
}

fn main() {
    let mut app = App::build();

    app.insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(WindowDescriptor {
            title: "Bevy Kira Example".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        // Some tests
        .add_state(MyState::Loading)
        .insert_resource(MyAudioChannels::default())
        .add_plugin(AudioPlugin)
        .add_system_set(
            // Play Song on loop
            SystemSet::on_enter(MyState::First)
                //.with_system(setup_bg.system()) //Works every time
                .with_system(setup_bg_loader.system()) // Fails sometimes

        )
        .add_system_set(
            // Play Audio every 3 seconds
            SystemSet::on_update(MyState::First)
                .with_run_criteria(FixedTimestep::step(3.0))
                //.with_system(play_fx1.system()) //Works every time
                .with_system(play_fx1_loader.system()) // Never works, will panic
        ).add_system_set(
            // Dipslay Loading Status while in loading state
            SystemSet::on_update(MyState::Loading)
            .with_system(display_loading_status::<Handle<AudioSource>, AudioAssets>.system())
        );

        AssetLoader::new(MyState::Loading, MyState::First)
        .with_collection::<AudioAssets>()
        .build(&mut app);

        app.run();
}

// some debug info about AssetCollection
#[allow(dead_code)]
fn display_loading_status<T: Component + Debug + Clone + Eq + Hash, Assets: AssetCollection>(
    asset_server: Res<AssetServer>,
    loading_asset_handles: Option<Res<LoadingAssetHandles<Assets>>>,
) {
    if let Some(loading_asset_handles) = loading_asset_handles {
        let load_state = asset_server.get_group_load_state(loading_asset_handles.handles.clone());
        println!("{:?} - {:?}", pretty_type_name::<T>(), load_state);
    }
}


#[derive(AssetCollection)]
pub struct AudioAssets {
    #[asset(path = "bensound-creativeminds.wav")]
    pub background_music: Handle<AudioSource>,

    #[asset(path = "bounce.wav")]
    pub bounce: Handle<AudioSource>,
}

#[allow(dead_code)]
fn setup_bg(
    audio: Res<Audio>,
    assets: Res<AssetServer>,
    channels: Res<MyAudioChannels>,
) {
    println!("background sound!");
    audio.set_volume_in_channel(0.5, &channels.background);
    audio.play_looped_in_channel(
        assets.load("bensound-creativeminds.wav"), // works
        &channels.background,
    );
}

#[allow(dead_code)]
fn setup_bg_loader(
    audio: Res<Audio>,
    channels: Res<MyAudioChannels>,
    audio_asset: Res<AudioAssets>,
) {
    println!("loader background sound!");
    audio.set_volume_in_channel(0.5, &channels.background);
    audio.play_looped_in_channel(
        audio_asset.background_music.clone(),
        &channels.background,
    );
}

#[allow(dead_code)]
fn play_fx1(
    audio: Res<Audio>,
    assets: Res<AssetServer>,
    channels: Res<MyAudioChannels>,
) {
    println!("fx1 sound!");
    audio.play_in_channel(
        assets.load("bounce.wav"), // works
        &channels.fx1,
    );
}
#[allow(dead_code)]
fn play_fx1_loader(
    audio: Res<Audio>,
    channels: Res<MyAudioChannels>,
    audio_asset: Res<AudioAssets>,
) {
    println!("loader fx1 sound!");
    audio.play_in_channel(
        audio_asset.bounce.clone(), // Fails
        &channels.fx1,
    );
}

#[allow(dead_code)]
pub struct MyAudioChannels {
    background: AudioChannel,
    fx1: AudioChannel,
}

impl Default for MyAudioChannels {
    fn default() -> Self {
        MyAudioChannels {
            background: AudioChannel::new("bg".to_owned()),
            fx1: AudioChannel::new("fx1".to_owned()),
        }
    }
}
