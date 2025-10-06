/*
 * E-ink Power CLI - Power Management
 * Copyright (c) 2025 Dynamic Devices Ltd
 * All rights reserved.
 */

//! Power management module for battery monitoring and power control

pub mod battery;
pub mod control;

pub use battery::BatteryMonitor;
pub use control::PowerController;
