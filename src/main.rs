use std::{env, thread, time};

use hyprland::{
    data::{Client, CursorPosition},
    dispatch::{Dispatch, DispatchType},
    shared::{HyprData, HyprDataActiveOptional},
};

fn main() {
    let mut args = env::args();
    args.next();
    let classes: Vec<String> = args.collect();
    loop {
        if let Ok(Some(active_client)) = Client::get_active() {
            if classes
                .iter()
                .fold(false, |acc, x| acc || active_client.class.contains(x))
            {
                let cursor = {
                    let position = CursorPosition::get().unwrap();
                    (position.x, position.y)
                };
                let cursor_adjusted = (
                    num::clamp(
                        cursor.0,
                        (active_client.at.0 + 1).into(),
                        (active_client.at.0 + active_client.size.0 - 1).into(),
                    ),
                    num::clamp(
                        cursor.1,
                        (active_client.at.1 + 1).into(),
                        (active_client.at.1 + active_client.size.1 - 1).into(),
                    ),
                );
                if cursor_adjusted != cursor {
                    Dispatch::call(DispatchType::MoveCursor(
                        cursor_adjusted.0,
                        cursor_adjusted.1,
                    ))
                    .unwrap();
                }
            }
        }
        thread::sleep(time::Duration::from_micros(300));
    }
}
