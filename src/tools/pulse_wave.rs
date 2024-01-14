const ONE_PULSE: u16 = 4096;
const MIDDLE: u16 = ONE_PULSE / 2;
const THREE_QUARTER: u16 = MIDDLE + MIDDLE * 3/4;

pub struct PulseWave {
    pulse_width: u16,
    pulse_speed: u8,
    pulse_delay: u8,
    pub ticks: u16,

    sample_step: u16,
    pulse_width_step: u16,
    pulse_delay_step: u8,
    ticks_step: u16,

    increment: bool,

}

impl PulseWave {
    /*
     * pulse_width: 0..3584
     * pulse_speed: 1..31
     * pulse_delay: 1..255
     */
    pub fn new(pulse_width: u16, pulse_speed: u8, pulse_delay: u8) -> Self {
        //
        PulseWave {
            pulse_width,
            pulse_speed,
            pulse_delay,
            ticks: Self::get_ticks(pulse_width, pulse_speed, pulse_delay),

            sample_step: 0,
            pulse_width_step: pulse_width,
            pulse_delay_step: pulse_delay,
            ticks_step: Self::get_ticks(pulse_width, pulse_speed, pulse_delay),

            increment: true,
        }
    }

    // how many ticks for full sample
    fn get_ticks(pulse_width : u16, pulse_speed: u8, pulse_delay: u8) -> u16 {
        if pulse_speed != 0 {
            let step1 = pulse_delay as u16 * (THREE_QUARTER - pulse_width) / pulse_speed as u16;
            let step2 = pulse_delay as u16 * (THREE_QUARTER - MIDDLE) / pulse_speed as u16;
            step1 + 2*step2
        } else {
            1
        }

    }

    pub fn get_loop_start(&self) -> u16 {
        let step1 = if self.pulse_speed != 0 {
            self.pulse_delay as u16 * (THREE_QUARTER - self.pulse_width) / self.pulse_speed as u16
        } else {
            0
        };
        return step1;
    }

    pub fn get_loop_end(&self) -> u16 {
        return self.ticks;
    }

    pub fn reset(&mut self) {
        self.sample_step = 0;
        self.pulse_width_step = self.pulse_width;
        self.pulse_delay_step = self.pulse_delay;
        self.ticks_step = self.ticks;
    }

}

impl Iterator for PulseWave {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {

        if self.sample_step >= THREE_QUARTER {
            if self.ticks_step > 0 {
                self.ticks_step -= 1;
            }

            self.sample_step = 0;
            if self.pulse_delay_step == 0 {
                self.pulse_delay_step = self.pulse_delay;

                if self.increment {
                    if self.pulse_width_step + self.pulse_speed as u16 >= THREE_QUARTER {
                        self.increment = false;
                    } else {
                        self.pulse_width_step += self.pulse_speed as u16;
                    }
                } else {
                    if self.pulse_width_step - self.pulse_speed as u16  <= MIDDLE {
                        self.increment = true;
                    } else {
                        self.pulse_width_step -= self.pulse_speed as u16;
                    }
                }
                
            } else {
                self.pulse_delay_step -= 1;
            }
        }


        let sample = if self.sample_step < self.pulse_width_step {
            0
        } else {
            255
        };

        self.sample_step += 1;

        if self.ticks_step > 0 {
            Some(sample)
        } else {
            None
        }
    }
}
