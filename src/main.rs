#![windows_subsystem = "windows"]

use slint::slint;
use std::{process, sync::mpsc, thread};
use system_shutdown::shutdown;
use tray_item::{IconSource, TrayItem};

enum Message {
    Quit,
    Shutdown,
    OpenInfo,
}

fn main() {
    let port = String::from("1000");

    let window = MainWindow::new().unwrap().clone_strong();

    window.hide().unwrap();

    let mut tray = TrayItem::new("Remote Shutdown", IconSource::Resource("icon")).unwrap();

    let (tx, rx) = mpsc::sync_channel(100);

    let show_info_tx = tx.clone();
    tray.add_menu_item("Info", move || {
        show_info_tx.send(Message::OpenInfo).unwrap()
    })
    .unwrap();
    let quit_tx = tx.clone();
    tray.add_menu_item("Quit", move || quit_tx.send(Message::Quit).unwrap())
        .unwrap();

    thread::spawn(move || {
        rouille::start_server(format!("0.0.0.0:{}", port), move |_request| {
            tx.send(Message::Shutdown).unwrap();

            rouille::Response::text("Shutting down...")
        });
    });

    loop {
        match rx.recv() {
            Ok(Message::Quit) => {
                println!("Quit");
                process::exit(0);
            }
            Ok(Message::Shutdown) => match shutdown() {
                Ok(_) => {
                    println!("Shutting down...");
                }
                Err(err) => {
                    println!("Unable to shut down: {}", err);
                }
            },
            Ok(Message::OpenInfo) => {
                window.run().unwrap();
            }
            _ => {}
        }
    }
}

slint! {
    import { VerticalBox } from "std-widgets.slint";

    export component MainWindow inherits Window {
        title: "Remote Shutdown";

        height: 100px;
        width: 200px;

        VerticalBox {
            alignment: center;

            Image {
                source: @image-url("icon.png");
            }
            Text {
                text: "Remote shutdown v1.0.0";
                font-size: 14px;
                horizontal-alignment: center;
                font-weight: 600;
                padding-bottom: 30px;
            }
            Text {
                text: "by SuperRedstone";
                horizontal-alignment: center;
            }
            Text {
                text: "Port: 1000";
                horizontal-alignment: center;
            }
        }
    }
}
