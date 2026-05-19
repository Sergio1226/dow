use std::sync::Mutex;

pub static WITH_IMAGE:Mutex<bool> =Mutex::new(false);