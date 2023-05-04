//! FuwaNe Foundation - Event

extern crate serde;
use serde::{ Serialize, Deserialize };


#[cfg(target_arch="wasm32")]
pub mod constants {
    pub const SAMPLE_RATE: usize = 48000;
    pub const AUDIO_FRAME_RATE: usize = 50;
    pub const MONO_FRAME_SIZE: usize = SAMPLE_RATE / AUDIO_FRAME_RATE;
    pub const STEREO_FRAME_SIZE: usize = 2 * MONO_FRAME_SIZE;
    pub const STEREO_FRAME_BYTE_SIZE: usize = STEREO_FRAME_SIZE * std::mem::size_of::<f32>();
}

#[cfg(not(target_arch="wasm32"))]
pub mod constants {
    pub use songbird::constants::{
        SAMPLE_RATE_RAW as SAMPLE_RATE, AUDIO_FRAME_RATE,
        MONO_FRAME_SIZE, STEREO_FRAME_SIZE, STEREO_FRAME_BYTE_SIZE
    };
}


pub mod communication {
    use std::sync::Arc;
    
    use tokio::sync::{ mpsc::{ Sender, Receiver, channel }, Mutex as AioMutex };


    pub struct Channel<T> {
        pub tx: Sender<T>,
        pub rx: Arc<AioMutex<Receiver<T>>>
    }

    pub fn create_lazy_channel<T>() -> Channel<T> {
        let (tx, rx) = channel(128); Channel {
            tx: tx, rx: Arc::new(AioMutex::new(rx))
        }
    }


    #[cfg(test)]
    mod tests {
        use super::create_lazy_channel;

        #[test]
        fn test_lazy_channel() {
            let c = create_lazy_channel::<bool>();
            c.tx.blocking_send(true).unwrap();
            assert!(c.rx.blocking_lock().blocking_recv().unwrap());
        }
    }
}


#[derive(Default, Serialize, Deserialize)]
pub struct Context {}