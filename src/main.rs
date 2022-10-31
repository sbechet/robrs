use std::{thread, time};
use alsa::pcm::{Access, Format, HwParams, PCM};
use alsa::{Direction, ValueOr};
use clap::Parser;

use resid::{ChipModel, SamplingMethod, Sid};
mod rhplayer;
use rhplayer::RhPlayer;

mod rhsongs;
use rhsongs::RhSongs;

use crate::rhsongs::MusicPlayer;

mod song_commando;
mod song_crazycomets;
mod song_lastv8;
mod song_monty_on_the_run;
mod song_spellbound;
mod song_thing_on_a_spring;
mod song_zoids;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Game name, can be montyontherun, commando, thingonaspring, crazycomets, zoids, lastv8
    name: String,
    /// song number, from 0 to ...
    number: usize,
    /// midi output
    midi: Option<bool>
}

fn sidplay(rhsongs: Option<&RhSongs>, number: usize) {

    println!("\n\n\nHello my friends :)\n\n");
    println!("Rust Rewrite Rob Hubbard Player playing just for u...");

    let mut sid: Box<Sid> = Box::new(Sid::new(ChipModel::Mos6581));

    // sid.set_sampling_parameters(SamplingMethod::Fast,1022727, 44100);   // NTSC Commando?
    // sid.set_sampling_parameters(SamplingMethod::Interpolate, 985_248, 44100);    // TODO:not working?
    // sid.set_sampling_parameters(SamplingMethod::Resample, 985_248, 44100);   // TODO:not working?
    // sid.set_sampling_parameters(SamplingMethod::ResampleFast, 985_248, 44100);   // TODO:not working?

    let mut player: Box<RhPlayer> = Box::new(RhPlayer::new(&mut sid, rhsongs.unwrap()));

    /*
       0 : Monty on the run main theme
       1 : next song
       2 : next song
       3<=x<=19 : SoundFX
    */
    // TODO:not working for monty on the run: 13, 14, 16,
    // TODO: not working for commando: 7, 12, 16
    player.init(number);

    // Open default playback device
    let pcm: Box<PCM> = Box::new(PCM::new("default", Direction::Playback, false).unwrap());

    // Set hardware parameters: 44100 Hz / Mono / 16 bit
    let hwp = Box::new(HwParams::any(&pcm).unwrap());
    hwp.set_channels(1).unwrap();
    hwp.set_rate(44100, ValueOr::Nearest).unwrap();
    hwp.set_format(Format::s16()).unwrap();
    hwp.set_access(Access::RWInterleaved).unwrap();
    pcm.hw_params(&hwp).unwrap();
    let io = Box::new(pcm.io_i16().unwrap());

    // Make sure we don't start the stream too early
    let hwp = Box::new(pcm.hw_params_current().unwrap());
    let swp = Box::new(pcm.sw_params_current().unwrap());
    swp.set_start_threshold(hwp.get_buffer_size().unwrap())
        .unwrap();
    pcm.sw_params(&swp).unwrap();

    let mut buffer = vec![0i16; 8192];
    let mut freq = 50; // Hz
    let wait_time = time::Duration::from_millis(freq);
    loop {
        player.play();

        // TODO: find solution to exit

        // alsa play
        let mut delta:u32 = 44100 / 2; // TODO:why?
        while delta > 0 {
            // println!("debuging resid-rs: {:?}", player.get_sids_regs());
            let (samples, next_delta) = player.sample(delta, &mut buffer[..], 1);
            io.writei(&buffer[0..samples]).unwrap();
            delta = next_delta;
        }
        // thread::sleep(wait_time);

    }
    // Wait for the stream to finish playback.
    pcm.drain().unwrap();
}

fn main() {
    let cli = Cli::parse();

    let rhsongs = match cli.name.as_str() {
        "commando" => Some(&song_commando::RHSONGS),
        "crazycomets" => Some(&song_crazycomets::RHSONGS),
        "montyontherun" => Some(&song_monty_on_the_run::RHSONGS),
        "lastv8" => Some(&song_lastv8::RHSONGS),
        "spellbound" => Some(&song_spellbound::RHSONGS),
        "thingonaspring" => Some(&song_thing_on_a_spring::RHSONGS),
        "zoids" =>  Some(&song_zoids::RHSONGS),
        _ => None,
    };

    let number = cli.number;

    if cli.midi.unwrap_or(false) {
        if rhsongs.is_some() {
            rhsongs.unwrap().print_song(number);
            rhsongs.unwrap().write_midi(number);
            rhsongs.unwrap().write_all_patterns();
            rhsongs.unwrap().write_channel_patterns(0, 0);
            rhsongs.unwrap().write_channel_patterns(0, 1);
            rhsongs.unwrap().write_channel_patterns(0, 2);
        }
    } else {
        sidplay(rhsongs, number);
    }
}
