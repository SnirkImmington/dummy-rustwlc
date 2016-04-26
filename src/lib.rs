//! Module defining main wlc functions.

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

extern crate libc;

#[macro_use]
extern crate bitflags;

use std::ffi;

pub mod handle;
pub mod callback;
pub mod types;
pub mod input;
pub mod wayland;
pub mod xkb;

pub use types::*;

// Log Handler hack
static mut rust_logging_fn: fn(_type: LogType, string: &str) = default_log_callback;

/// Query backend wlc is using.
///
/// # Results
/// * None: Unknown backend type
/// * DRM: "Direct Rendering Manager" - running on tty
/// * X11: Running inside an X server
pub fn get_backend_type() -> BackendType {
    unimplemented!()
}

/// Initialize wlc's callbacks and logger with a `WlcInterface`.
///
/// # Deprecated
/// wlc has deprecated this callback interface. They offer a new API with a
/// series of methods found in the `callback` module
///
/// To initialize wlc, register your callbacks with the functions described in
/// the `callbacks` module, and the logger using `log_set_handler` or
/// `log_set_default_handler`. Then call `init2()`.
///
/// # Permissions
/// If a compositor is initialized from the tty using suid or logind, it will
/// drop extra permissions after a call to `init()` or `init2()`. It is strongly
/// recommended to delay code which is not registering callbacks until after
/// this call.
///
/// # wlc Example
/// ```no_run
/// use rustwlc;
/// use rustwlc::callback;
/// use rustwlc::handle::WlcView;
///
/// // An example callback function
/// // See the various functions in the callback module for more information
/// extern "C" fn view_focus_callback(view: WlcView, focused: bool) {
///     println!("A view came into focus!");
/// }
///
/// // Set a default log callback
/// rustwlc::log_set_default_handler();
///
/// // Register some callbacks
/// callback::view_focus(view_focus_callback);
/// // ... and additional callbacks
///
/// // The only thing your code should do before init2 is register callbacks
/// // and log handlers.
/// let run_wlc = rustwlc::init()
///     .expect("Unable to initialize wlc!");
///
/// run_wlc();
/// ```
pub fn init() -> Option<fn() -> ()> {
    Some(run_wlc)
}

/// Deprecated alias to init().
///
/// When wlc went to 0.0.1, they added an argumentless init2
/// to replace the old init that took a WlcInterface. Now,
/// init2 has been renamed init and init is removed.
pub fn init2() -> Option<fn() -> ()> {
    init()
}

/// Runs wlc's event loop.
///
/// The initialize functions will return this function in an Option.
/// Only then can it be called to being wlc's main event loop.
fn run_wlc() {
    println!("wlc has been run!");
}

/// Halts execution of wlc.
pub fn terminate() {
    panic!("Called rustwlc::terminate!")
}

/// Registers a C callback for wlc logging.
///
/// Note that `rustwlc::log_set_default_handler()` will register a simple callback
/// that will print the type and text to the console.
///
/// # Parameters
/// The `handler` callback has two parameters:
/// * `type`: The `LogType` of the message being printed.
/// * `text`: The text to be logged, currently in C form. One may call `rustwlc::pointer_to_string`
/// to convert it to a Rust String.
///
/// # Safety
/// The callback function (like other callbacks in `rustwlc`) must be marked as extern as it is called
/// from C code.
///
/// In addition, `unsafe` will be required to convert the text into a Rust String.
pub fn log_set_handler(handler: extern "C" fn(type_: LogType, text: *const libc::c_char)) {
}

/// Registers a Rust callback for wlc logging.

/// This is a nice convenience function that should be used in place of
/// `log_set_handler`. That way you can just pass a safe Rust `&str`
/// and not depend on libc`.
pub fn log_set_rust_handler(handler: fn(type_: LogType, text: &str)) {
}

fn default_log_callback(log_type: LogType, text: &str) {
    println!("wlc [{:?}] {}", log_type, text);
}

/// Sets the wlc log callback to a simple function that prints to console.
///
/// Not calling any `log_set_handler` will have no logging, use this or
/// `log_set_handler` with a callback to use wlc logging.
///
/// # Example
/// ```no_run
/// use rustwlc;
///
/// // An example where only the default log handler is registered
/// rustwlc::log_set_default_handler();
///
/// if let Some(run_wlc) = rustwlc::init2() {
///      run_wlc();
/// }
/// else {
///     panic!("Unable to initialize wlc!");
/// }
/// ```
pub fn log_set_default_handler() {
    log_set_rust_handler(default_log_callback);
}

/// Unsafe strings conversion function.
///
/// Converts a `*const libc::c_char` to an owned `String`.
/// Useful for log callbacks.
///
/// # Example
/// Standard usage may be for the log callbacks.
/// ```rust
/// extern "C" fn default_log_callback(log_type: LogType, text: *const libc::c_char) {
///     let string = unsafe { pointer_to_string(text) };
///     println!("wlc [{:?}]: {}", log_type, string);
/// }
/// ```
pub unsafe fn pointer_to_string(pointer: *const libc::c_char) -> String {
    unimplemented!()
}
