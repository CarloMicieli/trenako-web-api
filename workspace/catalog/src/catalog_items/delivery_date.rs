use std::fmt;
use std::str;
use thiserror::Error;

pub type Quarter = u8;
pub type Year = i32;

#[derive(Debug, PartialEq, Eq)]
pub enum DeliveryDate {
    ByYear(Year),
    ByQuarter(Year, Quarter),
}

impl DeliveryDate {
    /// Creates a new delivery date without the quarter
    pub fn by_year(year: Year) -> Self {
        DeliveryDate::ByYear(year)
    }

    /// Creates a new delivery date with the quarter information
    pub fn by_quarter(year: Year, quarter: Quarter) -> Self {
        DeliveryDate::ByQuarter(year, quarter)
    }

    pub fn year(&self) -> Year {
        match self {
            DeliveryDate::ByQuarter(y, _) => *y,
            DeliveryDate::ByYear(y) => *y,
        }
    }

    pub fn quarter(&self) -> Option<Quarter> {
        match self {
            DeliveryDate::ByQuarter(_, q) => Some(*q),
            DeliveryDate::ByYear(_) => None,
        }
    }

    fn parse_year(s: &str) -> Result<Year, DeliveryDateParseError> {
        let year = s
            .parse::<Year>()
            .map_err(|_| DeliveryDateParseError::InvalidYearValue)?;
        if year < 1900 && year >= 2999 {
            return Err(DeliveryDateParseError::InvalidYearValue);
        }

        Ok(year)
    }

    fn parse_quarter(s: &str) -> Result<Quarter, DeliveryDateParseError> {
        if s.len() != 2 {
            return Err(DeliveryDateParseError::InvalidQuarterValue);
        }

        let quarter = s[1..]
            .parse::<Quarter>()
            .map_err(|_| DeliveryDateParseError::InvalidQuarterValue)?;
        if quarter < 1 && quarter >= 4 {
            return Err(DeliveryDateParseError::InvalidQuarterValue);
        }

        Ok(quarter)
    }
}

impl str::FromStr for DeliveryDate {
    type Err = DeliveryDateParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(DeliveryDateParseError::EmptyValue);
        }

        if s.contains('/') {
            let tokens: Vec<&str> = s.split_terminator('/').collect();
            if tokens.len() != 2 {
                return Err(DeliveryDateParseError::InvalidByQuarterValue);
            }

            let year = DeliveryDate::parse_year(tokens[0])?;
            let quarter = DeliveryDate::parse_quarter(tokens[1])?;
            Ok(DeliveryDate::ByQuarter(year, quarter))
        } else {
            let year = DeliveryDate::parse_year(s)?;
            Ok(DeliveryDate::ByYear(year))
        }
    }
}

impl fmt::Display for DeliveryDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeliveryDate::ByQuarter(y, q) => write!(f, "{}/Q{}", y, q),
            DeliveryDate::ByYear(y) => write!(f, "{}", y),
        }
    }
}

#[derive(Debug, Error)]
pub enum DeliveryDateParseError {
    #[error("Delivery date cannot be empty")]
    EmptyValue,
    #[error("Invalid delivery date by quarter")]
    InvalidByQuarterValue,
    #[error("Delivery date year component is not valid")]
    InvalidYearValue,
    #[error("Delivery date quarter component is not valid")]
    InvalidQuarterValue,
}

#[cfg(test)]
mod tests {
    use super::*;

    mod delivery_date_tests {
        use super::*;

        #[test]
        fn it_should_parse_string_as_delivery_dates() {
            let dd1 = "2020/Q1".parse::<DeliveryDate>();
            let dd2 = "2020".parse::<DeliveryDate>();

            assert!(dd1.is_ok());

            let dd1_val = dd1.unwrap();
            assert_eq!(2020, dd1_val.year());
            assert_eq!(Some(1), dd1_val.quarter());

            assert!(dd2.is_ok());

            let dd2_val = dd2.unwrap();
            assert_eq!(2020, dd2_val.year());
            assert_eq!(None, dd2_val.quarter());
        }

        #[test]
        fn it_should_produce_string_representations_from_delivery_dates() {
            let dd1 = "2020/Q1".parse::<DeliveryDate>().unwrap();
            let dd2 = "2020".parse::<DeliveryDate>().unwrap();

            assert_eq!("2020/Q1", dd1.to_string());
            assert_eq!("2020", dd2.to_string());
        }
    }
}
