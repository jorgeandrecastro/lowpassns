// Copyright (C) 2026 Jorge Andre Castro
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 2 or any later version.

#![no_std] // Suitable for embedded systems without standard library

//! # lowpassns
//!
//! Lightweight first-order low-pass filter for embedded systems.
//! Robust, `no_std`, Rust implementation for makers and MCU projects.

#[cfg(feature = "f32")]
pub type Float = f32;
#[cfg(not(feature = "f32"))]
pub type Float = f64;

/// Core first-order low-pass filter structure
#[derive(Debug, Clone, Copy)]
pub struct LowPass1 {
    /// Current filtered value
    y: Float,
    /// Time constant τ
    pub tau: Float,
}

impl LowPass1 {
    /// Create a new low-pass filter with initial value and time constant
    #[inline(always)]
    pub fn new(initial: Float, tau: Float) -> Self {
        Self { y: initial, tau }
    }

    /// Update the filter state with new input and timestep
    #[inline(always)]
    pub fn update(&mut self, input: Float, dt: Float) -> Float {
        if !dt.is_finite() || dt <= 0.0 {
            return self.y;
        }

        // Clamp dt for safety in embedded contexts
        let safe_dt = if dt < 1e-6 { 1e-6 } else if dt > 0.1 { 0.1 } else { dt };

        let alpha = safe_dt / (self.tau + safe_dt);
        self.y = self.y + alpha * (input - self.y);
        self.y
    }

    /// Reset filter state
    #[inline(always)]
    pub fn reset(&mut self, value: Float) {
        self.y = value;
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;

    #[test]
    fn test_lowpass_stability() {
        let mut filter = LowPass1::new(0.0, 0.1);
        let dt = 0.01;
        let mut y = 0.0;
        for _ in 0..200 { y = filter.update(1.0, dt); }
        assert!((y - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_zero_dt_integrity() {
        let mut filter = LowPass1::new(0.5, 0.1);
        let y = filter.update(1.0, 0.0);
        assert_eq!(y, 0.5);
    }
}