#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

extern crate socket;

use socket::{UDP_DATA, TCP_DATA, SCTP_DATA};
use std::{thread, time};
use socket::{Service, ServiceKind};
use tauri::Window;


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![tcp_event, tcp, tcp_send, udp_event, udp, udp_send, sctp_event, sctp, sctp_send])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String
}

#[tauri::command]
fn tcp_event(window: Window) {
    std::thread::spawn(move || {
        loop {
            unsafe{
                let content = &mut TCP_DATA;
                if content.len() != 0 {
                    window.emit("tcp-event", Payload { message: content.to_string() }).unwrap();
                    content.clear();
                    thread::sleep(time::Duration::from_millis(100));
                }
            }
        }
    });
}

#[tauri::command(async)]
fn tcp(port: u16) -> Result<String, String> {
    let server = Service::form(port, ServiceKind::TCP);
    
    match server.listener() {
        Ok(()) => Ok(String::from("Success")),
        Err(e) => {
            println!("{}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
fn tcp_send(port: u16, content: String) -> Result<String, String> {
    let server = Service::form(port, ServiceKind::TCP);
    
    match server.send_file(content.as_bytes()) {
        Ok(()) => Ok(String::from("Success")),
        Err(e) => {
            println!("{}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
fn udp_event(window: Window) {
    std::thread::spawn(move || {
        loop {
            unsafe{
                let content = &mut UDP_DATA;
                if content.len() != 0 {
                    window.emit("udp-event", Payload { message: content.to_string() }).unwrap();
                    content.clear();
                    thread::sleep(time::Duration::from_millis(100));
                }
            }
        }
    });
}

#[tauri::command(async)]
fn udp(port: u16) -> Result<String, String> {
    let server = Service::form(port, ServiceKind::UDP);
    
    match server.listener() {
        Ok(()) => Ok(String::from("Success")),
        Err(e) => {
            println!("{}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
fn udp_send(port: u16, content: String) -> Result<String, String> {
    let server = Service::form(port, ServiceKind::UDP);
    
    match server.send_file(content.as_bytes()) {
        Ok(()) => Ok(String::from("Success")),
        Err(e) => {
            println!("{}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
fn sctp_event(window: Window) {
    std::thread::spawn(move || {
        loop {
            unsafe{
                let content = &mut SCTP_DATA;
                if content.len() != 0 {
                    window.emit("sctp-event", Payload { message: content.to_string() }).unwrap();
                    content.clear();
                    thread::sleep(time::Duration::from_millis(100));
                }
            }
        }
    });
}

#[tauri::command(async)]
fn sctp(port: u16) -> Result<String, String> {
    let server = Service::form(port, ServiceKind::SCTP);
    
    match server.listener() {
        Ok(()) => Ok(String::from("Success")),
        Err(e) => {
            println!("{}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
fn sctp_send(port: u16, content: String) -> Result<String, String> {
    let server = Service::form(port, ServiceKind::SCTP);
    
    match server.send_file(content.as_bytes()) {
        Ok(()) => Ok(String::from("Success")),
        Err(e) => {
            println!("{}", e);
            Err(e.to_string())
        }
    }
}
