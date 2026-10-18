// Lifted from kommando (src-tauri/src/wayland_activation.rs, 2026-10-17),
// which m2d had copied too; skriva was the third app needing it.
//
//! xdg-activation tokens tied to the user's latest keypress or click.
//!
//! GNOME only lets a launched app take focus if it carries a token that Mutter
//! can link to recent user input on a still-focused surface. GTK3's own launch
//! context builds the token from the last pointer-button serial only, which
//! doesn't cover a keyboard-only flow (hotkey, type, Enter). Here we listen to
//! our own keyboard/pointer events on GDK's Wayland connection and request the
//! token with the latest serial ourselves. (The "…_TIME0" suffix on Mutter's
//! tokens is normal on Wayland and not a sign of an invalid token.)

use std::sync::{Mutex, OnceLock};

use gtk::glib::translate::ToGlibPtr;
use gtk::prelude::*;
use wayland_client::{
    backend::{Backend, ObjectId},
    protocol::{wl_keyboard, wl_pointer, wl_registry, wl_seat, wl_surface::WlSurface},
    Connection, Dispatch, EventQueue, Proxy, QueueHandle, WEnum,
};
use wayland_protocols::xdg::activation::v1::client::{xdg_activation_token_v1, xdg_activation_v1};

#[derive(Default)]
struct State {
    seat: Option<wl_seat::WlSeat>,
    activation: Option<xdg_activation_v1::XdgActivationV1>,
    last_serial: Option<u32>,
    token: Option<String>,
}

struct Tracker {
    conn: Connection,
    queue: EventQueue<State>,
    state: State,
}

static TRACKER: OnceLock<Mutex<Tracker>> = OnceLock::new();

/// Hooks into GDK's Wayland connection. Must run on the GTK main thread, after
/// GTK is initialised; a no-op on X11 or if the compositor lacks xdg-activation.
pub fn init() {
    let Some(display) = gtk::gdk::Display::default() else {
        return;
    };
    if display.type_().name() != "GdkWaylandDisplay" {
        return;
    }
    let display_ptr: *mut gtk::gdk::ffi::GdkDisplay = display.to_glib_none().0;
    let wl_display =
        unsafe { gdk_wayland_sys::gdk_wayland_display_get_wl_display(display_ptr as *mut _) };
    if wl_display.is_null() {
        return;
    }
    let backend = unsafe { Backend::from_foreign_display(wl_display as *mut _) };
    let conn = Connection::from_backend(backend);
    let mut queue = conn.new_event_queue();
    conn.display().get_registry(&queue.handle(), ());

    let mut state = State::default();
    // First roundtrip binds the globals, second gets seat capabilities.
    for _ in 0..2 {
        if queue.roundtrip(&mut state).is_err() {
            return;
        }
    }
    if state.seat.is_none() || state.activation.is_none() {
        return;
    }
    let _ = TRACKER.set(Mutex::new(Tracker { conn, queue, state }));
}

/// Requests a token for `window`'s surface, tied to the latest input serial.
/// Must run on the GTK main thread while `window` still has focus.
pub fn token(window: &gtk::ApplicationWindow) -> Option<String> {
    let mut guard = TRACKER.get()?.lock().ok()?;
    let Tracker { conn, queue, state } = &mut *guard;

    // GDK has already read the latest input from the socket; this just
    // processes the copies routed to our queue.
    queue.dispatch_pending(state).ok()?;
    let serial = state.last_serial?;

    let gdk_window = window.window()?;
    let window_ptr: *mut gtk::gdk::ffi::GdkWindow = gdk_window.to_glib_none().0;
    let surface_ptr =
        unsafe { gdk_wayland_sys::gdk_wayland_window_get_wl_surface(window_ptr as *mut _) };
    if surface_ptr.is_null() {
        return None;
    }
    let surface_id =
        unsafe { ObjectId::from_ptr(WlSurface::interface(), surface_ptr as *mut _) }.ok()?;
    let surface = WlSurface::from_id(conn, surface_id).ok()?;

    let token = state
        .activation
        .as_ref()?
        .get_activation_token(&queue.handle(), ());
    token.set_serial(serial, state.seat.as_ref()?);
    token.set_surface(&surface);
    token.commit();

    state.token = None;
    while state.token.is_none() {
        queue.blocking_dispatch(state).ok()?;
    }
    token.destroy();
    state.token.take()
}

impl Dispatch<wl_registry::WlRegistry, ()> for State {
    fn event(
        state: &mut Self,
        registry: &wl_registry::WlRegistry,
        event: wl_registry::Event,
        _: &(),
        _: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        if let wl_registry::Event::Global {
            name,
            interface,
            version,
        } = event
        {
            match interface.as_str() {
                "wl_seat" if state.seat.is_none() => {
                    state.seat = Some(registry.bind(name, version.min(5), qh, ()));
                }
                "xdg_activation_v1" => {
                    state.activation = Some(registry.bind(name, 1, qh, ()));
                }
                _ => {}
            }
        }
    }
}

impl Dispatch<wl_seat::WlSeat, ()> for State {
    fn event(
        _: &mut Self,
        seat: &wl_seat::WlSeat,
        event: wl_seat::Event,
        _: &(),
        _: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        if let wl_seat::Event::Capabilities {
            capabilities: WEnum::Value(caps),
        } = event
        {
            if caps.contains(wl_seat::Capability::Keyboard) {
                seat.get_keyboard(qh, ());
            }
            if caps.contains(wl_seat::Capability::Pointer) {
                seat.get_pointer(qh, ());
            }
        }
    }
}

impl Dispatch<wl_keyboard::WlKeyboard, ()> for State {
    fn event(
        state: &mut Self,
        _: &wl_keyboard::WlKeyboard,
        event: wl_keyboard::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        // Presses only: Mutter checks the serial against the latest key
        // down, so a release (Ctrl let go after Ctrl+Enter) voids the token.
        if let wl_keyboard::Event::Key {
            serial,
            state: WEnum::Value(wl_keyboard::KeyState::Pressed),
            ..
        } = event
        {
            state.last_serial = Some(serial);
        }
    }
}

impl Dispatch<wl_pointer::WlPointer, ()> for State {
    fn event(
        state: &mut Self,
        _: &wl_pointer::WlPointer,
        event: wl_pointer::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        if let wl_pointer::Event::Button {
            serial,
            state: WEnum::Value(wl_pointer::ButtonState::Pressed),
            ..
        } = event
        {
            state.last_serial = Some(serial);
        }
    }
}

impl Dispatch<xdg_activation_v1::XdgActivationV1, ()> for State {
    fn event(
        _: &mut Self,
        _: &xdg_activation_v1::XdgActivationV1,
        _: xdg_activation_v1::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<xdg_activation_token_v1::XdgActivationTokenV1, ()> for State {
    fn event(
        state: &mut Self,
        _: &xdg_activation_token_v1::XdgActivationTokenV1,
        event: xdg_activation_token_v1::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        if let xdg_activation_token_v1::Event::Done { token } = event {
            state.token = Some(token);
        }
    }
}
