# lowpassns

[![License: GPL-2.0-or-later](https://img.shields.io/badge/License-GPL%202.0%2B-blue.svg)](https://www.gnu.org/licenses/gpl-2.0)  
[![No Std](https://img.shields.io/badge/no_std-compatible-green.svg)](https://docs.rs/lowpassns)  
[![Maintenance](https://img.shields.io/badge/Maintenance-Actively--developed-brightgreen.svg)](https://github.com/jorgeandrecastro/lowpassns)  

## Lightweight `no_std` Low-Pass Filter for Embedded Systems

**lowpassns** is a small, efficient, and robust **first-order low-pass filter** crate for `no_std` Rust environments. Designed for embedded systems like RP2040 or STM32, it smooths sensor readings or signals in real-time while being memory- and CPU-efficient. 


# Update 
#![forbid(unsafe_code)] for safety and opt-level = 3 for speed 

---

## Table of Contents
- [Features](#features)  
- [Installation](#installation)  
- [Quickstart](#quickstart)  
- [API Reference](#api-reference)  
- [Performance & Optimization](#performance--optimization)  
- [Testing](#testing)  
- [License](#license)  

---

## 🚀 Features
- ✅ **Pure `no_std`**: Zero standard library dependencies, perfect for bare-metal and RTOS.  
- ⚡ **Flexible Floating-Point**: Default `f64`, optional `f32` feature for memory-constrained devices.  
- 🔧 **Lightweight**: Minimal stack usage, O(1) per update.  
- 🛡️ **Stable & Safe**: Handles zero/negative dt safely, clamps tiny dt for embedded portability.  
- 📈 **Smooth Filtering**: Simple first-order low-pass behavior for sensor smoothing or signal conditioning.  
- ⛑️ **Resettable State**: Filter value can be reset at any time.  

---

## 🛠️ Installation

Add to your `Cargo.toml`:

````toml
[dependencies]
lowpassns = lowpassns = "0.2.0"


Enable f32 for constrained MCUs:

lowpassns = { version="0.2.0", features = ["f32"] }
```` 
Build with optimizations:

cargo build --release

# 🚀 Quickstart
````rust
use lowpassns::LowPass1;

fn main() {
    // Initial value = 0.0, time constant τ = 0.1 s
    let mut filter = LowPass1::new(0.0, 0.1);

    let dt = 0.01; // 10 ms timestep

    for i in 0..100 {
        let input = 1.0; // example sensor input
        let output = filter.update(input, dt);
        // output smoothly approaches input
    }

    filter.reset(0.0); // Reset the filter state
}
````
# 📚 API Reference
| Method   | Signature                                             | Description                              |
| -------- | ----------------------------------------------------- | ---------------------------------------- |
| `new`    | `LowPass1::new(initial: Float, tau: Float)`           | Creates a new low-pass filter.           |
| `update` | `update(&mut self, input: Float, dt: Float) -> Float` | Updates the filter state for a timestep. |
| `reset`  | `reset(&mut self, value: Float)`                      | Resets the filter state.                 |


Type Float = f64 (default) or f32 via feature flag.

# ⚡ Performance & Optimization
Speed opt-level = 3  
CPU cost: constant time O(1) per update
Stack-only, zero allocations
Ideal for 100 Hz – 10 kHz control loops on MCUs
🧪 Testing

Includes tests for:

Step response stability
Zero/negative dt handling
Reset functionality

Run:

cargo test -- --nocapture
# ⚖️ License

GPL-2.0-or-later © 2026 Jorge Andre Castro.

Free to use, modify, and distribute. Any derivative works must also be GPL-2.0-or-later.