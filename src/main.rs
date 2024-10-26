use reqwest::blocking;
use std::fs::File;
use std::io::{self, copy};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

use windows::Win32::UI::WindowsAndMessaging::{
    SystemParametersInfoW, SPIF_UPDATEINIFILE, SPI_SETDESKWALLPAPER,
};

const URL: &str = "https://bing.img.run/uhd.php";
const PATH: &str = "C:/Users/Administrator/Pictures/Today_Bing_Wallpaper.jpg";

fn main() {
    while TcpStream::connect("8.8.8.8:53").is_err() {
        thread::sleep(Duration::from_secs(3))
    }

    let response = blocking::get(URL).unwrap(); // 发送 GET 请求
    let mut file = File::create(PATH).unwrap(); // 创建文件
    let mut content = io::Cursor::new(response.bytes().unwrap()); // 获取内容并写入

    copy(&mut content, &mut file).unwrap(); // 将下载的内容写入文件

    let file_path_wide: Vec<u16> = PATH.encode_utf16().chain(std::iter::once(0)).collect();

    unsafe {
        SystemParametersInfoW(
            SPI_SETDESKWALLPAPER,
            0,
            Some(file_path_wide.as_ptr() as *mut _),
            SPIF_UPDATEINIFILE,
        )
        .unwrap()
    }
}
