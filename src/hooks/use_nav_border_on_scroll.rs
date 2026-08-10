use std::cell::RefCell;
use std::rc::Rc;

use dioxus::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

struct NavScrollBorderGuard {
    window: web_sys::Window,
    _scroll: Closure<dyn FnMut(web_sys::Event)>,
    _resize: Closure<dyn FnMut(web_sys::Event)>,
}

impl Drop for NavScrollBorderGuard {
    fn drop(&mut self) {
        let _ = self.window.remove_event_listener_with_callback(
            "scroll",
            self._scroll.as_ref().unchecked_ref(),
        );
        let _ = self.window.remove_event_listener_with_callback(
            "resize",
            self._resize.as_ref().unchecked_ref(),
        );
    }
}

fn sync_nav_border(window: &web_sys::Window, mut border_visible: Signal<bool>) {
    let y = window.scroll_y().unwrap_or(0.0);
    let scroll_height = window
        .document()
        .and_then(|d| d.document_element())
        .map(|e| e.scroll_height() as f64)
        .unwrap_or(0.0);
    let viewport = window
        .inner_height()
        .ok()
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);
    let can_scroll = scroll_height > viewport + 1.0;
    border_visible.set(can_scroll && y > 0.5);
}

/// `true` when the page can scroll and the user has scrolled down — show `border-gray-800`; otherwise transparent.
pub fn use_nav_border_on_scroll() -> Signal<bool> {
    let border_visible = use_signal(|| false);
    let border_for_listener = border_visible.clone();

    let guard: Rc<RefCell<Option<NavScrollBorderGuard>>> =
        use_hook(|| Rc::new(RefCell::new(None)));

    let guard_drop = guard.clone();
    use_drop(move || {
        guard_drop.borrow_mut().take();
    });

    let guard_effect = guard.clone();
    use_effect(move || {
        guard_effect.borrow_mut().take();

        let Some(window) = web_sys::window() else {
            return;
        };

        sync_nav_border(&window, border_for_listener.clone());

        let win_scroll = window.clone();
        let border_scroll = border_for_listener.clone();
        let scroll_closure = Closure::wrap(Box::new(move |_ev: web_sys::Event| {
            sync_nav_border(&win_scroll, border_scroll);
        }) as Box<dyn FnMut(web_sys::Event)>);

        let win_resize = window.clone();
        let border_resize = border_for_listener.clone();
        let resize_closure = Closure::wrap(Box::new(move |_ev: web_sys::Event| {
            sync_nav_border(&win_resize, border_resize);
        }) as Box<dyn FnMut(web_sys::Event)>);

        let _ = window.add_event_listener_with_callback(
            "scroll",
            scroll_closure.as_ref().unchecked_ref(),
        );
        let _ = window.add_event_listener_with_callback(
            "resize",
            resize_closure.as_ref().unchecked_ref(),
        );

        *guard_effect.borrow_mut() = Some(NavScrollBorderGuard {
            window,
            _scroll: scroll_closure,
            _resize: resize_closure,
        });
    });

    border_visible
}
