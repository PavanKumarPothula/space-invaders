use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, Triangle},
};
use esp_idf_hal as hal;
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use hal::{
    gpio::{self, GpioPin},
    i2c::{self, MasterPins},
    prelude::*,
};
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

// //OLED Struct

// pub struct oled {
//     display: Ssd1306,
//     yoffset: i32,
//     style: PrimitiveStyle,
// }

// impl oled {
//     fn new(
//         peripherals: Peripherals,
//         sda: gpio::OutputPin,
//         scl: gpio::OutputPin,
//         yoffset: i32,
//     ) -> Self {
//         //I2C initialization
//         let i2c = peripherals.i2c0;
//         let config = <i2c::config::MasterConfig as Default>::default().baudrate(100.kHz().into());

//         //Display Interface Init
//         let interface = I2CDisplayInterface::new(
//             i2c::Master::<i2c::I2C0, _, _>::new(i2c, i2c::MasterPins { sda, scl }, config).unwrap(),
//         );
//         let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
//             .into_buffered_graphics_mode();
//         display.init().unwrap();
//         display.clear();

//         let style = PrimitiveStyleBuilder::new()
//             .stroke_width(1)
//             .stroke_color(BinaryColor::On)
//             .build();
//         Self {
//             display,
//             yoffset,
//             style,
//         }
//     }

//     fn draw_rectangle(&mut self, top_left: (i32, i32), size: (i32, i32)) {
//         Rectangle::new(
//             Point::new(top_left.0, top_left.1),
//             Size::new(size.0, size.1),
//         )
//         .into_styled(self.style)
//         .draw(&mut self.display)
//         .unwrap();
//         display.flush().unwrap();
//     }

//     fn draw_triangle(&mut self, v1: (i32, i32), v2: (i32, i32), v3: (i32, i32)) {
//         // triangle
//         Triangle::new(
//             Point::new(v1.0, v1.1),
//             Point::new(v2.0, v2.1),
//             Point::new(v3.0, v3.1),
//         )
//         .into_styled(self.style)
//         .draw(&mut self.display)
//         .unwrap();
//         display.flush().unwrap();
//     }
// }
fn main() {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    //Take peripherals else halt
    let peripherals = Peripherals::take().unwrap();
    //I2C initialization
    let i2c = peripherals.i2c0;
    let sda = peripherals.pins.gpio21;
    let scl = peripherals.pins.gpio22;
    print_type_of(sda);
    print_type_of(scl);
    let config = <i2c::config::MasterConfig as Default>::default().baudrate(100.kHz().into());

    //Display Interface Init
    let interface = I2CDisplayInterface::new(
        i2c::Master::<i2c::I2C0, _, _>::new(i2c, i2c::MasterPins { sda, scl }, config).unwrap(),
    );
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    display.init().unwrap();
    display.clear();

    //Render Graphics
    let yoffset = 20;

    let style = PrimitiveStyleBuilder::new()
        .stroke_width(1)
        .stroke_color(BinaryColor::On)
        .build();

    Rectangle::new(Point::new(0, 0), Size::new(127, 63))
        .into_styled(style)
        .draw(&mut display)
        .unwrap();

    // triangle
    Triangle::new(
        Point::new(16, 16 + yoffset),
        Point::new(16 + 16, 16 + yoffset),
        Point::new(16 + 8, yoffset),
    )
    .into_styled(style)
    .draw(&mut display)
    .unwrap();

    display.flush().unwrap();
    loop {}
}
