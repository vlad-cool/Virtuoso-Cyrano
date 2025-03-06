// struct PinLocation {
//     chip: u8,
//     line: u8,
// }

// pub enum MODES {
//     INPUT,
//     OUTPUT,
// }

// pub struct Gpio {
//     chips: Vec<libgpiod::chip::Chip>,
// }

// impl Gpio {
//     fn get_pin_by_phys_number(pin_number: u8) -> Option<PinLocation> {
//         match pin_number {
//             3 => Some(PinLocation { chip: 0, line: 12 }),
//             5 => Some(PinLocation { chip: 0, line: 11 }),
//             7 => Some(PinLocation { chip: 0, line: 6 }),
//             8 => Some(PinLocation { chip: 0, line: 13 }),
//             10 => Some(PinLocation { chip: 0, line: 14 }),
//             11 => Some(PinLocation { chip: 0, line: 1 }),
//             12 => Some(PinLocation { chip: 0, line: 16 }),
//             13 => Some(PinLocation { chip: 0, line: 0 }),
//             15 => Some(PinLocation { chip: 0, line: 3 }),
//             16 => Some(PinLocation { chip: 0, line: 15 }),
//             18 => Some(PinLocation { chip: 0, line: 68 }),
//             19 => Some(PinLocation { chip: 0, line: 64 }),
//             21 => Some(PinLocation { chip: 0, line: 65 }),
//             22 => Some(PinLocation { chip: 0, line: 2 }),
//             23 => Some(PinLocation { chip: 0, line: 66 }),
//             24 => Some(PinLocation { chip: 0, line: 67 }),
//             26 => Some(PinLocation { chip: 0, line: 71 }),
//             27 => Some(PinLocation { chip: 0, line: 19 }),
//             28 => Some(PinLocation { chip: 0, line: 18 }),
//             29 => Some(PinLocation { chip: 0, line: 7 }),
//             31 => Some(PinLocation { chip: 0, line: 8 }),
//             32 => Some(PinLocation { chip: 1, line: 2 }),
//             33 => Some(PinLocation { chip: 0, line: 9 }),
//             36 => Some(PinLocation { chip: 1, line: 4 }),
//             35 => Some(PinLocation { chip: 0, line: 10 }),
//             37 => Some(PinLocation { chip: 0, line: 17 }),
//             38 => Some(PinLocation { chip: 0, line: 21 }),
//             40 => Some(PinLocation { chip: 0, line: 20 }),
//             _ => None,
//         }
//     }

//     pub fn new() -> Self {
//         chips = vec![
//             libgpiod::chip::Chip::open("/dev/gpiochip0"),
//             libgpiod::chip::Chip::open("/dev/gpiochip1"),
//         ]
//     }

//     pub fn pin_mode(&self, pins: vec<u8>, mode: MODES) -> Result<()> {
//         // self.PIN_MAP[pins].Ok
//     }

//     pub fn pins_mode(&self, pins: vec<u8>, mode: MODES) -> Result<()> {
//         // for pin in pins {}
//         // Ok
//     }
// }
