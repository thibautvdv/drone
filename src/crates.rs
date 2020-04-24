//! Supported Drone crates.

/// Drone platform crates.
#[allow(missing_docs)]
#[derive(Debug)]
pub enum Platform {
    CortexM,
}

/// Drone register and interrupt binding crates.
#[allow(missing_docs)]
#[derive(Debug)]
pub enum Bindings {
    Nrf,
    Stm32,
    TiSl,
}

impl Platform {
    /// Returns the name in kebab-case.
    pub fn kebab_name(&self) -> &str {
        match self {
            Self::CortexM => "cortex-m",
        }
    }

    /// Returns the name in underscore-case.
    pub fn underscore_name(&self) -> &str {
        match self {
            Self::CortexM => "cortex_m",
        }
    }

    /// Returns the name of the configuration flag.
    pub fn flag_name(&self) -> &str {
        match self {
            Self::CortexM => "cortex_m_core",
        }
    }
}

impl Bindings {
    /// Returns the name in kebab-case.
    pub fn kebab_name(&self) -> &str {
        match self {
            Self::Nrf => "nrf",
            Self::Stm32 => "stm32",
            Self::TiSl => "ti-sl",
        }
    }

    /// Returns the name in underscore-case.
    pub fn underscore_name(&self) -> &str {
        match self {
            Self::Nrf => "nrf",
            Self::Stm32 => "stm32",
            Self::TiSl => "ti_sl",
        }
    }

    /// Returns the name of the configuration flag.
    pub fn flag_name(&self) -> &str {
        match self {
            Self::Nrf => "nrf_mcu",
            Self::Stm32 => "stm32_mcu",
            Self::TiSl => "ti_sl_mcu",
        }
    }
}
