//! # Termip
//! A terminal manipulation library providing a pure function interface for cross platform
//! applications.
//!
//! ## Goals
//! Termip aims to provide a consistent and thin abstraction over platform specific sys calls and
//! terminal handling.

/// A module providing event structures for handling input from a terminal
pub mod events;

/// A module providing styling functionality for the terminal
pub mod style;

/// A module providing utilities to manipulate the terminal
pub mod terminal;

