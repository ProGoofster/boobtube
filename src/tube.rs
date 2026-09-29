// Tube pin interface and aging state.
// Units: volts (V), amperes (A), ohms, coulombs (C), hours.

/// Gun indices for the per-gun arrays.
pub const RED: usize = 0;
pub const GREEN: usize = 1;
pub const BLUE: usize = 2;
pub const NUM_GUNS: usize = 3;

/// Voltages applied to the tube pins, one set per simulation sample.
#[derive(Debug, Clone, Copy, Default)]
pub struct TubeInputs {
    pub v_heater: f32,
    pub v_g2: f32,
    pub v_g3: f32,
    pub v_anode: f32,
    pub v_k: [f32; NUM_GUNS],
    pub v_g1: [f32; NUM_GUNS],
    pub v_yoke_h: f32,
    pub v_yoke_v: f32,
}

/// Currents drawn by the tube through its pins, one set per simulation sample.
#[derive(Debug, Clone, Copy, Default)]
pub struct TubeOutputs {
    pub i_k: [f32; NUM_GUNS],
    pub i_g2: f32,
    pub i_anode: f32,
    pub i_yoke_h: f32,
    pub i_yoke_v: f32,
}

/// Aging and fault state for a single gun.
#[derive(Debug, Clone, Copy)]
pub struct GunAge {
    /// 1.0 = new, 0.0 = dead.
    pub emission_activity: f32,
    /// Added to the cutoff voltage (V).
    pub cutoff_offset_v: f32,
    /// Multiplier on beam current gain. 1.0 = new.
    pub gain_factor: f32,
    /// Heater to cathode leakage (ohms). INFINITY = healthy.
    pub hk_leak_ohms: f32,
    /// G1 to cathode leakage (ohms). INFINITY = healthy.
    pub g1k_leak_ohms: f32,
    pub heater_open: bool,
    /// Accumulated cathode charge (C). Drives the wear model.
    pub charge_drawn_c: f64,
    /// Accumulated time at operating temperature.
    pub hours_hot: f64,
}

impl Default for GunAge {
    fn default() -> Self {
        Self {
            emission_activity: 1.0,
            cutoff_offset_v: 0.0,
            gain_factor: 1.0,
            hk_leak_ohms: f32::INFINITY,
            g1k_leak_ohms: f32::INFINITY,
            heater_open: false,
            charge_drawn_c: 0.0,
            hours_hot: 0.0,
        }
    }
}

/// Aging state for the whole tube.
/// The per-pixel phosphor charge map lives on the GPU as a texture.
#[derive(Debug, Clone, Copy)]
pub struct TubeAge {
    pub guns: [GunAge; NUM_GUNS],
    /// 1.0 = good vacuum.
    pub vacuum_quality: f32,
    /// Added spot growth. 0.0 = new.
    pub focus_degradation: f32,
}

impl Default for TubeAge {
    fn default() -> Self {
        Self {
            guns: [GunAge::default(); NUM_GUNS],
            vacuum_quality: 1.0,
            focus_degradation: 0.0,
        }
    }
}