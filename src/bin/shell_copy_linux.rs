use std::{thread::sleep, time::Duration};

use hid_simulator::KeyboardHelper;
use hidg::{Class, Device, Key, Keyboard};
use simple_logger::SimpleLogger;

fn main() -> std::io::Result<()> {
    SimpleLogger::new().init().unwrap();

    let mut device = Device::<Keyboard>::open(0)?;
    let mut input = Keyboard.input();
    let mut key_helper = KeyboardHelper::new(&mut device, &mut input);

    key_helper.press_multi(&[Key::LeftCtrl, Key::LeftAlt, Key::T])?; // open terminal
    sleep(Duration::from_millis(500));
    key_helper.press_line("nohup bash /media/$USER/MYUDISK/copy_linux.sh > /dev/null 2>&1 &")?;
    key_helper.press_line("exit")?;

    Ok(())
}
