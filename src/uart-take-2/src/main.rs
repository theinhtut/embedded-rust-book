#![no_main]
#![no_std]

use cortex_m::asm::delay;
use cortex_m_rt::entry;
use stm32f3xx_hal as hal;

use hal::pac;
use hal::serial::Serial;
use hal::prelude::*;
use hal::usb::{Peripheral, UsbBus};

use usb_device::prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC};

use panic_halt as _;

use cortex_m_semihosting::{hprintln};

#[entry]
fn main() -> ! {
    let x = 1;
    let y = 2;

    let mut dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(8.mhz()).freeze(&mut flash.acr);
    // let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpioc = dp.GPIOC.split(&mut rcc.ahb);

    let tx = gpioc.pc4.into_af7(&mut gpioc.moder, &mut gpioc.afrl);
    let rx = gpioc.pc5.into_af7(&mut gpioc.moder, &mut gpioc.afrl);

    let usart1 = Serial::usart1(
        dp.USART1,
        (tx,rx),
        115200.bps(),
        clocks,
        &mut rcc.apb2,
    );

    loop {
        let usart_1 = unsafe { &*hal::stm32::USART1::ptr() };

        // Writing 'X'
        usart_1.tdr.write(|w| unsafe { w.bits(0x58) });
        
        // Wait until there's data available
        while usart_1.isr.read().rxne().bit_is_clear() {}

        // Retrieve the data
        let b = usart_1.rdr.read().rdr().bits() as u8;
        hprintln!("read: {:02X}", b).unwrap();
    }
}

// fn main() -> ! {
//     let x = 1;
//     let y = 2;

//     let mut dp = pac::Peripherals::take().unwrap();

//     let mut flash = dp.FLASH.constrain();
//     let mut rcc = dp.RCC.constrain();

//     let clocks = rcc
//         .cfgr
//         .use_hse(8.mhz())
//         .sysclk(48.mhz())
//         .pclk1(24.mhz())
//         .pclk2(24.mhz())
//         .freeze(&mut flash.acr);

//     assert!(clocks.usbclk_valid());

//     let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);
//     let mut gpioc = dp.GPIOC.split(&mut rcc.ahb);

//     let mut usb_dp = gpioa
//         .pa12
//         .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

//     usb_dp.set_low().ok();
//     delay(clocks.sysclk().0 / 100);

//     let tx = gpioc.pc4.into_af7(&mut gpioc.moder, &mut gpioc.afrl);
//     let rx = gpioc.pc5.into_af7(&mut gpioc.moder, &mut gpioc.afrl);

//     let usb_dm = gpioa.pa11.into_af14(&mut gpioa.moder, &mut gpioa.afrh);
//     let usb_dp = usb_dp.into_af14(&mut gpioa.moder, &mut gpioa.afrh);

//     let usb = Peripheral {
//         usb: dp.USB,
//         pin_dm: usb_dm,
//         pin_dp: usb_dp,
//     };
//     let usb_bus = UsbBus::new(usb);

//     let mut serial = SerialPort::new(&usb_bus);

//     let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
//         .manufacturer("Fake company")
//         .product("Serial port")
//         .serial_number("TEST")
//         .device_class(USB_CLASS_CDC)
//         .build();

//     let uart1 = dp
//         .USART1
//         .uart(tx, rx, 115_200.bps(), clocks, &mut rcc.apb2)
//         .unwrap();

//     writeln!(usart1, "Hello\r").unwrap();

//     loop {
//         if !usb_dev.poll(&mut [&mut serial]) {
//             continue;
//         }

//         let mut buf = [0u8; 64];

//         match serial.read(&mut buf[..]) {
//             Ok(count) => {
//                 // count bytes were read to &buf[..count]
//                 hprintln!("Hello, world!").unwrap();
//             }
//             Err(UsbError::WouldBlock) => {} // No data received
//             Err(err) => {}                  // An error occurred
//         };

//         match serial.write(&[0x3a, 0x29]) {
//             Ok(count) => {
//                 let x = count;
//                 hprintln!("write").unwrap();
//             }
//             Err(UsbError::WouldBlock) => {} // No data could be written (buffers full)
//             Err(err) => {}                  // An error occurred
//         };
//     }
// }
