use crate::cpu;
use wasm_bindgen::prelude::*;
use web_sys::{AudioContext, GainNode, OscillatorNode, OscillatorType};
pub struct FmOsc {
    _ctx: AudioContext,
    _primary: OscillatorNode,
    pub gain: GainNode,
    _fm_gain: GainNode,
    _fm_osc: OscillatorNode,
    _fm_freq_ratio: f32,
    _fm_gain_ratio: f32,
}

impl FmOsc {
    pub fn new() -> Result<FmOsc, JsValue> {
        let _ctx = web_sys::AudioContext::new()?;

        let _primary = _ctx.create_oscillator()?;
        let _fm_osc = _ctx.create_oscillator()?;
        let gain = _ctx.create_gain()?;
        let _fm_gain = _ctx.create_gain()?;

        _primary.set_type(OscillatorType::Sine);
        _primary.frequency().set_value(880.0);
        gain.gain().set_value(0.0); // starts muted
        _fm_gain.gain().set_value(800.0);
        _fm_osc.set_type(OscillatorType::Sine);
        _fm_osc.frequency().set_value(50.0);

        _primary.connect_with_audio_node(&gain)?;
        gain.connect_with_audio_node(&_ctx.destination())?;
        _fm_osc.connect_with_audio_node(&_fm_gain)?;
        _fm_gain.connect_with_audio_param(&_primary.frequency())?;

        _primary.start()?;
        _fm_osc.start()?;

        Ok(FmOsc {
            _ctx,
            _primary,
            gain,
            _fm_gain,
            _fm_osc,
            _fm_freq_ratio: 10.0,
            _fm_gain_ratio: 10.0,
        })
    }
}

pub fn sound(emulator: &mut cpu::Emulator, audio_context: &FmOsc) {
    match emulator.sound_timer {
        0 => audio_context.gain.gain().set_value(0.0),
        _ => {
            audio_context.gain.gain().set_value(0.04);
            emulator.sound_timer -= 1;
        }
    }
}
