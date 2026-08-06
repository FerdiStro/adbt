use crate::app::{App, DebugState};
use std::process::{Command, Output};
use std::sync::Arc;
use std::time::Duration;
use std::{io, thread};

//check for adb installation
pub fn check_deps() -> Option<String> {
    if adb_executer(String::new()).is_ok() {
        Some("adb installed".to_string())
    } else {
        None
    }
}

pub fn check_devices(_app: &mut App) {
    let device_id_list_clone = Arc::clone(&_app.device_module.device_id_list);
    let debug_clone = Arc::clone(&_app.debug);

    thread::spawn(move || {
        loop {
            if let Ok(output) = adb_executer("devices".to_owned()) {
                if output.status.success() {
                    if let Ok(mut list_lock) = device_id_list_clone.lock() {
                        list_lock.clear();

                        let output_read = String::from_utf8_lossy(&output.stdout);

                        let mut first_line = false;

                        output_read.lines().for_each(|_device| {
                            if first_line {
                                list_lock.push(_device.replace("device", "").to_owned())
                            }
                            first_line = true;
                        });

                        list_lock.pop();
                    }
                } else {
                    DebugState::write_error(debug_clone.lock(), "ERROR: problems to scan devices");
                }
            }

            thread::sleep(Duration::from_secs(2));
        }
    });
}

fn adb_executer(command: String) -> io::Result<Output> {
    Command::new("adb").arg(command).output()
}
