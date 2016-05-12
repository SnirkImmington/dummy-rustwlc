//! Register wlc callbacks to events.
//!
//! See individual methods for callback details.
//!
//! # wlc Example
//! ```no_run
//! use rustwlc;
//! use rustwlc::callback;
//! use rustwlc::WlcView;
//!
//! // An example callback function
//! // See the various functions in this module for more information
//! extern "C" fn view_focus_callback(view: WlcView, focused: bool) {
//!     println!("A view came into focus!");
//! }
//!
//! // Set a default log callback
//! rustwlc::log_set_default_handler();
//!
//! // Register some callbacks
//! callback::view_focus(view_focus_callback);
//! // ... and additional callbacks
//!
//! // The only thing your code should do before init2 is register callbacks
//! // and log handlers.
//! let run_wlc = rustwlc::init2()
//!     .expect("Unable to initialize wlc!");
//!
//! run_wlc();
//! ```

use super::types::*;
use super::handle::{WlcOutput, WlcView};

/// Callback invoked when an output is created.
/// Return `true` to allow the output to exist.
///
/// # Example
/// ```rust
/// use rustwlc::WlcOutput;
///
/// extern fn on_output_created(output: WlcOutput) -> bool {
///     println!("Output {} ({:?}) was created", output.get_name(), output);
///     return true;
/// }
/// # fn main() { }
/// ```
pub fn output_created(callback: extern "C" fn(output: WlcOutput) -> bool) {
    
}

/// Callback invoked when an output is destroyed.
///
/// # Example
/// ```rust
/// use rustwlc::WlcOutput;
///
/// extern fn output_destroyed(output: WlcOutput) {
///     println!("Goodbye, {:?}", output);
/// }
/// # fn main() { }
/// ```
pub fn output_destroyed(callback: extern "C" fn(output: WlcOutput)) {
    
}

/// Callback invoked when an output gains focus.
///
/// # Example
/// ```rust
/// use rustwlc::WlcOutput;
///
/// extern fn output_focus(output: WlcOutput, focused: bool) {
///     println!("Output {} {} focus", output.get_name(),
///              if focused { "gained" } else { "lost" });
/// }
/// # fn main() { }
/// ```
pub fn output_focus(callback: extern "C" fn(output: WlcOutput, focused: bool)) {
    
}

/// Callback invoked when an output's resolution changes.
///
/// # Example
/// ```rust
/// use rustwlc::WlcOutput;
/// use rustwlc::Size;
///
/// extern fn output_resolution(output: WlcOutput,
///                             old_size: &Size, new_size: &Size) {
///     println!("Output {} went from {} to {}",
///              output.get_name(), old_size, new_size);
/// }
/// # fn main() { }
/// ```
pub fn output_resolution(callback: extern "C" fn(output: WlcOutput,
                                                 old_size: &Size,
                                                 new_size: &Size)) {
    
}

/// Output context created. This generally happens on a tty switch.
pub fn output_context_destroyed(cb: extern "C" fn(output: WlcOutput)) {
    
}

/// Output context destroyed
pub fn output_context_created(cb: extern "C" fn(output: WlcOutput)) {
    
}

/// Callback invoked pre-render for an output.
pub fn output_render_pre(callback: extern "C" fn(output: WlcOutput)) {
    
}

/// Callback invoked post-render for an output.
pub fn output_render_post(callback: extern "C" fn(output: WlcOutput)) {
    
}

/// Callback invoked when a view is created.
/// Return `true` to allow the view to be created.
///
/// When a new view is created, the following should probably be applied:
/// * Set the view's mask to the output's mask
/// * Focus the view
/// * Bring the view to the front
///
/// # Example
/// ```rust
/// use rustwlc::WlcView;
///
/// extern fn view_created(view: WlcView) -> bool {
///     println!("View \"{}\" was created ({:?})", view.get_class(), view);
///     view.set_mask(view.get_output().get_mask());
///     view.bring_to_front();
///     view.focus();
///     return true;
/// }
/// # fn main() { }
/// ```
pub fn view_created(callback: extern "C" fn(view: WlcView) -> bool) {
    
}

/// Callback invoked when a view is destroyed.
///
/// When a view is destroyed, it's a good idea to shift focus to
/// some other view, i.e. the last one used.
///
/// # Example
/// ```rust
/// use rustwlc::WlcView;
///
/// extern fn view_destroyed(view: WlcView) {
///     println!("Goodbye, {:?}", view);
/// }
/// # fn main() { }
/// ```
pub fn view_destroyed(callback: extern "C" fn(view: WlcView)) {
    
}

/// Callback invoked when a view is focused.
///
/// The view's `ViewState::VIEW_ACTIVATED` bit should be set to true here.
///
/// # Example
/// ```rust
/// use rustwlc::WlcView;
/// // The bitflags constants need to be imported manually.
/// use rustwlc::VIEW_ACTIVATED;
///
/// extern fn view_focus(view: WlcView, focused: bool) {
///     println!("View {:?} is {} focus, updating...",
///               view, if focused { "in" } else { "out of" });
///     view.set_state(VIEW_ACTIVATED, focused);
/// }
/// ```
pub fn view_focus(callback: extern "C" fn(handle: WlcView, focused: bool)) {
    
}

/// Callback invoked when a view switches outputs.
///
/// Moving views between outputs is unsupported in wlc at the time of writing.
/// Wayland mandates each output have its own memory buffer so it may take wlc
/// some time before this is implemented.
pub fn view_move_to_output(callback: extern "C" fn(view: WlcView,
                                                   old_output: WlcOutput,
                                                   new_output: WlcOutput)) {
    
}

/// Callback invoked when a view requests geometry.
pub fn view_request_geometry(callback: extern "C" fn(handle: WlcView,
                                                     geometry: &Geometry)) {
    
}

/// Callback invoked when a view requests a `ViewState`.
pub fn view_request_state(callback: extern "C" fn(current: WlcView,
                                                  state: ViewState,
                                                  handled: bool)) {
    
}

/// Callback invoked when a view requests a move.
pub fn view_request_move(callback: extern "C" fn(handle: WlcView,
                                                 destination: &Point)) {
    
}

/// Callback invoked when a view requests a resize.
pub fn view_request_resize(callback: extern "C" fn(handle: WlcView,
                                                   edge: ResizeEdge,
                                                   location: &Point)) {
    
}

/// Callback invoked pre-view-render.
pub fn view_render_pre(callback: extern "C" fn(view: WlcView)) {
    
}

/// Callback invoked post-view-render.
pub fn view_render_post(callback: extern "C" fn(view: WlcView)) {
    
}

/// Callback invoked on keypresses.
/// Return `true` to block the press from the view.
///
/// # Arguments
/// The first `u32` is a timestamp, the second is the key code. The view may be
/// the root window.
///
/// Proper values for `key` can be found in `input.h` or a similar library/crate
/// - see wlc documentation on the subject, it may not support your keyboard
/// layout at the moment.
///
/// # Example
/// ```rust
/// use rustwlc::WlcView;
/// use rustwlc::{KeyboardModifiers, KeyState};
///
/// extern fn keyboard_key(view: WlcView, time: u32, mods: &KeyboardModifiers,
///                        key: u32, state: KeyState) -> bool {
///     println!("Key {} {:?} on {:?} at {} with modifiers {:?}",
///              key, view, state, time, mods);
///     return false;
/// }
/// # fn main() { }
/// ```
pub fn keyboard_key(callback: extern "C" fn(view: WlcView, time: u32,
                                            mods: &KeyboardModifiers, key: u32,
                                            state: KeyState) -> bool) {
    
}

/// Callback invoked on mouse clicks.
/// Return `true` to block the click from the view.
///
/// # Arguments
/// The first u32 is a timestamp, the second is the button code.
/// The view may be the root window. Proper values for `button`
/// can be found in `input.h` or a similar library/crate.
///
/// # Example
/// ```rust
/// use rustwlc::WlcView;
/// use rustwlc::{KeyboardModifiers, ButtonState, Point};
///
/// extern fn pointer_button(view: WlcView, time: u32,
///                          mods: &KeyboardModifiers, button: u32,
///                          state: ButtonState, point: &Point) -> bool {
///     println!("Button {} {:?} at {} at {} in {:?}, keyboard mods: {:?}",
///              button, state, time, point, view, mods);
///     return false;
/// }
/// # fn main() { }
/// ```
pub fn pointer_button(callback: extern "C" fn(view: WlcView, time: u32,
                                              mods: &KeyboardModifiers,
                                              button: u32, state: ButtonState,
                                              point: &Point) -> bool) {
    
}

/// Callback invoked on mouse scroll.
/// Return `true` to block the scroll from the view.
///
/// # Arguments
/// * view: The WlcView (or output root) that was scrolled in
/// * time: Timestamp
/// * mods: Current pressed keyboard modifiers
/// * axis: Which direction the scroll was in
/// * amount: The first argument seems to be either 10 or -10 depending on
/// up/down (or right/left if `axis == ScrollAxis::Horizontal`).
/// The second one, when tested on a standard laptop trackpad, seems to be
/// a double slightly above zero.
pub fn pointer_scroll(callback: extern "C" fn(view: WlcView, time: u32,
                                              mods: &KeyboardModifiers,
                                              axis: ScrollAxis,
                                              amount: [f64; 2]) -> bool) {
    
}

/// Callback invoked on pointer motion.
/// Return `true` to block the motion from the view.
///
/// `rustwlc::input::pointer::set_position`
/// must be invoked to actually move the cursor!
///
/// # Example
/// ```rust
/// use rustwlc::WlcView;
/// use rustwlc::Point;
/// use rustwlc::input::pointer;
///
/// extern fn pointer_motion(view: WlcView, time: u32, point: &Point) -> bool {
///     println!("Pointer was moved to {} in {:?} at {}", point, view, time);
///     // This is very important.
///     pointer::set_position(point);
///     return false;
/// }
/// # fn main() { }
/// ```
pub fn pointer_motion(callback: extern "C" fn(view: WlcView, time: u32,
                                              point: &Point) -> bool) {
    
}

/// Callback invoked on touchscreen touch.
/// Return `true` to block the touch from the view.
///
/// # Arguments
/// * `mods`: Which keyboard modifiers are being pressed during the event
/// * `touch`: What kind of event it is (a touch down, a frame being made,
/// a touch release). In the case of `TouchType::Frame`, `slot` and `point`
/// will both be zero.
/// * `slot`: Which finger - in cases of multiple touches down - is causing
/// the event
/// * `point`: Where the touch event happened
pub fn touch(callback: extern "C" fn(handle: WlcView, time: u32,
                                     mods: &KeyboardModifiers, touch: TouchType,
                                     slot: i32, point: &Point) -> bool) {
    
}

/// Callback invoked by wlc after `rustwlc::init` is called.
pub fn compositor_ready(callback: extern "C" fn()) {
    
}

/// Callback invoked by wlc when a compositor is terminating
pub fn compositor_terminate(callback: extern "C" fn()) {
    
}
