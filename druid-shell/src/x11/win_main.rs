// Copyright 2019 The xi-editor Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! x11 implementation of runloop.

use lazy_static;
use std::mem;
use std::ptr;
use x11_dl::xlib;

lazy_static! {
    pub static ref XLIB: x11_dl::xlib::Xlib = xlib::Xlib::open().expect("Could not load xlib");
}

pub struct XSession {
    pub display: *mut x11_dl::xlib::Display,
}

impl XSession {
    pub fn new() -> Self {
        unsafe {
            XSession {
                display: (XLIB.XOpenDisplay)(ptr::null()),
            }
        }
    }
}

impl Drop for XSession {
    #[inline]
    fn drop(&mut self) {
        unsafe { (XLIB.XCloseDisplay)(self.display) };
    }
}

unsafe impl Send for XSession {}
unsafe impl Sync for XSession {}

lazy_static! {
    pub static ref XSESSION: XSession = XSession::new();
}

pub struct RunLoop;

impl RunLoop {
    pub fn new() -> RunLoop {
        RunLoop {}
    }

    pub fn run(&mut self) {
        unsafe {
            let mut event: xlib::XEvent = mem::uninitialized();
            loop {
                (XLIB.XNextEvent)(XSESSION.display, &mut event);
            }
        }
    }
}

pub fn request_quit() {
    unimplemented!()
}
