use heck::ShoutySnakeCase;
use std::fmt;
use std::str;

/// The control method for this railway model.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Control {
    /// The model can be fitted with a dcc decoder.
    DccReady,

    /// The model has a dcc decoder installed.
    Dcc,

    /// The model has a dcc decoder installed with the sound module.
    DccSound,

    DccFitted,
}

impl fmt::Display for Control {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", s.to_shouty_snake_case())
    }
}

impl str::FromStr for Control {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err("Control value cannot be blank");
        }

        match s {
            "DCC_READY" => Ok(Control::DccReady),
            "DCC" => Ok(Control::Dcc),
            "DCC_SOUND" => Ok(Control::DccSound),
            _ => Err("Invalid value for control [allowed values are DCC, DCC_READY, DCC_SOUND]"),
        }
    }
}

/// NMRA and NEM Connectors for digital control (DCC)
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DccInterface {
    Nem651,
    Nem652,
    Plux8,
    Plux16,
    Plux22,
    Next18,
    Mtc21,
}

impl str::FromStr for DccInterface {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err("Dcc interface value cannot be blank");
        }

        match s {
            "NEM_651" => Ok(DccInterface::Nem651),
            "NEM_652" => Ok(DccInterface::Nem652),
            "PLUX_8" => Ok(DccInterface::Plux8),
            "PLUX_16" => Ok(DccInterface::Plux16),
            "PLUX_22" => Ok(DccInterface::Plux22),
            "NEXT_18" => Ok(DccInterface::Next18),
            "MTC_21" => Ok(DccInterface::Mtc21),
            _ => Err("Invalid value for dcc interfaces"),
        }
    }
}

impl fmt::Display for DccInterface {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", s.to_shouty_snake_case())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod dcc_interface_tests {
        use super::*;

        #[test]
        fn it_should_parse_string_as_dcc_interfaces() {
            let dcc = "NEM_652".parse::<DccInterface>();
            assert!(dcc.is_ok());
            assert_eq!(dcc.unwrap(), DccInterface::Nem652);
        }

        #[test]
        fn it_should_fail_to_parse_invalid_string_as_dcc_interfaces() {
            let blank = "".parse::<DccInterface>();
            assert!(blank.is_err());

            let invalid = "invalid".parse::<DccInterface>();
            assert!(invalid.is_err());
        }

        #[test]
        fn it_should_display_dcc_interfaces() {
            let dcc = DccInterface::Nem652;
            assert_eq!("NEM652", dcc.to_string());
        }
    }

    mod control_tests {
        use super::*;

        #[test]
        fn it_should_parse_string_as_controls() {
            let c = "DCC_READY".parse::<Control>();
            assert!(c.is_ok());
            assert_eq!(c.unwrap(), Control::DccReady);
        }

        #[test]
        fn it_should_fail_to_parse_invalid_value_as_controls() {
            let blank = "".parse::<Control>();
            assert!(blank.is_err());

            let invalid = "invalid".parse::<Control>();
            assert!(invalid.is_err());
        }

        #[test]
        fn it_should_display_controls() {
            let c = Control::DccReady;
            assert_eq!("DCC_READY", c.to_string());
        }
    }
}
