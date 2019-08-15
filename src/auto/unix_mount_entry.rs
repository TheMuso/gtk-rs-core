// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gio_sys;
use gobject_sys;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct UnixMountEntry(Boxed<gio_sys::GUnixMountEntry>);

    match fn {
        copy => |ptr| gobject_sys::g_boxed_copy(gio_sys::g_unix_mount_entry_get_type(), ptr as *mut _) as *mut gio_sys::GUnixMountEntry,
        free => |ptr| gobject_sys::g_boxed_free(gio_sys::g_unix_mount_entry_get_type(), ptr as *mut _),
        get_type => || gio_sys::g_unix_mount_entry_get_type(),
    }
}
