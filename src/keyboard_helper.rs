use std::{collections::HashMap, str::FromStr};

use hidg::{Device, Key, Keyboard, KeyboardInput, Result};
use lazy_static::lazy_static;
use log::warn;

lazy_static! {
    static ref SPECIAL_KEY_MAPPER: HashMap<char, Vec<Key>> = HashMap::from([
        ('-', vec![Key::Minus]),
        ('_', vec![Key::LeftShift, Key::Minus]),
        ('.', vec![Key::Dot]),
        ('"', vec![Key::LeftShift, Key::Apostrophe]),
        ('\'', vec![Key::Apostrophe]),
        ('/', vec![Key::Slash]),
        ('\\', vec![Key::BackSlash]),
        (' ', vec![Key::Space]),
        (':', vec![Key::LeftShift, Key::Semicolon]),
        ('$', vec![Key::LeftShift, Key::Num4]),
        ('(', vec![Key::LeftShift, Key::Num9]),
        (')', vec![Key::LeftShift, Key::Num0]),
        ('{', vec![Key::LeftShift, Key::LeftBrace]),
        ('}', vec![Key::LeftShift, Key::RightBrace]),
        ('=', vec![Key::Equal]),
        (';', vec![Key::Semicolon]),
        ('>', vec![Key::LeftShift, Key::Dot]),
        ('&', vec![Key::LeftShift, Key::Num7])
    ]);
}

pub struct KeyboardHelper<'a> {
    device: &'a mut Device<Keyboard>,
    input: &'a mut KeyboardInput,
}

impl<'a> KeyboardHelper<'a> {
    pub fn new(
        device: &'a mut Device<Keyboard>,
        input: &'a mut KeyboardInput,
    ) -> KeyboardHelper<'a> {
        KeyboardHelper { device, input }
    }

    /// 'press_one' 按下单个按键并松开
    pub fn press_one(&mut self, key: Key) -> Result<()> {
        self.input.press_key(key);
        self.device.input(self.input)?;

        // 不需要sleep，参见kernel drivers/usb/gadget/function/f_hid.c:f_hidg_write
        // 内核一次只传输一个hid报告，多余的报告睡眠等待。当首个报告被主机消费后才能继续发。
        // sleep(Duration::from_micros(10));

        self.input.release_key(key);
        self.device.input(self.input)?;
        Ok(())
    }

    /// `press_multi` 同时按下多个按键，然后一起松开，例如Win + R  
    /// Keys中不可超过多于6个的按键
    pub fn press_multi(&mut self, keys: &[Key]) -> Result<()> {
        if keys.len() > 6 {
            warn!("cannot press more than 6 keys one time, press will be truncated to the first 6 keys!");
        }
        keys.iter().for_each(|key| self.input.press_key(*key));
        self.device.input(self.input)?;
        // sleep(Duration::from_micros(10));
        keys.iter().for_each(|key| self.input.release_key(*key));
        self.device.input(self.input)?;
        Ok(())
    }

    /// 主机处于非大写锁定状态，也就是没有按下CAPS LOCK键，press_char才能正常工作，否则输入的大小写正好相反
    pub fn press_char(&mut self, ch: char) -> Result<()> {
        if ch.is_digit(10) || ch.is_lowercase() {
            self.press_one(Self::ch_to_key(ch))?;
        } else if ch.is_uppercase() {
            self.press_multi(&[Key::LeftShift, Self::ch_to_key(ch.to_ascii_lowercase())])?;
        } else {
            match SPECIAL_KEY_MAPPER.get(&ch) {
                Some(key_vec) => {
                    self.press_multi(key_vec)?;
                }
                None => {
                    panic!("Unsupport char {}", ch);
                }
            }
        }
        Ok(())
    }

    /// 发送cmd，cmd中的每个字符单独按下
    pub fn press_cmd(&mut self, cmd: &str) -> Result<()> {
        for ch in cmd.chars() {
            self.press_char(ch)?;
        }
        Ok(())
    }

    /// 发送cmd+ENTER，cmd中的每个字符单独按下
    pub fn press_line(&mut self, cmd: &str) -> Result<()> {
        self.press_cmd(cmd)?;
        self.press_one(Key::Enter)?;
        Ok(())
    }

    fn ch_to_key(ch: char) -> Key {
        let ch_str = ch.to_string();
        Key::from_str(&ch_str)
            .map_err(|_e| {
                panic!("unexpected error in Key::from_str when transform {}", ch);
            })
            .unwrap()
    }
}
