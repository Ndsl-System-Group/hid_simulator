use hidg::{Key, Result};
use std::{thread::sleep, time::Duration};

use crate::keyboard_helper::KeyboardHelper;
use crate::DRIVER_LETTER_VAR_NAME;

pub struct KeySimulator<'a, 'b> {
    helper: &'a mut KeyboardHelper<'b>,
}

impl<'a, 'b> KeySimulator<'a, 'b> {
    pub fn new(helper: &'a mut KeyboardHelper<'b>) -> KeySimulator<'a, 'b> {
        KeySimulator { helper }
    }

    pub fn open_powershell_admin(&mut self) -> Result<()> {
        self.helper.press_multi(&[Key::LeftMeta, Key::R])?; // LeftMeta即Win键
        sleep(Duration::from_millis(1000)); // 睡眠一段时间，否则Windows反应不过来，不会将Win+R的窗口作为焦点

        [
            Key::P,
            Key::O,
            Key::W,
            Key::E,
            Key::R,
            Key::S,
            Key::H,
            Key::E,
            Key::L,
            Key::L,
        ]
        .into_iter()
        .try_for_each(|key| self.helper.press_one(key))?;

        self.helper
            .press_multi(&[Key::LeftCtrl, Key::LeftShift, Key::Enter])?;

        sleep(Duration::from_millis(1000)); // 等待UAC窗口
        self.helper.press_one(Key::Left)?;
        sleep(Duration::from_millis(100));
        self.helper.press_one(Key::Enter)?;
        sleep(Duration::from_millis(1000)); // 必须等待一段时间，当PowerShell完全弹出后，它才开始接收输入

        Ok(())
    }

    // 必须在admin状态下运行，否则会报错
    pub fn whitelist_the_program(&mut self, exe_file_path: &str) -> Result<()> {
        let whitelist_cmd = format!(
            "Add-MpPreference -ExclusionPath '${{{DRIVER_LETTER_VAR_NAME}}}\\{exe_file_path}'"
        );
        self.helper.press_line(&whitelist_cmd)?;
        Ok(())
    }

    pub fn open_powershell(&mut self) -> Result<()> {
        self.helper.press_multi(&[Key::LeftMeta, Key::R])?; // LeftMeta即Win键
        sleep(Duration::from_millis(1000)); // 睡眠一段时间，否则Windows反应不过来，不会将Win+R的窗口作为焦点

        [
            Key::P,
            Key::O,
            Key::W,
            Key::E,
            Key::R,
            Key::S,
            Key::H,
            Key::E,
            Key::L,
            Key::L,
        ]
        .into_iter()
        .try_for_each(|key| self.helper.press_one(key))?;

        self.helper.press_one(Key::Enter)?;
        sleep(Duration::from_millis(1000)); // 必须等待一段时间，当PowerShell完全弹出后，它才开始接收输入

        Ok(())
    }

    /// 在主机powershell上执行：获取driver_name对应盘符的指令，盘符保存在powershell的DRIVER_LETTER_VAR_NAME环境变量中  
    /// 如果使用Get-Volume命令，则DRIVER_LETTER_VAR_NAME中不含盘符后的冒号  
    /// 另外两个命令返回的变量中，都含盘符的冒号
    pub fn get_driver_letter(&mut self, driver_name: &str) -> Result<()> {
        // Get-Volume命令延迟太高
        // let get_drive_letter_cmd = format!("${DRIVER_LETTER_VAR_NAME} = (Get-Volume -FileSystemLabel \"{driver_name}\").DriveLetter");

        // Get-CimInstance命令，Win8以后对Get-WmiObject的替代，性能最好
        // let get_drive_letter_cmd = format!(
        //     "${DRIVER_LETTER_VAR_NAME} = (Get-CimInstance -Query \"SELECT DeviceID FROM Win32_LogicalDisk WHERE VolumeName='{driver_name}' AND DriveType=2\").DeviceID"
        // );

        // Get-WmiObject命令，兼容性最好，从Win7开始可用
        let get_drive_letter_cmd = format!(
            "${DRIVER_LETTER_VAR_NAME} = (Get-WmiObject -Query \"SELECT DeviceID FROM Win32_LogicalDisk WHERE VolumeName='{driver_name}' AND DriveType=2\").DeviceID"
        );

        self.helper.press_line(&get_drive_letter_cmd)?;
        Ok(())
    }

    /// 必须在open_powershell之后调用, cmd末尾会自动进行回车
    pub fn run_command_in_powershell(&mut self, cmd: &str) -> Result<()> {
        self.helper.press_line(cmd)?;
        Ok(())
    }
}
