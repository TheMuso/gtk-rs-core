// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use FontDescription;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::fmt;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct FontFace(Object<ffi::PangoFontFace, ffi::PangoFontFaceClass>);

    match fn {
        get_type => || ffi::pango_font_face_get_type(),
    }
}

pub trait FontFaceExt {
    fn describe(&self) -> Option<FontDescription>;

    fn get_face_name(&self) -> Option<String>;

    fn is_synthesized(&self) -> bool;

    fn list_sizes(&self) -> Vec<i32>;
}

impl<O: IsA<FontFace>> FontFaceExt for O {
    fn describe(&self) -> Option<FontDescription> {
        unsafe {
            from_glib_full(ffi::pango_font_face_describe(self.to_glib_none().0))
        }
    }

    fn get_face_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::pango_font_face_get_face_name(self.to_glib_none().0))
        }
    }

    fn is_synthesized(&self) -> bool {
        unsafe {
            from_glib(ffi::pango_font_face_is_synthesized(self.to_glib_none().0))
        }
    }

    fn list_sizes(&self) -> Vec<i32> {
        unsafe {
            let mut sizes = ptr::null_mut();
            let mut n_sizes = mem::uninitialized();
            ffi::pango_font_face_list_sizes(self.to_glib_none().0, &mut sizes, &mut n_sizes);
            FromGlibContainer::from_glib_full_num(sizes, n_sizes as usize)
        }
    }
}

impl fmt::Display for FontFace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FontFace")
    }
}
