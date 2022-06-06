// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Icon;
use crate::MenuModel;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GMenuItem")]
    pub struct MenuItem(Object<ffi::GMenuItem>);

    match fn {
        type_ => || ffi::g_menu_item_get_type(),
    }
}

impl MenuItem {
    #[doc(alias = "g_menu_item_new")]
    pub fn new(label: Option<&str>, detailed_action: Option<&str>) -> MenuItem {
        unsafe {
            from_glib_full(ffi::g_menu_item_new(
                label.to_glib_none().0,
                detailed_action.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_menu_item_new_from_model")]
    #[doc(alias = "new_from_model")]
    pub fn from_model(model: &impl IsA<MenuModel>, item_index: i32) -> MenuItem {
        unsafe {
            from_glib_full(ffi::g_menu_item_new_from_model(
                model.as_ref().to_glib_none().0,
                item_index,
            ))
        }
    }

    #[doc(alias = "g_menu_item_new_section")]
    pub fn new_section(label: Option<&str>, section: &impl IsA<MenuModel>) -> MenuItem {
        unsafe {
            from_glib_full(ffi::g_menu_item_new_section(
                label.to_glib_none().0,
                section.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_menu_item_new_submenu")]
    pub fn new_submenu(label: Option<&str>, submenu: &impl IsA<MenuModel>) -> MenuItem {
        unsafe {
            from_glib_full(ffi::g_menu_item_new_submenu(
                label.to_glib_none().0,
                submenu.as_ref().to_glib_none().0,
            ))
        }
    }

    //#[doc(alias = "g_menu_item_get_attribute")]
    //#[doc(alias = "get_attribute")]
    //pub fn is_attribute(&self, attribute: &str, format_string: &str, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) -> bool {
    //    unsafe { TODO: call ffi:g_menu_item_get_attribute() }
    //}

    #[doc(alias = "g_menu_item_get_attribute_value")]
    #[doc(alias = "get_attribute_value")]
    pub fn attribute_value(
        &self,
        attribute: &str,
        expected_type: Option<&glib::VariantTy>,
    ) -> Option<glib::Variant> {
        unsafe {
            from_glib_full(ffi::g_menu_item_get_attribute_value(
                self.to_glib_none().0,
                attribute.to_glib_none().0,
                expected_type.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_menu_item_get_link")]
    #[doc(alias = "get_link")]
    pub fn link(&self, link: &str) -> Option<MenuModel> {
        unsafe {
            from_glib_full(ffi::g_menu_item_get_link(
                self.to_glib_none().0,
                link.to_glib_none().0,
            ))
        }
    }

    //#[doc(alias = "g_menu_item_set_action_and_target")]
    //pub fn set_action_and_target(&self, action: Option<&str>, format_string: Option<&str>, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) {
    //    unsafe { TODO: call ffi:g_menu_item_set_action_and_target() }
    //}

    #[doc(alias = "g_menu_item_set_action_and_target_value")]
    pub fn set_action_and_target_value(
        &self,
        action: Option<&str>,
        target_value: Option<&glib::Variant>,
    ) {
        unsafe {
            ffi::g_menu_item_set_action_and_target_value(
                self.to_glib_none().0,
                action.to_glib_none().0,
                target_value.to_glib_none().0,
            );
        }
    }

    //#[doc(alias = "g_menu_item_set_attribute")]
    //pub fn set_attribute(&self, attribute: &str, format_string: Option<&str>, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) {
    //    unsafe { TODO: call ffi:g_menu_item_set_attribute() }
    //}

    #[doc(alias = "g_menu_item_set_attribute_value")]
    pub fn set_attribute_value(&self, attribute: &str, value: Option<&glib::Variant>) {
        unsafe {
            ffi::g_menu_item_set_attribute_value(
                self.to_glib_none().0,
                attribute.to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "g_menu_item_set_detailed_action")]
    pub fn set_detailed_action(&self, detailed_action: &str) {
        unsafe {
            ffi::g_menu_item_set_detailed_action(
                self.to_glib_none().0,
                detailed_action.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "g_menu_item_set_icon")]
    pub fn set_icon(&self, icon: &impl IsA<Icon>) {
        unsafe {
            ffi::g_menu_item_set_icon(self.to_glib_none().0, icon.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "g_menu_item_set_label")]
    pub fn set_label(&self, label: Option<&str>) {
        unsafe {
            ffi::g_menu_item_set_label(self.to_glib_none().0, label.to_glib_none().0);
        }
    }

    #[doc(alias = "g_menu_item_set_link")]
    pub fn set_link(&self, link: &str, model: Option<&impl IsA<MenuModel>>) {
        unsafe {
            ffi::g_menu_item_set_link(
                self.to_glib_none().0,
                link.to_glib_none().0,
                model.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "g_menu_item_set_section")]
    pub fn set_section(&self, section: Option<&impl IsA<MenuModel>>) {
        unsafe {
            ffi::g_menu_item_set_section(
                self.to_glib_none().0,
                section.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "g_menu_item_set_submenu")]
    pub fn set_submenu(&self, submenu: Option<&impl IsA<MenuModel>>) {
        unsafe {
            ffi::g_menu_item_set_submenu(
                self.to_glib_none().0,
                submenu.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }
}

impl fmt::Display for MenuItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("MenuItem")
    }
}
