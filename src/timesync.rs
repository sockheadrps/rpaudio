use std::sync::Arc;
use std::time::Duration;

// Assuming you have a custom AudioSink struct instead of rodio::Sink
use crate::AudioSink;

#[derive(Clone)]
pub struct TimeSyncEffect {
    pub time: Duration,
    pub action: Arc<dyn Fn(&AudioSink) + Send + Sync>,
    pub applied: bool,
}

impl TimeSyncEffect {
    pub fn new<F>(time: Duration, action: F) -> Self
    where
        F: Fn(&AudioSink) + Send + Sync + 'static,
    {
        Self {
            time,
            action: Arc::new(action),
            applied: false,
        }
    }

    pub fn apply(&mut self, audiosink: &AudioSink) {
        eprintln!("Applying effect at {:?}", self.time);
        if !self.applied {
            (self.action)(audiosink);
            self.applied = true;
        }
    }
}

#[derive(Clone)]
pub struct EffectActionData {
    pub action_type: ActionType,
    pub params: Vec<f32>,
}

#[derive(Clone)]
pub enum ActionType {
    Fade,
    ChangeSpeed,
}

#[derive(Clone)]
pub struct TimeSyncEffects {
    pub effects: Vec<TimeSyncEffect>,
}

impl TimeSyncEffects {
    pub fn new() -> Self {
        Self {
            effects: Vec::new(),
        }
    }

    pub fn add_effect(&mut self, effect: TimeSyncEffect) {
        self.effects.push(effect);
    }

    pub fn apply_due_effects(&mut self, audiosink: &AudioSink, current_time: Duration) {
        for effect in &mut self.effects {
            if current_time >= effect.time && !effect.applied {
              eprintln!("Applying effect at {:?}", effect.time);
                effect.apply(audiosink);
            }
        }
    }
}
