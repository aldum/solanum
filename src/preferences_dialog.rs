// preferences_dialog.rs
//
// Copyright 2021 Christopher Davis <christopherdavis@gnome.org>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
//
// SPDX-License-Identifier: GPL-3.0-or-later

use gtk::CompositeTemplate;
use gtk::{gio, glib};
use libadwaita::prelude::*;
use libadwaita::subclass::prelude::*;

use std::cell::OnceCell;

mod imp {
    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/org/gnome/Solanum/preferences-dialog.ui")]
    pub struct SolanumPreferencesDialog {
        #[template_child]
        pub lap_spin: TemplateChild<libadwaita::SpinRow>,
        #[template_child]
        pub short_break_spin: TemplateChild<libadwaita::SpinRow>,
        #[template_child]
        pub long_break_spin: TemplateChild<libadwaita::SpinRow>,
        #[template_child]
        pub session_count_spin: TemplateChild<libadwaita::SpinRow>,
        #[template_child]
        pub fullscreen_switch: TemplateChild<libadwaita::SwitchRow>,
        pub settings: OnceCell<gio::Settings>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SolanumPreferencesDialog {
        const NAME: &'static str = "SolanumPreferencesDialog";
        type Type = super::SolanumPreferencesDialog;
        type ParentType = libadwaita::PreferencesDialog;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for SolanumPreferencesDialog {}
    impl WidgetImpl for SolanumPreferencesDialog {}
    impl AdwDialogImpl for SolanumPreferencesDialog {}
    impl PreferencesDialogImpl for SolanumPreferencesDialog {}
}

glib::wrapper! {
    pub struct SolanumPreferencesDialog(ObjectSubclass<imp::SolanumPreferencesDialog>)
        @extends gtk::Widget, libadwaita::Dialog, libadwaita::PreferencesDialog;
}

impl SolanumPreferencesDialog {
    pub fn new(settings: &gio::Settings) -> Self {
        let obj = glib::Object::builder::<Self>().build();

        let imp = obj.imp();

        settings.bind("lap-length", &*imp.lap_spin, "value").build();
        settings
            .bind("short-break-length", &*imp.short_break_spin, "value")
            .build();
        settings
            .bind("long-break-length", &*imp.long_break_spin, "value")
            .build();
        settings
            .bind(
                "sessions-until-long-break",
                &*imp.session_count_spin,
                "value",
            )
            .build();
        settings
            .bind("fullscreen-break", &*imp.fullscreen_switch, "active")
            .build();

        obj
    }
}
