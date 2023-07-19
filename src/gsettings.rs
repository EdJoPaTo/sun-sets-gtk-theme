use gio::prelude::SettingsExtManual;

// See
// /usr/share/glib-2.0/schemas/org.gnome.desktop.interface.gschema.xml
// /usr/share/glib-2.0/schemas/org.gnome.desktop.enums.xml

pub struct Gsettings(gio::Settings);

impl Gsettings {
    pub fn new() -> Self {
        Self(gio::Settings::new("org.gnome.desktop.interface"))
    }

    fn set(&self, key: &'static str, value: &str) {
        let current: String = self.0.get(key);
        if current != value {
            println!("{key} '{value}' (was '{current}')");
            self.0.set(key, value).unwrap();
        }
    }

    pub fn get_color_scheme(&self) -> String {
        self.0.get("color-scheme")
    }

    pub fn set_color_scheme(&self, scheme: &str) {
        self.set("color-scheme", scheme);
    }

    pub fn set_theme(&self, scheme: &str) {
        self.set("gtk-theme", scheme);
    }
}
