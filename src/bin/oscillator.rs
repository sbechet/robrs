struct Oscillator {
    vmax: isize,
    direction: bool,
    current_value: isize,
    tickdelay: usize,
    tickcount: usize,
    w8minmax: bool,
}

impl Oscillator {
    /**
     * vmax: oscillator beetween 0..vmax..0..vmax..
     * tickdelay: repeat value tickdelay time
     * w8minmax: wait and repeat min and max value like 0..vmax vmax..0 0..vmax vmax..0...
     */
    fn new(vmax: isize, tickdelay: usize, w8minmax: bool) -> Self {
        Oscillator {
            vmax,
            direction: true,
            current_value: 0,
            tickdelay,
            tickcount: 0,
            w8minmax,
        }
    }
}

impl Iterator for Oscillator {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        let result = Some(self.current_value);

        self.tickcount += 1;

        if self.tickcount == self.tickdelay {
            self.tickcount = 0;

            if self.direction {
                if !self.w8minmax {
                    self.current_value += 1;
                }

                if self.current_value >= self.vmax {
                    self.direction = false;
                } else if self.w8minmax {
                    self.current_value += 1;
                }
            } else {
                if !self.w8minmax {
                    self.current_value -= 1;
                }

                if self.current_value <= 0 {
                    self.direction = true;
                } else if self.w8minmax {
                    self.current_value -= 1;
                }
            }
        }

        result
    }
}

fn main() {
    let vmax = 3;
    let tickdelay = 1;
    let w8minmax = true;
    let oscillator = Oscillator::new(vmax, tickdelay, w8minmax);

    for value in oscillator.take(5 * vmax as usize + 1) {
        println!("{}", value);
    }
}
