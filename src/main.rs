#![cfg_attr(windows, windows_subsystem = "windows")]

use std::fs;
use std::process::Command;
use slint::{PhysicalPosition, SharedString, WindowPosition};
use std::process;
use winapi::um::winuser::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};

slint::include_modules!();

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = AppWindow::new()?;

    // Получаем размеры окна
    let window_size = app.window().size();
    let window_width = window_size.width;
    let window_height = window_size.height;

    // Получаем размеры экрана
    let screen_width = unsafe { GetSystemMetrics(SM_CXSCREEN) };
    let screen_height = unsafe { GetSystemMetrics(SM_CYSCREEN) };

    // Вычисляем позицию для центрирования окна
    let x = (screen_width as u32 - window_width) / 2;
    let y = (screen_height as u32 - window_height) / 2 + 100;

    // Устанавливаем позицию окна
    app.window().set_position(WindowPosition::from(PhysicalPosition::new(
        x as i32, y as i32
    )));

    app.on_button_ok(move |password: SharedString| {
        let uuid = fs::read_to_string("bin/uuid.txt").unwrap();
        match get_disk_letter(&uuid) {
            None => {}
            Some(a) => {
                println!("Dismounting {}", a);
                dismount(a);
            }
        }
        dismount_vera(&uuid);
        mount_vera(&uuid, &password.as_str());
        process::exit(0x0);
    });

    app.on_button_exit(move || {
        process::exit(0x0);
    });

    app.run()?;

    Ok(())
}

fn get_disk_letter(uuid: &str) -> Option<String> {
    let command =
        "Get-WmiObject -Class Win32_Volume | Where-Object { $_.DeviceID -like \"\\\\?\\Volume{".to_owned() +
            uuid +
            "}\\\" } | Select-Object -ExpandProperty DriveLetter";
    let output = Command::new("powershell")
        .arg("-WindowStyle")
        .arg("Hidden")
        .arg("-Command")
        .arg(command)
        .output()
        .expect("failed to execute command");

    if output.status.success() {
        let result = String::from_utf8_lossy(&output.stdout);
        return Some(
            result.trim().to_string().strip_suffix(":")?.to_owned()
        );
    }

    None
}

fn dismount(letter: String) {
    Command::new("mountvol")
        .arg(format!("{}:", letter))
        .arg("/p")
        .output()
        .expect("failed to execute command");
}

fn dismount_vera(uuid: &str) {
    Command::new("bin/VeraCrypt.exe")
        .arg("/d")
        .arg("/v")
        .arg("\"\\\\?\\Volume{".to_owned() + uuid + "}\\\"")
        .arg("/s")
        .arg("/q")
        .output()
        .expect("failed to execute command");
}

fn mount_vera(uuid: &str, password: &str) {
    Command::new("bin/VeraCrypt.exe")
        .arg("/hash")
        .arg("SHA-512")
        .arg("/p")
        .arg(password)
        .arg("/m")
        .arg("rm")
        .arg("/m")
        .arg("label=Crypted")
        .arg("/v")
        .arg("\\\\?\\Volume{".to_owned() + uuid + "}\\")
        .arg("/e")
        .arg("/l")
        .arg("S")
        .arg("/q")
        .spawn()
        .expect("failed to execute command");
}
