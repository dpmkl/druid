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

//! x11 implementation of window creation.

pub mod application;
pub mod dialog;
pub mod menu;
pub mod util;
pub mod win_main;

pub use menu::Menu;

use std::any::Any;
use std::ffi::OsString;

use crate::keyboard::{KeyEvent, KeyModifiers};
use crate::platform::dialog::{FileDialogOptions, FileDialogType};
use crate::window::{MouseButton, MouseEvent, WinHandler};
use crate::Error;

#[derive(Clone, Default)]
pub struct WindowHandle;

impl WindowHandle {
    pub fn show(&self) {
        unimplemented!()
    }

    pub fn close(&self) {
        unimplemented!()
    }

    pub fn invalidate(&self) {
        unimplemented!()
    }
    
    pub fn get_idle_handle(&self) -> Option<IdleHandle> {
        unimplemented!()
    }

    pub fn get_dpi(&self) -> f32 {
        unimplemented!()
    }

    pub fn px_to_pixels(&self, x: f32) -> i32 {
        unimplemented!()
    }

    pub fn px_to_pixels_xy(&self, x: f32, y: f32) -> (i32, i32) {
        unimplemented!()
    }

    pub fn pixels_to_px<T: Into<f64>>(&self, x: T) -> f32 {
        unimplemented!()
    }

    pub fn pixels_to_px_xy<T: Into<f64>>(&self, x: T, y: T) -> (f32, f32) {
        unimplemented!()
    }

    pub fn file_dialog(
        &self,
        _ty: FileDialogType,
        _options: FileDialogOptions,
    ) -> Result<OsString, Error> {
        unimplemented!()
    }
}

pub struct WindowBuilder {
    handler: Option<Box<dyn WinHandler>>,
    title: String,
    enable_mouse_move_events: bool,
    menu: Option<Menu>,
}

impl WindowBuilder {
    pub fn new() -> Self {
        WindowBuilder {
            handler: None,
            title: String::new(),
            enable_mouse_move_events: true,
            menu: Some(Menu::default()),
        }
    }

    pub fn set_handler(&mut self, handler: Box<dyn WinHandler>) {
        self.handler = Some(handler);
    }

    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    pub fn set_menu(&mut self, menu: Menu) {
        self.menu = Some(menu);
    }
    pub fn set_enable_mouse_move_events(&mut self, to: bool) {
        self.enable_mouse_move_events = to;
    }

    pub fn build(self) -> Result<WindowHandle, Error> {
        unimplemented!()
    }
}

pub struct IdleHandle;

impl IdleHandle {
    /// Add an idle handler, which is called (once) when the message loop
    /// is empty. The idle handler will be run from the main UI thread, and
    /// won't be scheduled if the associated view has been dropped.
    ///
    /// Note: the name "idle" suggests that it will be scheduled with a lower
    /// priority than other UI events, but that's not necessarily the case.
    pub fn add_idle<F>(&self, callback: F)
    where
        F: FnOnce(&dyn Any) + Send + 'static,
    {
        unimplemented!()
    }
}