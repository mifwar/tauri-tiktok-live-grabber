// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde_json::json;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use tiktoklive::core::live_client::TikTokLiveClient;
use tiktoklive::data::live_common::TikTokLiveSettings;
use tiktoklive::generated::events::TikTokLiveEvent;
use tiktoklive::TikTokLive;

use tauri::{Manager, Window};

static mut MAIN_WINDOW: Mutex<Option<Window>> = Mutex::new(None);

#[tauri::command]
fn start_thread(username: String) {
    println!("Starting thread for {}", username);
    thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async move {
            // Initialize the TikTok live client
            let client = TikTokLive::new_client(&username)
                .configure(configure)
                .on_event(handle_event)
                .build();

            // Connect to the TikTok live stream
            client.connect().await;

            // Check periodically if the disconnect flag is set
            println!("Connected to {} the stream", username);
            loop {
                // TODO: disconnect client when the flag is true
                thread::sleep(Duration::from_millis(100));
                // if *should_disconnect_clone.lock().unwrap() {
                //     client.disconnect();
                //     break;
                // }
            }
        });
    });
}

fn handle_event(_client: &TikTokLiveClient, event: &TikTokLiveEvent) {
    unsafe {
        let window_lock = MAIN_WINDOW.lock().unwrap();
        let main_window = window_lock.as_ref().unwrap();

        match event {
            TikTokLiveEvent::OnMember(join_event) => {
                println!("{} joined the stream", join_event.raw_data.user.nickname);
            }
            TikTokLiveEvent::OnChat(chat_event) => {
                let payload = json!({
                    "type": "chat",
                    "nickname": chat_event.raw_data.user.nickname,
                    "content": chat_event.raw_data.content
                });
                main_window.emit("tiktok-live-event", payload).unwrap();
            }
            TikTokLiveEvent::OnGift(gift_event) => {
                let payload = json!({
                    "type": "gift",
                    "nickname": gift_event.raw_data.user.nickname,
                    "gift_name": gift_event.raw_data.gift.name,
                    "gift_amount": gift_event.raw_data.gift.combo
                });
                main_window.emit("tiktok-live-event", payload).unwrap();
            }
            _ => {}
        }
    }
}

fn configure(settings: &mut TikTokLiveSettings) {
    settings.http_data.time_out = Duration::from_secs(12);
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![start_thread])
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();
            let mut window_lock = unsafe { MAIN_WINDOW.lock().unwrap() };
            *window_lock = Some(main_window);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
