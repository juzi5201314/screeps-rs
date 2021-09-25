use std::convert::TryFrom;

use js_sys::Object;

use crate::{get_prop, EffEctId, PowerLevel, PowerId};

pub type Effects = Vec<Effect>;

pub enum Effect {
    Natural(NaturalEffect),
    Power(PowerEffect),
}

pub struct PowerEffect {
    effect: PowerId,
    level: PowerLevel,
    ticks_remaining: u32,
}

pub struct NaturalEffect {
    effect: EffEctId,
    ticks_remaining: u32,
}

impl From<Object> for Effect {
    fn from(obj: Object) -> Self {
        let level = get_prop(&obj, "level");
        if level.is_undefined() {
            // natural effect
            Effect::Natural(NaturalEffect {
                ticks_remaining: get_prop(&obj, "ticksRemaining").as_f64().unwrap() as u32,
                effect: EffEctId::try_from(get_prop(&obj, "effect").as_f64().unwrap() as u16)
                    .unwrap(),
            })
        } else {
            // power effect
            Effect::Power(PowerEffect {
                ticks_remaining: get_prop(&obj, "ticksRemaining").as_f64().unwrap() as u32,
                effect: PowerId::try_from(get_prop(&obj, "effect").as_f64().unwrap() as u8)
                    .unwrap(),
                level: PowerLevel::try_from(get_prop(&obj, "level").as_f64().unwrap() as u16)
                    .unwrap(),
            })
        }
    }
}
