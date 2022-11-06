use std::error::Error;
use std::process::Command;

pub fn current() -> Result<String, Box<dyn Error>> {
    let output = Command::new("gsettings")
        .arg("get")
        .arg("org.gnome.desktop.interface")
        .arg("gtk-theme")
        .output()?;

    if output.status.success() {
        let stdout = String::from_utf8(output.stdout)?;
        let trimmed = stdout.trim();
        let result = trimmed[1..trimmed.len() - 1].to_string();
        Ok(result)
    } else {
        let text = format!("gsettings failed to execute {output:?}");
        let error = std::io::Error::new(std::io::ErrorKind::Other, text);
        Err(Box::new(error))
    }
}

pub fn set(gtk_theme_name: &str) -> Result<(), Box<dyn Error>> {
    let output = Command::new("gsettings")
        .arg("set")
        .arg("org.gnome.desktop.interface")
        .arg("gtk-theme")
        .arg(gtk_theme_name)
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        let text = format!("gsettings failed to execute {output:?}");
        let error = std::io::Error::new(std::io::ErrorKind::Other, text);
        Err(Box::new(error))
    }
}
