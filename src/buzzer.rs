use arduino_hal::{
    hal::port::PB1,
    port::{
        mode::{Floating, Input},
        Pin,
    },
};

// TODO use something like pitch_calc to pass frequency, pitches, etc.
// pitch_calc requires std :(

// Theoretically a prescaler of 8 would get us
// a better resolution on pitch however, it causes
// problems and does not produce the right pitch.
// Minimum freq: ocr1a=65,535, freq=15 Hz
// Maximum freq: ocr1a=3, freq=250,000 Hz
// Human hearing range between 20-20,000 Hz
// https://www.desmos.com/calculator/m80qodskjs
// For now, 64.
const PRESCALER: u32 = 64;

const BUZZER_VARIANT: u8 = 15;

type Hz = u32;
type PWMPin = Pin<Input<Floating>, PB1>;

pub struct Buzzer {
    tc1: arduino_hal::pac::TC1,
}

impl Buzzer {
    pub fn new(tc1: arduino_hal::pac::TC1, d9: PWMPin) -> Self {
        let rtn = Buzzer { tc1 };
        rtn.init(d9);
        rtn
    }

    fn init(&self, d9: PWMPin) {
        // Configure the timer in Fast PWM with TOP=OCC1A
        // TC1 gives us 16-bits to work with
        // Also enable PWM on pin 9
        // See also enable/disable
        // Section 15.9 of ATmega328P data sheet
        let initial_freq = freq_to_occr1a(1000);

        self.tc1.ocr1a.write(|w| w.bits(initial_freq));
        self.tc1.tccr1b.write(|w| {
            match PRESCALER {
                8 => w.cs1().prescale_8(),
                64 => w.cs1().prescale_64(),
                256 => w.cs1().prescale_256(),
                1024 => w.cs1().prescale_1024(),
                _ => panic!(),
            }
            .wgm1()
            .variant(BUZZER_VARIANT)
        });

        d9.into_output();

        self.off();
    }

    pub fn set_freq(&self, freq: Hz) {
        let new_output = freq_to_occr1a(freq);
        self.tc1.ocr1a.write(|w| w.bits(new_output));
        self.enable();
    }

    pub fn off(&self) {
        self.disable();
    }

    fn disable(&self) {
        self.tc1
            .tccr1a
            .write(|w| w.wgm1().variant(BUZZER_VARIANT).com1a().disconnected());
    }
    fn enable(&self) {
        self.tc1
            .tccr1a
            .write(|w| w.wgm1().variant(BUZZER_VARIANT).com1a().match_toggle());
    }
}

fn freq_to_occr1a(freq: Hz) -> u16 {
    // freq = 16000000 / (2 * PRESCALER * (occr1a + 1))
    // Section 15.9.3 of ATmega328P data sheet
    ((16000000 / (freq * 2 * PRESCALER)) - 1)
        .try_into()
        .unwrap_or(u16::MAX)
}
