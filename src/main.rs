use alsa::pcm::{Access, Format, HwParams, State, PCM};
use alsa::{Direction, ValueOr};
use std::thread::sleep;
use std::time::{Duration, Instant};

use resid::{ChipModel, SamplingMethod, Sid};
mod rhplayer;
use rhplayer::RhPlayer;

mod song_monty_on_the_run;

fn main() {
    println!("\n\n\nHello my friends :)\n\n");
    println!("Monty On The Run Playing just for u...");

    let mut sid: Box<Sid> = Box::new(Sid::new(ChipModel::Mos6581));
    // sid.set_sampling_parameters(SamplingMethod::Fast, 970200, 44100);   // TODO:970200=44100*22 not sure it's a good idea?
    // sid.set_sampling_parameters(SamplingMethod::Interpolate, 985_248, 44100);    // TODO:not working?
    // sid.set_sampling_parameters(SamplingMethod::Resample, 985_248, 44100);   // TODO:not working?
    // sid.set_sampling_parameters(SamplingMethod::ResampleFast, 985_248, 44100);   // TODO:not working?
    let mut player: Box<RhPlayer> =
        Box::new(RhPlayer::new(&mut sid, &song_monty_on_the_run::RHSONGS));

    /*
       0 : Monty on the run main theme
       1 : next song
       2 : next song
       3<=x<=19 : SoundFX
    */
    // TODO:not working for 13, 14, 16, 
    player.init(0);

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
    loop {
        player.play();

        // TODO: find solution to exit

        // alsa play
        let mut delta = 44100 / 2; // TODO:why?
        while delta > 0 {
            //println!("debuging resid-rs: {:?}", player.get_sid_regs());
            let (samples, next_delta) = player.sample(delta, &mut buffer[..], 1);
            io.writei(&buffer[0..samples]).unwrap();
            delta = next_delta;
        }
    }
    // Wait for the stream to finish playback.
    pcm.drain().unwrap();
}
