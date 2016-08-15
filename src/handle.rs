//! Contains definitions for wlc handle types.
//!
//! # Implementations
//! - **Debug**: pointer-prints the underlying `uintptr_t` handle
//! - **Eq, Ord**: compare the underlying `uintptr_t` handle
//! - **Clone**: View handles can safely be cloned.

use libc::{uintptr_t};

use super::types::{Geometry, ResizeEdge, Point, Size, ViewType, ViewState};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Represents a handle to a wlc view.
///
pub struct WlcView(uintptr_t);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Represents a handle to a wlc output.
pub struct WlcOutput(uintptr_t);

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

static ZERO_RES: Size = Size { w: 0, h: 0 };

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

    /// Create a dummy WlcOutput for testing purposes.
    ///
    /// # Unsafety
    /// The following operations on a dummy WlcOutput will cause crashes:
    ///
    /// - `WlcOutput::focused` when wlc is not running
    /// - `WlcOutput::list` when wlc is not running
    /// - `WlcOutput::set_resolution` on a dummy output
    ///
    /// In addition, `WlcOutput::set_views` will return an error.
    ///
    /// All other methods can be used on dummy outputs.
    ///
    /// # Example
    /// ```rust
    /// # use rustwlc::WlcOutput;
    /// let output = WlcOutput::dummy(0u32);
    /// let output2 = WlcOutput::dummy(1u32);
    /// assert!(output < output2);
    /// assert!(output != output2);
    /// ```
    pub fn dummy(code: u32) -> WlcOutput {
        WlcOutput(code as uintptr_t)
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
    ///
    /// # Safety
    /// This function will crash the program if run when wlc is not running.
    pub fn list() -> Vec<WlcOutput> {
        unimplemented!()
    }

    /// Gets the currently focused output.
    ///
    /// # Safety
    /// This function will crash the program if run when wlc is not running.
    pub fn focused() -> WlcOutput {
        unimplemented!()
    }

    /// Gets the name of the WlcOutput.
    ///
    /// Names are usually assigned in the format WLC-n,
    /// where the first output is WLC-1.
    pub fn get_name(&self) -> String {
        "".to_string()
    }

    /// Gets the sleep status of the output.
    ///
    /// Returns `true` if the monitor is sleeping,
    /// such as having been set with `set_sleep`.
    pub fn get_sleep(&self) -> bool {
        false
    }

    /// Sets the sleep status of the output.
    pub fn set_sleep(&self, sleep: bool) {
    }

    /// Gets the output resolution in pixels.
    pub fn get_resolution(&self) -> Option<Size> {
        Some(ZERO_RES)
    }

    /// Sets the resolution of the output.
    ///
    /// # Safety
    /// This method will crash the program if use when wlc is not running.
    pub fn set_resolution(&self, size: Size, scaling: u32) {
    }

    /// Get views in stack order.
    ///
    /// This is mainly useful for wm's who need another view stack for inplace sorting.
    /// For example tiling wms, may want to use this to keep their tiling order separated
    /// from floating order.
    /// This handles `wlc_output_get_views` and `wlc_output_get_mutable_views`.
    pub fn get_views(&self) -> Vec<WlcView> {
        Vec::new()
    }

    /// Gets the mask of this output
    pub fn get_mask(&self) -> u32 {
        0
    }

    /// Sets the mask for this output
    pub fn set_mask(&self, mask: u32) {
    }

    /// # Deprecated
    /// This function is equivalent to simply calling get_views
    pub fn get_mutable_views(&self) -> Vec<WlcView> {
        self.get_views()
    }

    /// Attempts to set the views of a given output.
    ///
    /// Returns success if operation succeeded. An error will be returned
    /// if something went wrong or if wlc isn't running.
    pub fn set_views(&self, views: &mut Vec<WlcView>) -> Result<(), &'static str> {
        Err("Currently running dummy-rustwlc")
    }

    /// Focuses compositor on a specific output.
    ///
    /// Pass in Option::None for no focus.
    pub fn focus(output: Option<WlcOutput>) {
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

    /// Create a dummy WlcView for testing purposes.
    ///
    /// # Unsafety
    /// The following methods on views may crash the program:
    ///
    /// - `WlcView::focus` if wlc is not running
    /// - `WlcView::send_to_back` if wlc is not running
    /// - `WlcView::send_below` if wlc is not running
    /// - `WlcView::bring_above` if wlc is not running
    /// - `WlcView::bring_to_font` if wlc is not running
    ///
    /// All other methods can be used on dummy views.
    ///
    /// # Note
    /// `WlcView::root()` is equivalent to `WlcView::dummy(0)`.
    ///
    /// ```rust
    /// # use rustwlc::WlcView;
    /// assert!(WlcView::root() == WlcView::dummy(0))
    /// ```
    /// # Example
    /// ```rust
    /// # use rustwlc::WlcView;
    /// let view = WlcView::dummy(0u32);
    /// let view2 = WlcView::dummy(1u32);
    /// assert!(view < view2);
    /// assert!(view != view2);
    /// ```
    pub fn dummy(code: u32) -> WlcView {
        WlcView(code as uintptr_t)
    }

    /// Returns a reference to the root window (desktop background).
    ///
    /// # Example
    /// ```
    /// # use rustwlc::WlcView;
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
    /// # use rustwlc::WlcView;
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
    /// # use rustwlc::WlcView;
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
        WlcOutput::dummy(0)
    }

    /// Sets the output that the view renders on.
    ///
    /// This may not be supported by wlc at this time.
    pub fn set_output(&self, output: WlcOutput) {
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
    pub fn send_below(&self, other: WlcView) {
    }

    /// Brings this view above another.
    pub fn bring_above(&self, other: WlcView) {
    }

    /// Brings this view to the front of the stack
    /// within its WlcOutput.
    pub fn bring_to_front(&self) {
    }

    // TODO Get masks enum working properly
    /// Gets the current visibilty bitmask for the view.
    pub fn get_mask(&self) -> u32 {
        0
    }

    // TODO Get masks enum working properly
    /// Sets the visibilty bitmask for the view.
    pub fn set_mask(&self, mask: u32) {
    }

    /// Gets the geometry of the view.
    pub fn get_geometry(&self) -> Option<Geometry> {
        Some(Geometry {
            origin: Point { x: 0, y: 0},
            size:   Size  { w: 0, h: 0}
        })
    }

    /// Gets the geometry of the view (that wlc displays).
    pub fn get_visible_geometry(&self) -> Geometry {
        let geo = Geometry { origin: Point { x: 0, y: 0}, size: Size { w: 0, h: 0 }};
        return geo;
    }

    /// Sets the geometry of the view.
    ///
    /// Set edges if geometry is caused by interactive resize.
    pub fn set_geometry(&self, edges: ResizeEdge, geometry: Geometry) {
    }

    /// Gets the type bitfield of the curent view
    pub fn get_type(&self) -> ViewType {
        ViewType::empty()
    }

    /// Set flag in the type field. Toggle indicates whether it is set.
    pub fn set_type(&self, view_type: ViewType, toggle: bool) {
    }

    // TODO get bitflags enums
    /// Get the current ViewState bitfield.
    pub fn get_state(&self) -> ViewState {
        ViewState::empty()
    }

    /// Set ViewState bit. Toggle indicates whether it is set or not.
    pub fn set_state(&self, state: ViewState, toggle: bool) {
    }

    /// Gets parent view, returns `WlcView::root()` if this view has no parent.
    pub fn get_parent(&self) -> WlcView {
        WlcView::root()
    }

    /// Set the parent of this view.
    ///
    /// Call with `WlcView::root()` to make its parent the root window.
    pub fn set_parent(&self, parent: WlcView) {
    }

    /// Get the title of the view
    pub fn get_title(&self) -> String {
        "".to_string()
    }

    /// Get class (shell surface only).
    pub fn get_class(&self) -> String {
        "".to_string()
    }

    /// Get app id (xdg-surface only).
    pub fn get_app_id(&self) -> String {
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn dummy_views() {
        let dummy = WlcView::dummy(1);
        assert!(!dummy.is_root(), "Dummy(1) is root");
        assert!(dummy.is_window(), "Dummy(1) is root");
        let _title = dummy.get_title();
        let _class = dummy.get_class();
        let _app_id = dummy.get_app_id();
        // Let's do some stuff with views
        dummy.close(); // works
        let output = dummy.get_output();
        assert!(output == WlcOutput::dummy(0));
        dummy.set_output(output);
        // dummy.focus(); // SEGFAULTS
        // dummy.send_to_back();
        // dummy.send_below(&dummy);
        // dummy.bring_above(&dummy);
        // dummy.bring_to_front();
        let mask = dummy.get_mask();
        dummy.set_mask(mask);
        let geometry = dummy.get_geometry();
        dummy.set_geometry(EDGE_NONE, &Geometry {
            origin: Point { x: 0, y: 0 },
            size: Size { w: 0, h: 0 }
        });
        let view_type = dummy.get_type();
        assert!(view_type.is_empty(), "Dummy had a view type");
        dummy.set_type(ViewType::empty(), true);
        let view_state = dummy.get_state();
        assert!(view_state.is_empty(), "Dummu had a view state");
        dummy.set_state(view_state, true);
        let parent = dummy.get_parent();
        assert!(parent.is_root(), "Dummy had real parent");
        dummy.set_parent(parent);
    }

    #[test]
    fn dummy_outputs() {
        let dummy = WlcOutput::dummy(1);
        //let _current = WlcOutput::focused();
        //let _outputs = WlcOutput::list();
        //dummy.set_resolution(resolution.clone());
        dummy.schedule_render();
        let _name = dummy.get_name();
        let sleep = dummy.get_sleep();
        dummy.set_sleep(sleep);
        let _resolution = dummy.get_resolution();
        let mut views = dummy.get_views();
        dummy.set_views(&mut views).unwrap_err();
        let mask = dummy.get_mask();
        dummy.set_mask(mask);
        WlcOutput::focus(Some(dummy));
    }
}
