use std::cmp::PartialEq;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use dark_light::Mode;

#[derive(PartialEq)]
struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}
impl Color {
    fn from_rgb(r: f32, g: f32, b: f32) -> Self {
        Color { r, g, b }
    }
}

#[derive(Debug, PartialEq)]
enum AccentColor {
    Multicolor,
    Blue,
    Purple,
    Pink,
    Red,
    Orange,
    Yellow,
    Green,
    Graphite
}
impl AccentColor {
    fn from_id(id: Option<i64>) -> Self {
        match id {
            None => AccentColor::Multicolor,
            Some(id) => {
                match id {
                    -1 => AccentColor::Graphite,
                    0 => AccentColor::Red,
                    1 => AccentColor::Orange,
                    2 => AccentColor::Yellow,
                    3 => AccentColor::Green,
                    4 => AccentColor::Blue,
                    5 => AccentColor::Purple,
                    6 => AccentColor::Pink,
                    _ => AccentColor::Multicolor,
                }
            }
        }
    }
}

fn main() {
    let mut is_dark = if let Mode::Dark = dark_light::detect() { true } else { false };
    let mut accent_color = AccentColor::from_id(get_accent_color_id());
    let mut highlight_color = get_highlight_color_value();
    println!("深色模式状态：{}", is_dark);
    println!("Accent Color: {:?}", accent_color);
    if let Some((color, name)) = &highlight_color {
        println!("Highlight Color: {}, {}, {}, {}", color.r, color.g, color.b, name);
    } else {
        println!("Highlight Color: Accent Color");
    }
    loop {
        let new_is_dark = if let Mode::Dark = dark_light::detect() { true } else { false };
        let new_accent_color = AccentColor::from_id(get_accent_color_id());
        let new_highlight_color = get_highlight_color_value();
        if new_is_dark != is_dark || new_accent_color != accent_color || new_highlight_color != highlight_color {
            is_dark = new_is_dark;
            accent_color = new_accent_color;
            highlight_color = new_highlight_color;
            println!("深色模式状态：{}", is_dark);
            println!("Accent Color: {:?}", accent_color);
            if let Some((color, name)) = &highlight_color {
                println!("Highlight Color: {}, {}, {}, {}", color.r, color.g, color.b, name);
            } else {
                println!("Highlight Color: Accent Color");
            }
        }
        sleep(Duration::from_millis(250))
    }
}

fn get_highlight_color_value() -> Option<(Color, String)> {
    let mut highlight_color_value = None;

    let output = Command::new("defaults")
        .arg("read")
        .arg("-g")
        .arg("AppleHighlightColor")
        .output();
    let mut color_name = String::new();
    if let Ok(output) = output {
        if output.status.success() { //当Highlight color选中为Accent color时，output的status就不是success
            let highlight_color = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !highlight_color.is_empty() {
                let components: Vec<&str> = highlight_color.split_whitespace().collect();
                if components.len() >= 3 {
                    // 解析 RGB 值
                    let red: f32 = components[0].parse().unwrap_or(0.0);
                    let green: f32 = components[1].parse().unwrap_or(0.0);
                    let blue: f32 = components[2].parse().unwrap_or(0.0);
                    color_name = components[3].to_string();
                    highlight_color_value = Some((Color::from_rgb(red, green, blue), color_name));
                }
            }
        }
    };
    highlight_color_value
}

fn get_accent_color_id() -> Option<i64> {
    let mut accent_color_id = None;

    let output = Command::new("defaults")
        .arg("read")
        .arg("-g")
        .arg("AppleAccentColor")
        .output();
    if let Ok(output) = output {
        if output.status.success() { //当Accent color选中为Multicolor时，output的status就不是success
            let highlight_color = String::from_utf8_lossy(&output.stdout).trim().to_string();

            if !highlight_color.is_empty() {
                accent_color_id = Some(highlight_color.parse::<i64>().unwrap_or(0));
            }
        }
    };
    accent_color_id
}