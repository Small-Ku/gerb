/*
 * gerb
 *
 * Copyright 2022 - Manos Pitsidianakis
 *
 * This file is part of gerb.
 *
 * gerb is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * gerb is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with gerb. If not, see <http://www.gnu.org/licenses/>.
 */

use super::tool_impl::*;
use crate::views::{Canvas, GlyphEditView};
use glib::subclass::prelude::{ObjectImpl, ObjectSubclass};
use gtk::Inhibit;
use gtk::{
    glib::{self},
    prelude::*,
    subclass::prelude::*,
};

#[derive(Default)]
pub struct ZoomInToolInner;

#[glib::object_subclass]
impl ObjectSubclass for ZoomInToolInner {
    const NAME: &'static str = "ZoomInTool";
    type ParentType = ToolImpl;
    type Type = ZoomInTool;
}

impl ObjectImpl for ZoomInToolInner {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
        obj.set_property::<String>(ToolImpl::NAME, "ZoomIn".to_string());
        obj.set_property::<gtk::Image>(
            ToolImpl::ICON,
            crate::resources::svg_to_image_widget(crate::resources::ZOOM_IN_ICON_SVG),
        );
    }
}

impl ToolImplImpl for ZoomInToolInner {
    fn on_button_press_event(
        &self,
        _obj: &ToolImpl,
        _view: GlyphEditView,
        _viewport: &Canvas,
        _event: &gtk::gdk::EventButton,
    ) -> Inhibit {
        Inhibit(false)
    }

    fn on_button_release_event(
        &self,
        _obj: &ToolImpl,
        _view: GlyphEditView,
        _viewport: &Canvas,
        _event: &gtk::gdk::EventButton,
    ) -> Inhibit {
        Inhibit(false)
    }

    fn on_motion_notify_event(
        &self,
        _obj: &ToolImpl,
        _view: GlyphEditView,
        _viewport: &Canvas,
        _event: &gtk::gdk::EventMotion,
    ) -> Inhibit {
        Inhibit(false)
    }

    fn setup_toolbox(&self, _obj: &ToolImpl, toolbar: &gtk::Toolbar, view: &GlyphEditView) {
        let obj = self.instance();
        let name = obj.property::<String>(ToolImpl::NAME);
        let button = gtk::ToolButton::builder()
            .icon_widget(&obj.property::<gtk::Image>(ToolImpl::ICON))
            .label(&name)
            .visible(true)
            .tooltip_text(&name)
            .build();
        button.connect_clicked(
            clone!(@strong view, @strong obj as self_ => move |_button| {
                self_.on_activate(&view)
            }),
        );
        toolbar.add(&button);
        toolbar.set_item_homogeneous(&button, false);
    }

    fn on_activate(&self, _obj: &ToolImpl, view: &GlyphEditView) {
        let t = &view.imp().viewport.imp().transformation;
        t.zoom_in();
    }
}

impl ZoomInToolInner {}

glib::wrapper! {
    pub struct ZoomInTool(ObjectSubclass<ZoomInToolInner>)
        @extends ToolImpl;
}

impl Default for ZoomInTool {
    fn default() -> Self {
        Self::new()
    }
}

impl ZoomInTool {
    pub fn new() -> Self {
        glib::Object::new(&[]).unwrap()
    }
}

#[derive(Default)]
pub struct ZoomOutToolInner;

#[glib::object_subclass]
impl ObjectSubclass for ZoomOutToolInner {
    const NAME: &'static str = "ZoomOutTool";
    type ParentType = ToolImpl;
    type Type = ZoomOutTool;
}

impl ObjectImpl for ZoomOutToolInner {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
        obj.set_property::<String>(ToolImpl::NAME, "ZoomOut".to_string());
        obj.set_property::<gtk::Image>(
            ToolImpl::ICON,
            crate::resources::svg_to_image_widget(crate::resources::ZOOM_OUT_ICON_SVG),
        );
    }
}

impl ToolImplImpl for ZoomOutToolInner {
    fn on_button_press_event(
        &self,
        _obj: &ToolImpl,
        _view: GlyphEditView,
        _viewport: &Canvas,
        _event: &gtk::gdk::EventButton,
    ) -> Inhibit {
        Inhibit(false)
    }

    fn on_button_release_event(
        &self,
        _obj: &ToolImpl,
        _view: GlyphEditView,
        _viewport: &Canvas,
        _event: &gtk::gdk::EventButton,
    ) -> Inhibit {
        Inhibit(false)
    }

    fn on_motion_notify_event(
        &self,
        _obj: &ToolImpl,
        _view: GlyphEditView,
        _viewport: &Canvas,
        _event: &gtk::gdk::EventMotion,
    ) -> Inhibit {
        Inhibit(false)
    }

    fn setup_toolbox(&self, _obj: &ToolImpl, toolbar: &gtk::Toolbar, view: &GlyphEditView) {
        let obj = self.instance();
        let name = obj.property::<String>(ToolImpl::NAME);
        let button = gtk::ToolButton::builder()
            .icon_widget(&obj.property::<gtk::Image>(ToolImpl::ICON))
            .label(&name)
            .visible(true)
            .tooltip_text(&name)
            .build();
        button.connect_clicked(
            clone!(@strong view, @strong obj as self_ => move |_button| {
                self_.on_activate(&view)
            }),
        );
        toolbar.add(&button);
        toolbar.set_item_homogeneous(&button, false);
    }

    fn on_activate(&self, _obj: &ToolImpl, view: &GlyphEditView) {
        let t = &view.imp().viewport.imp().transformation;
        t.zoom_out();
    }
}

impl ZoomOutToolInner {}

glib::wrapper! {
    pub struct ZoomOutTool(ObjectSubclass<ZoomOutToolInner>)
        @extends ToolImpl;
}

impl Default for ZoomOutTool {
    fn default() -> Self {
        Self::new()
    }
}

impl ZoomOutTool {
    pub fn new() -> Self {
        glib::Object::new(&[]).unwrap()
    }
}