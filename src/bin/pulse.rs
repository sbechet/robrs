use std::fs::File;
use std::path::Path;
use clap::Parser;
use wav::WAV_FORMAT_PCM;

use robrs::tools::pulse_wave::PulseWave;
use robrs::tools::resample::Resample;


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Filename prefix
    filename: String,
    /// Pulse Width
    #[arg(short = 'w', long, default_value = "2048", help = "0..=4095")]
    pulse_width: u16,
    /// Pulse Delay
    #[arg(short = 'd', long, default_value = "0", help = "0..=31")]
    pulse_delay: u8,
    /// Pulse Speed
    #[arg(short = 's', long, default_value = "0", help = "0..=255")]
    pulse_speed: u8,
}

fn main() {
    let cli = Cli::parse();

    let sid_wave = PulseWave::new(cli.pulse_width, cli.pulse_speed, cli.pulse_delay);
    let loop_start = sid_wave.get_loop_start();
    let loop_end = sid_wave.get_loop_end();
    let ticks = sid_wave.ticks;
    println!("{} ticks", ticks);

    let mut data: Vec<u8> = vec![];
    for sample in sid_wave.into_iter() {
        data.push(sample);
    }

    let samples_per_tick = data.len() as f32/ticks as f32;
    let divat48 = 48000.0 / 440.0;
    let ratio = samples_per_tick / divat48;
    println!("{} samples per tick, we want 440Hz => {} per ticks, we want one sample per {}", samples_per_tick, divat48, ratio);
    println!("loop_start:{}, loop_end={}", loop_start, loop_end);

    // let filename = format!("{}_{}_{}_{}_{}.orig.wav",cli.filename, cli.pulse_width, cli.pulse_speed, cli.pulse_delay, data.len());
    // let mut out_file = File::create(Path::new(&filename)).unwrap();

    // let header = wav::Header::new(WAV_FORMAT_PCM, 1, 48_000, 8);
    // wav::write(header, &wav::BitDepth::Eight(data.clone()), &mut out_file).unwrap();


    let resample = Resample::new(samples_per_tick as usize, divat48 as usize, data.into_iter());

    let mut data2: Vec<u8> = vec![];
    for sample in resample.into_iter() {
        data2.push(sample);
    }

    let filename = format!("{}_{}_{}_{}_{}.wav",cli.filename, cli.pulse_width, cli.pulse_speed, cli.pulse_delay, data2.len()/ticks as usize);
    let mut out_file = File::create(Path::new(&filename)).unwrap();

    let header = wav::Header::new(WAV_FORMAT_PCM, 1, 48_000, 8);
    wav::write(header, &wav::BitDepth::Eight(data2), &mut out_file).unwrap();

}
