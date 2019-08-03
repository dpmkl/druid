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

//! x11 implementation of menus.

use crate::keycodes::MenuKey;

pub struct Menu;

impl Menu {
    pub fn new() -> Menu {
        Menu {}
    }

    pub fn add_dropdown(&mut self, menu: Menu, _text: &str) {
        // unimplemented!()
    }

    pub fn add_item(&mut self, id: u32, text: &str, key: impl Into<MenuKey>) {
        // unimplemented!()
    }

    pub fn add_separator(&mut self) {
        // unimplemented!()
    }
}

impl Default for Menu {
    fn default() -> Menu {
        Menu::new()
    }
}
