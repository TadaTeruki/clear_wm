mod client;
mod clientmanager;
mod domain;

use client::Client;
use clientmanager::ClientManager;
use domain::traits::ClientManagerImpl;
use x11rb::connection::Connection;
use x11rb::protocol::{xproto::*, Event};
use x11rb::COPY_DEPTH_FROM_PARENT;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (conn, screen_num) = x11rb::connect(None).unwrap();
    let mut climgr:ClientManager<Client> = ClientManager::new(&conn);

    let screen = &conn.setup().roots[screen_num];
    let win_id = conn.generate_id()?;

    let values = CreateWindowAux::default()
        .event_mask(
            EventMask::EXPOSURE
                | EventMask::BUTTON_PRESS
                | EventMask::BUTTON_RELEASE
                | EventMask::POINTER_MOTION
        )
        .background_pixel(screen.white_pixel);
    conn.create_window(
        COPY_DEPTH_FROM_PARENT,
        win_id,
        screen.root,
        0,
        0,
        200,
        200,
        0,
        WindowClass::INPUT_OUTPUT,
        0,
        &values,
    )?;
    conn.map_window(win_id)?;
    conn.flush()?;

    climgr.bind(win_id);

    loop {
        let event = conn.wait_for_event()?;
        match event {
            Event::ButtonPress(event) => {
                println!("Button press event");
                let win_id = event.event;
                if let Some(client_id) = climgr.find_from_window(win_id) {
                    if let Some(_) = climgr.query_client(client_id) {
                        println!("This window is managed as an application window");
                    }
                }
            }
            Event::MotionNotify(_) => {
                println!("Motion notify event");
            }
            Event::ButtonRelease(_) => {
                println!("Button release event");
            }
            _ => {
                println!("Other event");
            }
        }
    }


    /* 
    let (conn, screen_num) = x11rb::connect(None).unwrap();
    let screen = &conn.setup().roots[screen_num];
    let win_id = conn.generate_id()?;

    let values = CreateWindowAux::default()
        .event_mask(
            EventMask::EXPOSURE
                | EventMask::BUTTON_PRESS
                | EventMask::BUTTON_RELEASE
                | EventMask::POINTER_MOTION
                | EventMask::SUBSTRUCTURE_NOTIFY,
        )
        .background_pixel(screen.white_pixel);
    conn.create_window(
        COPY_DEPTH_FROM_PARENT,
        win_id,
        screen.root,
        0,
        0,
        100,
        100,
        0,
        WindowClass::INPUT_OUTPUT,
        0,
        &values,
    )?;
    conn.map_window(win_id)?;
    conn.flush()?;
    loop {
        let event = conn.wait_for_event()?;
        match event {
            Event::Expose(_) => {
                println!("Expose event");
            }
            Event::ButtonPress(_) => {
                println!("Button press event");
            }
            Event::MotionNotify(_) => {
                println!("Motion notify event");
            }
            Event::ButtonRelease(_) => {
                println!("Button release event");
            }
            _ => {
                println!("Other event");
            }
        }
    }
    */
}
