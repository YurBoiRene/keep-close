#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use arduino_hal::port::{
    mode::{Input, PullUp},
    Pin, PinOps,
};
use panic_halt as _;

use avr_device::interrupt;
use core::{
    cell::{Cell, RefCell},
    time::Duration,
};

/* spellchecker: disable */
use ufmt::{uwrite, uwriteln};

mod millis;
use millis::{millis, millis_init};

mod buzzer;
use buzzer::Buzzer;

mod button;
use button::Button;

type Console = arduino_hal::hal::usart::Usart0<arduino_hal::DefaultClock>;
static CONSOLE: interrupt::Mutex<RefCell<Option<Console>>> =
    interrupt::Mutex::new(RefCell::new(None));
/* spellchecker: enable */

macro_rules! print {
    ($($t:tt)*) => {
        interrupt::free(
            |cs| {
                if let Some(console) = CONSOLE.borrow(cs).borrow_mut().as_mut() {
                    let _ = uwrite!(console, $($t)*);
                }
            },
        )
    };
}

macro_rules! println {
    ($($t:tt)*) => {
        interrupt::free(
            |cs| {
                if let Some(console) = CONSOLE.borrow(cs).borrow_mut().as_mut() {
                    let _ = uwriteln!(console, $($t)*);
                }
            },
        )
    };
}

fn put_console(console: Console) {
    interrupt::free(|cs| {
        *CONSOLE.borrow(cs).borrow_mut() = Some(console);
    })
}

struct Door<T: PinOps> {
    reed: Pin<Input<PullUp>, T>,
}

impl<T: PinOps> Door<T> {
    fn is_open(&self) -> bool {
        self.reed.is_high()
    }
}

const OPEN_ALARM_MILLIS: Duration = Duration::from_secs(30);
#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let serial = arduino_hal::default_serial!(dp, pins, 57600);
    put_console(serial);
    let buzzer = Buzzer::new(dp.TC1, pins.d9);

    let mut button = Button::new(pins.d4);
    let alarm_enabled = Cell::new(true);

    millis_init(dp.TC0);

    unsafe { avr_device::interrupt::enable() };

    let mut led = pins.d13.into_output();
    let reed = pins.d2.into_pull_up_input();

    println!("keep-close started.");
    let door = Door { reed };
    let mut last_open = millis();
    let mut last_open_status = false;
    loop {
        button.on_press(|| {
            let enabled = alarm_enabled.get();

            alarm_enabled.set(!enabled);

            print!("Alarm set to ");
            if alarm_enabled.get() {
                println!("enabled");
            } else {
                println!("disabled");
            }
        });

        let cur_open_status = door.is_open();
        if cur_open_status {
            led.set_high();
            if !last_open_status {
                println!("Door just opened.");

                last_open = millis();
            }

            let cur_time = millis();
            if cur_time - last_open > OPEN_ALARM_MILLIS {
                println!("OPEN TOO LONG!!");
                if alarm_enabled.get() {
                    let freq = 1000;
                    buzzer.set_freq(freq);
                } else {
                    buzzer.off();
                }
            }
        } else {
            led.set_low();
            buzzer.off();
        }
        last_open_status = cur_open_status;
    }
}
