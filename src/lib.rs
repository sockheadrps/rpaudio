use audiosink::AudioSink;
use audioqueue::AudioChannel;
use mixer::ChannelManager;
use pyo3::prelude::*;
use timesync::{ActionType, ChangeSpeed, FadeIn, FadeOut};
mod audioqueue;
mod exceptions;
mod exmetadata;
mod audiosink;
mod mixer;
mod timesync;
mod audiotimer;
use crate::exceptions::EffectConflictException;
pub use exmetadata::MetaData;


#[pymodule(name = "rpaudio")]
mod rpaudio {
    use super::*;

    #[pymodule_export]
    use super::AudioSink;
    #[pymodule_export]
    use super::AudioChannel;
    #[pymodule_export]
    use super::ChannelManager;
    #[pymodule_export]
    use super::ActionType;


    #[pymodule]
    mod effects {
        #[pymodule_export]
        use super::FadeIn;
        #[pymodule_export]
        use super::FadeOut;
        #[pymodule_export]
        use super::ChangeSpeed;
    }

    #[pymodule]
    #[pyo3(name = "exceptions")]
    mod rpaudio_exceptions {
        #[pymodule_export]
        use super::EffectConflictException;
    }
}