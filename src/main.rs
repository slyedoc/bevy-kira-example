
use bevy::prelude::*;
use bevy_kira_audio::*;

fn main() {
    let mut app = App::build();
    app.insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(WindowDescriptor {
            title: "Bevy Kira Example".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .insert_resource(MyAudioChannels::default())
        .add_plugin(AudioPlugin)
        .add_startup_system(setup_bg.system())
        .run();
}

fn setup_bg(
        audio: Res<Audio>,
        assets: Res<AssetServer>,
        channels: Res<MyAudioChannels>,
) {
    println!("play background sound!");
    audio.play_looped_in_channel(
        assets.load("bensound-creativeminds.wav"),
        &channels.background,
    );
}

#[allow(dead_code)]
pub struct MyAudioChannels {
    background: AudioChannel,
    fx1: AudioChannel,
    fx2: AudioChannel,
}

impl Default for MyAudioChannels {
    fn default() -> Self {
        MyAudioChannels {
            background: AudioChannel::new("bg".to_owned()),
            fx1: AudioChannel::new("fx1".to_owned()),
            fx2: AudioChannel::new("fx2".to_owned()),
        }
    }
}