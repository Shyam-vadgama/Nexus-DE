use image::imageops::FilterType;
use image::GenericImageView;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ThemeError {
    #[error("Image error: {0}")]
    Image(#[from] image::ImageError),
    #[error("Failed to parse hex color: {0}")]
    InvalidHex(String),
}

/// Material You dynamic color palette generated from a dominant wallpaper color.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NexusPalette {
    pub primary: String,
    pub on_primary: String,
    pub primary_container: String,
    pub on_primary_container: String,
    pub secondary: String,
    pub surface: String,
    pub on_surface: String,
    pub surface_variant: String,
    pub outline: String,
    pub background: String,
    pub active_border: String,
    pub inactive_border: String,
}

impl Default for NexusPalette {
    fn default() -> Self {
        Self {
            primary: "#38BDF8".to_string(),
            on_primary: "#031E2C".to_string(),
            primary_container: "#084C61".to_string(),
            on_primary_container: "#BEE9F7".to_string(),
            secondary: "#7DD3FC".to_string(),
            surface: "#0F172A".to_string(),
            on_surface: "#F8FAFC".to_string(),
            surface_variant: "#1E293B".to_string(),
            outline: "#475569".to_string(),
            background: "#020617".to_string(),
            active_border: "rgba(56, 189, 248, 1.0)".to_string(),
            inactive_border: "rgba(30, 41, 59, 0.7)".to_string(),
        }
    }
}

impl NexusPalette {
    /// Generate palette from an RGB seed color
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        // Convert RGB to HSL-like metrics for tonal shifts
        let (h, s, l) = rgb_to_hsl(r, g, b);

        // Ensure minimum saturation for vibrant accents
        let accent_s = s.max(0.45);
        let accent_l = l.clamp(0.45, 0.65);

        let primary = hsl_to_hex(h, accent_s, accent_l);
        let on_primary = if accent_l > 0.5 {
            "#050B14".to_string()
        } else {
            "#FFFFFF".to_string()
        };

        let primary_container = hsl_to_hex(h, accent_s * 0.8, 0.22);
        let on_primary_container = hsl_to_hex(h, accent_s * 0.6, 0.88);

        // Secondary: slight hue shift
        let sec_h = (h + 30.0) % 360.0;
        let secondary = hsl_to_hex(sec_h, (accent_s * 0.7).clamp(0.2, 0.7), 0.65);

        // Neutral dark surface tones
        let surface_h = h;
        let surface = hsl_to_hex(surface_h, 0.15, 0.09);
        let surface_variant = hsl_to_hex(surface_h, 0.18, 0.16);
        let background = hsl_to_hex(surface_h, 0.20, 0.05);
        let outline = hsl_to_hex(surface_h, 0.12, 0.35);
        let on_surface = "#F8FAFC".to_string();

        let active_border = format!("rgba({}, {}, {}, 0.95)", r, g, b);
        let inactive_border = "rgba(40, 48, 66, 0.60)".to_string();

        Self {
            primary,
            on_primary,
            primary_container,
            on_primary_container,
            secondary,
            surface,
            on_surface,
            surface_variant,
            outline,
            background,
            active_border,
            inactive_border,
        }
    }

    /// Generate palette from hex string (e.g., "#38BDF8" or "38BDF8")
    pub fn from_hex(hex: &str) -> Result<Self, ThemeError> {
        let clean = hex.trim().trim_start_matches('#');
        if clean.len() != 6 {
            return Err(ThemeError::InvalidHex(hex.to_string()));
        }

        let r = u8::from_str_radix(&clean[0..2], 16)
            .map_err(|_| ThemeError::InvalidHex(hex.to_string()))?;
        let g = u8::from_str_radix(&clean[2..4], 16)
            .map_err(|_| ThemeError::InvalidHex(hex.to_string()))?;
        let b = u8::from_str_radix(&clean[4..6], 16)
            .map_err(|_| ThemeError::InvalidHex(hex.to_string()))?;

        Ok(Self::from_rgb(r, g, b))
    }

    /// Extract dominant color from wallpaper image and build palette
    pub fn from_wallpaper<P: AsRef<Path>>(path: P) -> Result<Self, ThemeError> {
        let img = image::open(path)?;
        // Downsample to fast thumbnail (64x64) for instantaneous palette extraction (<5ms)
        let thumb = img.resize_exact(64, 64, FilterType::Nearest);

        let mut color_histogram: HashMap<(u8, u8, u8), u32> = HashMap::new();

        for (_x, _y, pixel) in thumb.pixels() {
            let [r, g, b, _a] = pixel.0;

            // Quantize to 16 levels per channel to cluster nearby tones
            let qr = (r / 16) * 16 + 8;
            let qg = (g / 16) * 16 + 8;
            let qb = (b / 16) * 16 + 8;

            let (_h, s, l) = rgb_to_hsl(qr, qg, qb);

            // Skip extreme darkness (black) or extreme brightness (white)
            if l < 0.10 || l > 0.92 {
                continue;
            }

            // Weight vibrant pixels higher than washed out pixels
            let weight = (s * 10.0) as u32 + 1;
            *color_histogram.entry((qr, qg, qb)).or_insert(0) += weight;
        }

        // Find most weighted color bucket
        let dominant = color_histogram
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(rgb, _)| rgb)
            .unwrap_or((56, 189, 248)); // Fallback sky blue

        Ok(Self::from_rgb(dominant.0, dominant.1, dominant.2))
    }

    /// Export palette as CSS variables for GTK4 and Web styling
    pub fn to_css(&self) -> String {
        format!(
            ":root {{\n  --nexus-primary: {0};\n  --nexus-on-primary: {1};\n  --nexus-primary-container: {2};\n  --nexus-on-primary-container: {3};\n  --nexus-secondary: {4};\n  --nexus-surface: {5};\n  --nexus-on-surface: {6};\n  --nexus-surface-variant: {7};\n  --nexus-outline: {8};\n  --nexus-background: {9};\n}}\n",
            self.primary,
            self.on_primary,
            self.primary_container,
            self.on_primary_container,
            self.secondary,
            self.surface,
            self.on_surface,
            self.surface_variant,
            self.outline,
            self.background,
        )
    }

    /// Export palette as Hyprland config snippet (e.g. col.active_border)
    pub fn to_hyprland_snippet(&self) -> String {
        // Extract pure hex without '#' for hyprland rgb(xxxxxx) format
        let primary_hex = self.primary.trim_start_matches('#');
        let secondary_hex = self.secondary.trim_start_matches('#');
        format!(
            "general {{\n    col.active_border = rgb({0}) rgb({1}) 45deg\n    col.inactive_border = rgba(30293bee)\n}}\n",
            primary_hex, secondary_hex
        )
    }
}

/// Helper: Convert RGB (0..=255) to HSL (H: 0..360, S: 0..1, L: 0..1)
fn rgb_to_hsl(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
    let rf = r as f32 / 255.0;
    let gf = g as f32 / 255.0;
    let bf = b as f32 / 255.0;

    let max = rf.max(gf).max(bf);
    let min = rf.min(gf).min(bf);
    let delta = max - min;

    let l = (max + min) / 2.0;

    if delta == 0.0 {
        return (0.0, 0.0, l);
    }

    let s = if l > 0.5 {
        delta / (2.0 - max - min)
    } else {
        delta / (max + min)
    };

    let mut h = if max == rf {
        (gf - bf) / delta + (if gf < bf { 6.0 } else { 0.0 })
    } else if max == gf {
        (bf - rf) / delta + 2.0
    } else {
        (rf - gf) / delta + 4.0
    };

    h *= 60.0;
    (h, s, l)
}

/// Helper: Convert HSL back to hex string "#RRGGBB"
fn hsl_to_hex(h: f32, s: f32, l: f32) -> String {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - (((h / 60.0) % 2.0) - 1.0).abs());
    let m = l - c / 2.0;

    let (rf, gf, bf) = match h as u32 {
        0..=59 => (c, x, 0.0),
        60..=119 => (x, c, 0.0),
        120..=179 => (0.0, c, x),
        180..=239 => (0.0, x, c),
        240..=299 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    let r = ((rf + m) * 255.0).round().clamp(0.0, 255.0) as u8;
    let g = ((gf + m) * 255.0).round().clamp(0.0, 255.0) as u8;
    let b = ((bf + m) * 255.0).round().clamp(0.0, 255.0) as u8;

    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palette_from_hex() {
        let palette = NexusPalette::from_hex("#38BDF8").expect("Parse valid hex");
        assert!(palette.primary.starts_with('#'));
        assert!(palette.surface.starts_with('#'));
        assert!(palette.background.starts_with('#'));
    }

    #[test]
    fn test_palette_from_invalid_hex() {
        assert!(NexusPalette::from_hex("invalid").is_err());
        assert!(NexusPalette::from_hex("#12345").is_err());
    }

    #[test]
    fn test_palette_hyprland_snippet() {
        let palette = NexusPalette::default();
        let snippet = palette.to_hyprland_snippet();
        assert!(snippet.contains("col.active_border"));
        assert!(snippet.contains("rgb("));
    }

    #[test]
    fn test_palette_from_synthetic_image() {
        use image::{ImageBuffer, Rgb};
        // Create 64x64 synthetic vibrant red/orange image
        let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_fn(64, 64, |x, _y| {
            if x < 32 {
                Rgb([240, 80, 20])
            } else {
                Rgb([20, 20, 20])
            }
        });

        let tmp_path = std::env::temp_dir().join("nexus_test_wallpaper.png");
        img.save(&tmp_path).expect("Save test image");

        let palette = NexusPalette::from_wallpaper(&tmp_path).expect("Extract from image");
        assert!(palette.primary.starts_with('#'));
        let _ = std::fs::remove_file(tmp_path);
    }
}
