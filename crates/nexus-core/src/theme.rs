use serde::{Deserialize, Serialize};

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
        }
    }
}

impl NexusPalette {
    /// Export palette as CSS variables for GTK4 and Web/QML styling.
    pub fn to_css(&self) -> String {
        format!(
            ":root {{\n  --nexus-primary: {0};\n  --nexus-on-primary: {1};\n  --nexus-primary-container: {2};\n  --nexus-surface: {3};\n  --nexus-on-surface: {4};\n  --nexus-surface-variant: {5};\n  --nexus-outline: {6};\n  --nexus-background: {7};\n}}\n",
            self.primary,
            self.on_primary,
            self.primary_container,
            self.surface,
            self.on_surface,
            self.surface_variant,
            self.outline,
            self.background,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palette_css_export() {
        let palette = NexusPalette::default();
        let css = palette.to_css();
        assert!(css.contains("--nexus-primary: #38BDF8;"));
        assert!(css.contains("--nexus-background: #020617;"));
    }
}
