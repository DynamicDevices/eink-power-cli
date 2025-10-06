/*
 * E-ink Power CLI - Serial Communication
 * Copyright (c) 2025 Dynamic Devices Ltd
 * All rights reserved.
 */

//! Serial communication module for interfacing with the MCXC143VFM power controller

pub mod connection;
pub mod protocol;

pub use connection::Connection;
pub use protocol::Protocol;
