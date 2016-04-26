//! Contains definitions for wlc handle types.

extern crate libc;
use libc::{uintptr_t, c_char, c_void};

use super::pointer_to_string;
use super::types::{Geometry, ResizeEdge, Point, Size, ViewType, ViewState};

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Represents a handle to a wlc view.
pub struct WlcView(libc::uintptr_t);

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Represents a handle to a wlc output.
pub struct WlcOutput(libc::uintptr_t);

impl From<WlcView> for WlcOutput {
    fn from(view: WlcView) -> Self {
        WlcOutput(view.0)
    }
}

impl From<WlcOutput> for WlcView {
    fn from(output: WlcOutput) -> Self {
        WlcView(output.0)
    }
}


impl WlcOutput {
    /// Compatability/debugging function.
    ///
    /// wlc internally stores views and outputs under the same type.
    /// If for some reason a conversion between the two was required,
    /// this function could be called. If this is the case please submit
    /// a bug report.
    pub fn as_view(self) -> WlcView {
        return WlcView::from(self)
    }

    /// Gets user-specified data.
    ///
    /// # Unsafety
    /// The wlc implementation of this method uses `void*` pointers
    /// for raw C data. This function will internaly do a conversion
    /// between the input `T` and a `libc::c_void`.
    ///
    /// This is a highly unsafe conversion with no guarantees. As
    /// such, usage of these functions requires an understanding of
    /// what data they will have. Please review wlc's usage of these
    /// functions before attempting to use them yourself.
    pub unsafe fn get_user_data<T>(&self) -> &mut T {
        unimplemented!()
    }

    /// Sets user-specified data.
    ///
    /// # Unsafety
    /// The wlc implementation of this method uses `void*` pointers
    /// for raw C data. This function will internaly do a conversion
    /// between the input `T` and a `libc::c_void`.
    ///
    /// This is a highly unsafe conversion with no guarantees. As
    /// such, usage of these functions requires an understanding of
    /// what data they will have. Please review wlc's usage of these
    /// functions before attempting to use them yourself.
    pub unsafe fn set_user_data<T>(&self, data: &T) {
        unimplemented!()
    }

    /// Schedules output for rendering next frame.
    ///
    /// If the output was already scheduled, this is
    /// a no-op; if output is currently rendering,
    /// it will render immediately after.
    pub fn schedule_render(&self) {
    }

    /// Gets a list of the current outputs.
    pub fn list() -> Vec<WlcOutput> {
        unimplemented!()
    }

    /// Gets the currently focused output.
    pub fn focused() -> WlcOutput {
        unimplemented!()
    }

    /// Gets the name of the WlcOutput.
    ///
    /// Names are usually assigned in the format WLC-n,
    /// where the first output is WLC-1.
    pub fn get_name(&self) -> String {
        unimplemented!()
    }

    /// Gets the sleep status of the output.
    ///
    /// Returns `true` if the monitor is sleeping,
    /// such as having been set with `set_sleep`.
    pub fn get_sleep(&self) -> bool {
        unimplemented!()
    }

    /// Sets the sleep status of the output.
    pub fn set_sleep(&self, sleep: bool) {
    }

    /// Gets the output resolution in pixels.
    pub fn get_resolution(&self) -> &Size {
        unimplemented!()
    }

    /// Sets the resolution of the output.
    pub fn set_resolution(&self, size: Size) {
    }

    /// Get views in stack order.
    ///
    /// This is mainly useful for wm's who need another view stack for inplace sorting.
    /// For example tiling wms, may want to use this to keep their tiling order separated
    /// from floating order.
    /// This handles `wlc_output_get_views` and `wlc_output_get_mutable_views`.
    pub fn get_views(&self) -> Vec<WlcView> {
        unimplemented!()
    }

    /// Gets the mask of this output
    pub fn get_mask(&self) -> u32 {
        unimplemented!()
    }

    /// Sets the mask for this output
    pub fn set_mask(&self, mask: u32) {
    }

    /// # Deprecated
    /// This function is equivalent to simply calling get_views
    pub fn get_mutable_views(&self) -> Vec<WlcView> {
        unimplemented!()
    }

    /// Attempts to set the views of a given output.
    ///
    /// Returns true if the operation succeeded.
    pub fn set_views(&self, views: &mut Vec<&WlcView>) -> Result<(), &'static str> {
        unimplemented!()
    }

    /// Focuses compositor on a specific output.
    ///
    /// Pass in Option::None for no focus.
    pub fn focus(output: Option<&WlcOutput>) {
    }
}

impl WlcView {
    /// Compatability/debugging function.
    ///
    /// wlc internally stores views and outputs under the same type.
    /// If for some reason a conversion between the two was required,
    /// this function could be called. If this is the case please submit
    /// a bug report.
    pub fn as_output(self) -> WlcOutput {
        WlcOutput::from(self)
    }

    /// Returns a reference to the root window (desktop background).
    ///
    /// # Example
    /// ```
    /// use rustwlc::handle::WlcView;
    ///
    /// let view = WlcView::root();
    /// assert!(view.is_root());
    /// ```
    pub fn root() -> WlcView {
        WlcView(0)
    }

    /// Whether this view is the root window (desktop background).
    ///
    /// # Example
    /// ```rust
    /// use rustwlc::handle::WlcView;
    /// # // This example can be run because WlcView::root() does not interact with wlc
    /// let view = WlcView::root();
    /// assert!(view.is_root());
    /// ```
    #[inline]
    pub fn is_root(&self) -> bool {
        self.0 == 0
    }

    /// Whether this view is not the root window (desktop background).
    ///
    /// # Usage
    /// A convenience method, the opposite of `view.is_root()`.
    ///
    /// # Example
    /// ```rust
    /// use rustwlc::handle::WlcView;
    ///
    /// let view = WlcView::root();
    /// assert!(view.is_root());
    /// assert!(!view.is_window());
    /// ```
    #[inline]
    pub fn is_window(&self) -> bool {
        self.0 != 0
    }

    /// Gets user-specified data.
    ///
    /// # Unsafety
    /// The wlc implementation of this method uses `void*` pointers
    /// for raw C data. This function will internaly do a conversion
    /// between the input `T` and a `libc::c_void`.
    ///
    /// This is a highly unsafe conversion with no guarantees. As
    /// such, usage of these functions requires an understanding of
    /// what data they will have. Please review wlc's usage of these
    /// functions before attempting to use them yourself.
    pub unsafe fn get_user_data<T>(&self) -> &mut T {
        unimplemented!()
    }

    /// Sets user-specified data.
    ///
    /// # Unsafety
    /// The wlc implementation of this method uses `void*` pointers
    /// for raw C data. This function will internaly do a conversion
    /// between the input `T` and a `libc::c_void`.
    ///
    /// This is a highly unsafe conversion with no guarantees. As
    /// such, usage of these functions requires an understanding of
    /// what data they will have. Please review wlc's usage of these
    /// functions before attempting to use them yourself.
    pub unsafe fn set_user_data<T>(&self, data: &T) {
        unimplemented!()
    }

    /// Closes this view.
    ///
    /// For the main windows of most programs, this should close the program where applicable.
    ///
    /// # Behavior
    /// This function will not do anything if `view.is_root()`.
    pub fn close(&self) {
    }

    /// Gets the WlcOutput this view is currently part of.
    pub fn get_output(&self) -> WlcOutput {
        unimplemented!()
    }

    /// Sets the output that the view renders on.
    ///
    /// This may not be supported by wlc at this time.
    pub fn set_output(&self, output: &WlcOutput) {
    }

    /// Brings this view to focus.
    ///
    /// Can be called on `WlcView::root()` to lose all focus.
    pub fn focus(&self) {
    }

    /// Sends the view to the back of the compositor
    pub fn send_to_back(&self) {
    }

    /// Sends this view underneath another.
    pub fn send_below(&self, other: &WlcView) {
    }

    /// Brings this view above another.
    pub fn bring_above(&self, other: &WlcView) {
    }

    /// Brings this view to the front of the stack
    /// within its WlcOutput.
    pub fn bring_to_front(&self) {
    }

    // TODO Get masks enum working properly
    /// Gets the current visibilty bitmask for the view.
    pub fn get_mask(&self) -> u32 {
        unimplemented!()
    }

    // TODO Get masks enum working properly
    /// Sets the visibilty bitmask for the view.
    pub fn set_mask(&self, mask: u32) {
    }

    /// Gets the geometry of the view.
    pub fn get_geometry(&self) -> Option<&Geometry> {
        unimplemented!()
    }

    /// Gets the geometry of the view (that wlc displays).
    pub fn get_visible_geometry(&self) -> Geometry {
        unimplemented!()
    }

    /// Sets the geometry of the view.
    /// Set edges if geometry is caused by interactive resize.
    pub fn set_geometry(&self, edges: ResizeEdge, geometry: &Geometry) {
    }

    /// Gets the type bitfield of the curent view
    pub fn get_type(&self) -> ViewType {
        unimplemented!()
    }

    /// Set flag in the type field. Toggle indicates whether it is set.
    pub fn set_type(&self, view_type: ViewType, toggle: bool) {
    }

    // TODO get bitflags enums
    /// Get the current ViewState bitfield.
    pub fn get_state(&self) -> ViewState {
        unimplemented!()
    }

    /// Set ViewState bit. Toggle indicates whether it is set or not.
    pub fn set_state(&self, state: ViewState, toggle: bool) {
    }

    /// Gets parent view, returns `WlcView::root()` if this view has no parent.
    pub fn get_parent(&self) -> WlcView {
        unimplemented!()
    }

    /// Set the parent of this view.
    ///
    /// Call with `WlcView::root()` to make its parent the root window.
    pub fn set_parent(&self, parent: &WlcView) {
    }

    /// Get the title of the view
    pub fn get_title(&self) -> String {
        unimplemented!()
    }

    /// Get class (shell surface only).
    pub fn get_class(&self) -> String {
        unimplemented!()
    }

    /// Get app id (xdg-surface only).
    pub fn get_app_id(&self) -> String {
        unimplemented!()
    }
}
