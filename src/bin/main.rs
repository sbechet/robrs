use alsa::pcm::{Access, Format, HwParams, PCM};
use alsa::{Direction, ValueOr};
use clap::Parser;
use std::fs::File;
use std::io::prelude::*;
use std::{thread, time};

use resid::{ChipModel, SamplingMethod, Sid};

use robrs::rhplayer::rhplayer::RhPlayer;
use robrs::rhplayer::rhsongs::RhSongs;
use robrs::song::*;
use robrs::xm::convert::Convert;

use xmrs::xm::xmmodule::XmModule;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Game name, can be aceii, commando, crazycomets, delta, lastv8, lightforce, montyontherun, sanxion, sanxion2d, spellbound, thingonaspring, zoids
    name: String,
    /// song number, from 0 to ...
    number: usize,
    #[arg(short = 'e', long, default_value = "false")]
    experimental: bool,
    #[arg(short = 'm', long, default_value = "false")]
    midi: bool,
    #[arg(short = 'x', long, default_value = "false")]
    xm: bool,
}

fn sidplay(rhsongs: Option<&RhSongs>, number: usize) {
    println!("\n\n\nHello my friends :)\n\n");
    println!("Rust Rewrite Rob Hubbard Player playing just for u...");

    let mut sid: Box<Sid> = Box::new(Sid::new(ChipModel::Mos6581));
    // let mut sid: Box<Sid> = Box::new(Sid::new(ChipModel::Mos8580));

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
    // TODO:not working for monty on the run: 13
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
    let freq = 50; // Hz
    let wait_time = time::Duration::from_millis(1000 / freq - 1);
    loop {
        player.play();

        // TODO: find solution to exit

        // alsa play
        let mut delta: u32 = 44100 / 2; // TODO:why?
        while delta > 0 {
            // println!("debuging resid-rs: {:?}", player.get_sid_regs());
            let (samples, next_delta) = player.sample(delta, &mut buffer[..], 1);
            io.writei(&buffer[0..samples]).unwrap();
            delta = next_delta;
        }
        thread::sleep(wait_time);
    }
    // Wait for the stream to finish playback.
    pcm.drain().unwrap();
}

fn main() {
    let cli = Cli::parse();

    let rhsongs = match cli.name.as_str() {
        "aceii" => Some(&song_ACE_II::RHSONGS),
        "commando" => Some(&song_Commando::RHSONGS),
        "crazycomets" => Some(&song_Crazy_Comets::RHSONGS),
        "delta" => Some(&song_Delta::RHSONGS),
        "humanrace" => Some(&song_The_Human_Race::RHSONGS),
        "ikarate" => Some(&song_International_Karate::RHSONGS),
        "lastv8" => Some(&song_The_Last_V8::RHSONGS),
        "lightforce" => Some(&song_Lightforce::RHSONGS),
        "montyontherun" => Some(&song_Monty_on_the_Run::RHSONGS),
        "sanxion" => Some(&song_Sanxion_Song_1::RHSONGS),
        "sanxion2d" => Some(&song_Sanxion_Song_2::RHSONGS),
        "spellbound" => Some(&song_Spellbound::RHSONGS),
        "thingonaspring" => Some(&song_Thing_on_a_Spring::RHSONGS),
        "zoids" => Some(&song_Zoids::RHSONGS),
        _ => None,
    };

    let number = cli.number;

    if cli.midi {
        if rhsongs.is_some() {
            rhsongs.unwrap().print_song(number);
            rhsongs.unwrap().write_midi(number);
            rhsongs.unwrap().write_all_tracks();
            rhsongs.unwrap().write_channel_tracks(0, 0);
            rhsongs.unwrap().write_channel_tracks(0, 1);
            rhsongs.unwrap().write_channel_tracks(0, 2);
        }
    } else if cli.xm {
        let (name, editor) = match cli.name.as_str() {
            "montyontherun" => ("Monty on the Run", "Rob Hubbard - 1985 Gremlin Graphics"),
            _ => ("", ""),
        };
        let module = Convert::convert(
            name.to_string(),
            editor.to_string(),
            rhsongs.unwrap(),
            number,
        );
        let mut xmmodule2: XmModule = XmModule::from_module(&module);
        let xmodule2_se = xmmodule2.save().unwrap();
        let mut file = File::create(format!("{}.xm", name)).unwrap();
        file.write_all(&xmodule2_se).unwrap();
        println!("Save XM file");
    } else if cli.experimental {
        
    } else {
        sidplay(rhsongs, number);
    }
}
