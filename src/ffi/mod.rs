use libc::c_char;
use std::{cell::RefCell, ffi, rc::Rc};
use crate::{prelude::*, widgets::{button, SharedWidget}};

#[unsafe(no_mangle)]
extern "C" fn wtk_app_sdl_new(title: *const c_char) -> *mut App<SDLBackend> {
    let title_c = unsafe { ffi::CStr::from_ptr(title) };
    let app = App::<SDLBackend>::new(title_c.to_str().unwrap());
    Box::into_raw(Box::new(app))
}

#[unsafe(no_mangle)]
extern "C" fn wtk_app_sdl_destroy(app: *mut App<SDLBackend>) {
    unsafe { let _ = Box::from_raw(app); };
}

#[unsafe(no_mangle)]
extern "C" fn wtk_app_run(app: *mut App<SDLBackend>) {
    let mut app = unsafe { Box::from_raw(app) };
    app.run();
}

#[unsafe(no_mangle)]
extern "C" fn wtk_button_new(text: *const c_char, cb: extern "C" fn(*mut Button)) -> *mut Button {
    let text = unsafe { ffi::CStr::from_ptr(text).to_str().unwrap() };
    let button = Button::new(text, move |b| {
        cb(b)
    });
    Box::into_raw(Box::new(button))
}

#[unsafe(no_mangle)]
extern "C" fn wtk_widget_destroy(button: *mut SharedWidget) {
    let _ = unsafe { Box::from_raw(button) };
}

#[unsafe(no_mangle)]
extern "C" fn wtk_button_share(button: *mut Button) -> *mut SharedWidget {
    let button = unsafe { Box::from_raw(button) };
    Box::into_raw(Box::new((*button).shared()))
}

#[unsafe(no_mangle)]
extern "C" fn wtk_button_set_text(text: *const c_char, button: *mut Button) {
    let text = unsafe { ffi::CStr::from_ptr(text).to_str().unwrap() };
    let button = unsafe { &mut *button };
    button.set_text(text);
}

#[unsafe(no_mangle)]
extern "C" fn wtk_app_sdl_add_widget(app: *mut App<SDLBackend>, widget: *mut SharedWidget) {
    let app = unsafe { &mut *app };
    let widget = unsafe { &*widget };
    app.add_widget(widget.clone());
}

