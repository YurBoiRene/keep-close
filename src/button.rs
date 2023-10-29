use core::time::Duration;

use arduino_hal::port::{
    mode::{self, Input, PullUp},
    Pin, PinOps,
};

use crate::millis;

type Pressed = bool;

pub struct Button<PIN> {
    pin: Pin<Input<PullUp>, PIN>,
    last_change: Duration,
    last_state: Pressed,
    on_press_triggered: bool,
}

const PRESS_TIME: Duration = Duration::from_millis(100);

impl<PIN: PinOps> Button<PIN> {
    pub fn new<MODE: mode::Io>(pin: Pin<MODE, PIN>) -> Self {
        let pin = pin.into_pull_up_input();
        Button {
            pin,
            last_change: Duration::default(),
            last_state: Pressed::default(),
            on_press_triggered: bool::default(),
        }
    }

    pub fn is_pressed(&self) -> Pressed {
        self.pin.is_low()
    }

    // Calls callback once after button is pressed.
    // Will only call once per press.
    pub fn on_press<T: Fn()>(&mut self, callback: T) {
        let state = self.is_pressed();
        let time = millis();
        if self.last_state != state {
            self.last_change = time;
            self.last_state = state;
            self.on_press_triggered = false;
        } else if (time - self.last_change > PRESS_TIME) && state && !self.on_press_triggered {
            self.on_press_triggered = true;
            callback();
        }
    }
}
