
use robrs::tools::oscillator::Oscillator;

fn main() {
    let vmax = 3;
    let tickdelay = 1;
    let w8minmax = true;
    let oscillator = Oscillator::new(vmax, tickdelay, w8minmax);

    for value in oscillator.take(5 * vmax as usize + 1) {
        println!("{}", value);
    }
}
