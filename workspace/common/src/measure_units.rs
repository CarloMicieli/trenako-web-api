use rust_decimal::Decimal;
use rust_decimal_macros::*;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct MeasureUnitConverter {
    from: MeasureUnit,
    to: MeasureUnit,
    ratio: Decimal,
}

impl MeasureUnitConverter {
    pub fn new(from: MeasureUnit, to: MeasureUnit, ratio: Decimal) -> Self {
        MeasureUnitConverter { from, to, ratio }
    }

    pub fn same_unit(mu: MeasureUnit) -> Self {
        MeasureUnitConverter {
            from: mu,
            to: mu,
            ratio: 1.into(),
        }
    }

    pub fn convert(&self, value: Decimal) -> Decimal {
        value * self.ratio
    }

    pub fn and_then<U, F>(self, value: Decimal, op: F) -> U
    where
        F: FnOnce(Decimal) -> U,
    {
        let converted: Decimal = value * self.ratio;
        op(converted)
    }
}

impl fmt::Display for MeasureUnitConverter {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Converter from {:?} to {:?}", self.from, self.to)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MeasureUnit {
    Millimeters,
    Inches,
    Miles,
    Kilometers,
}

impl MeasureUnit {
    pub fn symbol(&self) -> &str {
        match self {
            MeasureUnit::Miles => "mi",
            MeasureUnit::Inches => "in",
            MeasureUnit::Millimeters => "mm",
            MeasureUnit::Kilometers => "km",
        }
    }

    pub fn to(&self, other: MeasureUnit) -> MeasureUnitConverter {
        match (self, other) {
            (MeasureUnit::Inches, MeasureUnit::Millimeters) => {
                MeasureUnitConverter::new(
                    MeasureUnit::Inches,
                    MeasureUnit::Millimeters,
                    MeasureUnit::INCHES_TO_MILLIMETERS,
                )
            }
            (MeasureUnit::Millimeters, MeasureUnit::Inches) => {
                MeasureUnitConverter::new(
                    MeasureUnit::Millimeters,
                    MeasureUnit::Inches,
                    MeasureUnit::MILLIMETERS_TO_INCHES,
                )
            }
            (MeasureUnit::Kilometers, MeasureUnit::Miles) => {
                MeasureUnitConverter::new(
                    MeasureUnit::Kilometers,
                    MeasureUnit::Miles,
                    MeasureUnit::KILOMETERS_TO_MILES,
                )
            }
            (MeasureUnit::Miles, MeasureUnit::Kilometers) => {
                MeasureUnitConverter::new(
                    MeasureUnit::Miles,
                    MeasureUnit::Kilometers,
                    MeasureUnit::MILES_TO_KILOMETERS,
                )
            }
            (MeasureUnit::Inches, MeasureUnit::Inches) => {
                MeasureUnitConverter::same_unit(MeasureUnit::Inches)
            }
            (MeasureUnit::Millimeters, MeasureUnit::Millimeters) => {
                MeasureUnitConverter::same_unit(MeasureUnit::Millimeters)
            }
            (MeasureUnit::Kilometers, MeasureUnit::Kilometers) => {
                MeasureUnitConverter::same_unit(MeasureUnit::Kilometers)
            }
            (MeasureUnit::Miles, MeasureUnit::Miles) => {
                MeasureUnitConverter::same_unit(MeasureUnit::Inches)
            }
            _ => panic!("invalid converter"),
        }
    }

    const INCHES_TO_MILLIMETERS: Decimal = dec!(25.4);
    const MILLIMETERS_TO_INCHES: Decimal = dec!(0.0393701);
    const MILES_TO_KILOMETERS: Decimal = dec!(1.60934);
    const KILOMETERS_TO_MILES: Decimal = dec!(0.621371);
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     mod measure_units_tests {
//         use super::*;
//
//         #[test]
//         fn measure_unit_symbol_should_return_the_symbol() {
//             assert_eq!(MeasureUnit::Miles.symbol(), "mi");
//             assert_eq!(MeasureUnit::Millimeters.symbol(), "mm");
//             assert_eq!(MeasureUnit::Inches.symbol(), "in");
//             assert_eq!(MeasureUnit::Kilometers.symbol(), "km");
//         }
//
//         #[test]
//         fn measure_unit_should_convert_between_units() {
//             let converted = MeasureUnit::Inches
//                 .to(MeasureUnit::Millimeters)
//                 .convert(10.into());
//             assert_eq!(converted, 0.into());
//         }
//     }
// }
