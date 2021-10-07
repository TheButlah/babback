use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use tts::UtteranceId;

fn main() {
    let mut t = tts::Tts::default().expect("Could not create TTS");

    let kill_me = Arc::new(AtomicBool::new(false));
    let kill_me_clone = kill_me.clone();

    ctrlc::set_handler(move || kill_me_clone.store(true, Ordering::SeqCst))
        .expect("Failed to set ctrlc handle");

    let (speak_sender, speak_receiver) = std::sync::mpsc::sync_channel(1);

    if let Err(_) = t.set_volume(1.) {
        eprintln!("Failed to set volume");
    }

    if let Err(_) = t.set_rate(1.5) {
        eprintln!("Failed to set rate");
    }

    t.on_utterance_end(Some(Box::new(move |_| {
        println!("sending signal");
        speak_sender.send(()).unwrap()
    })))
    .expect("could not set callback");

    while !kill_me.load(Ordering::SeqCst) {
        std::thread::sleep(Duration::from_millis(100));
        t.speak("babb ack", false).expect("Could not speak");
        if let Err(_) = t.set_pitch(rand::random::<f32>() * 2.) {
            eprintln!("Failed to set pitch");
        }
        speak_receiver
            .recv_timeout(Duration::from_secs(5))
            .expect("Never received command to speak again");
    }
}
