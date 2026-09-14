use serde::{Deserialize, Serialize};
use std::fmt;

/// Operating tiers supported by NEXUS Desktop Environment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
#[repr(u8)]
pub enum NexusTier {
    /// Low footprint, hardware acceleration, minimal animations (<180MB RAM)
    #[serde(rename = "minimal")]
    Minimal = 1,
    /// Balanced experience, spring physics, launcher, widgets (<280MB RAM)
    #[default]
    #[serde(rename = "core")]
    Core = 2,
    /// Maximum fidelity, glassmorphism, live wallpaper, control center (<400MB RAM)
    #[serde(rename = "hyper")]
    Hyper = 3,
}

impl NexusTier {
    /// Convert an integer (1..=3) to a NexusTier.
    pub fn from_u8(val: u8) -> Option<Self> {
        match val {
            1 => Some(Self::Minimal),
            2 => Some(Self::Core),
            3 => Some(Self::Hyper),
            _ => None,
        }
    }

    /// Convert the tier to its numeric ID.
    pub fn to_u8(self) -> u8 {
        self as u8
    }

    /// Return human-readable label.
    pub fn label(self) -> &'static str {
        match self {
            Self::Minimal => "NEXUS Minimal",
            Self::Core => "NEXUS Core",
            Self::Hyper => "NEXUS Hyper",
        }
    }

    /// Whether desktop blur and frosted glass effects are enabled.
    pub fn has_glass_effects(self) -> bool {
        matches!(self, Self::Core | Self::Hyper)
    }

    /// Whether desktop widgets and dynamic hosts are active.
    pub fn has_widgets(self) -> bool {
        matches!(self, Self::Core | Self::Hyper)
    }

    /// Whether advanced control center drawer is active.
    pub fn has_control_center(self) -> bool {
        matches!(self, Self::Hyper)
    }
}

impl fmt::Display for NexusTier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.label())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tier_conversions() {
        assert_eq!(NexusTier::from_u8(1), Some(NexusTier::Minimal));
        assert_eq!(NexusTier::from_u8(2), Some(NexusTier::Core));
        assert_eq!(NexusTier::from_u8(3), Some(NexusTier::Hyper));
        assert_eq!(NexusTier::from_u8(0), None);
        assert_eq!(NexusTier::from_u8(4), None);

        assert_eq!(NexusTier::Minimal.to_u8(), 1);
        assert_eq!(NexusTier::Core.to_u8(), 2);
        assert_eq!(NexusTier::Hyper.to_u8(), 3);
    }

    #[test]
    fn test_tier_features() {
        assert!(!NexusTier::Minimal.has_glass_effects());
        assert!(NexusTier::Core.has_glass_effects());
        assert!(NexusTier::Hyper.has_glass_effects());

        assert!(!NexusTier::Minimal.has_control_center());
        assert!(!NexusTier::Core.has_control_center());
        assert!(NexusTier::Hyper.has_control_center());
    }
}
