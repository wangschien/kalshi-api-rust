use rust_decimal::Decimal;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixedPointError {
    pub input: String,
}

impl fmt::Display for FixedPointError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "malformed fixed-point value {}", self.input)
    }
}

impl std::error::Error for FixedPointError {}

/// Contract count in hundredths of a contract. `"2.50"` is 250.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QtyCenti(pub i64);

impl QtyCenti {
    pub fn parse(input: &str) -> Result<Self, FixedPointError> {
        let negative = input.starts_with('-');
        let body = input.strip_prefix('-').unwrap_or(input);
        if body.is_empty() || body.starts_with('+') {
            return Err(FixedPointError {
                input: input.to_owned(),
            });
        }
        let (whole, frac) = split_decimal(body).map_err(|_| FixedPointError {
            input: input.to_owned(),
        })?;
        if frac.len() > 2 {
            return Err(FixedPointError {
                input: input.to_owned(),
            });
        }
        let whole_n: i64 = whole.parse().map_err(|_| FixedPointError {
            input: input.to_owned(),
        })?;
        let mut frac_padded = frac.to_owned();
        while frac_padded.len() < 2 {
            frac_padded.push('0');
        }
        let frac_n: i64 = frac_padded.parse().map_err(|_| FixedPointError {
            input: input.to_owned(),
        })?;
        let magnitude = whole_n
            .checked_mul(100)
            .and_then(|v| v.checked_add(frac_n))
            .ok_or_else(|| FixedPointError {
                input: input.to_owned(),
            })?;
        let value = if negative { -magnitude } else { magnitude };
        Ok(Self(value))
    }

    pub fn wire(self) -> String {
        format_centi(self.0)
    }
}

pub fn parse_decimal(input: &str) -> Result<Decimal, FixedPointError> {
    Decimal::from_str(input).map_err(|_| FixedPointError {
        input: input.to_owned(),
    })
}

pub fn decimal_wire(value: Decimal, max_scale: u32) -> Result<String, FixedPointError> {
    if value.scale() > max_scale {
        return Err(FixedPointError {
            input: value.to_string(),
        });
    }
    let mut fixed = value;
    fixed.rescale(max_scale);
    Ok(fixed.to_string())
}

fn split_decimal(body: &str) -> Result<(&str, &str), ()> {
    if body.chars().any(|ch| !ch.is_ascii_digit() && ch != '.') {
        return Err(());
    }
    match body.split_once('.') {
        Some((whole, frac)) if !whole.is_empty() && frac.chars().all(|ch| ch.is_ascii_digit()) => {
            Ok((whole, frac))
        }
        None if !body.is_empty() => Ok((body, "")),
        _ => Err(()),
    }
}

pub fn format_centi(value: i64) -> String {
    let negative = value < 0;
    let magnitude = value.unsigned_abs();
    let whole = magnitude / 100;
    let frac = magnitude % 100;
    if negative {
        format!("-{whole}.{frac:02}")
    } else {
        format!("{whole}.{frac:02}")
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;
    use std::str::FromStr;

    use super::decimal_wire;

    #[test]
    fn price_wire_keeps_four_decimal_places() {
        assert_eq!(
            decimal_wire(Decimal::from_str("0.30").unwrap(), 4).unwrap(),
            "0.3000"
        );
        assert_eq!(
            decimal_wire(Decimal::from_str("0.3").unwrap(), 4).unwrap(),
            "0.3000"
        );
        assert_eq!(
            decimal_wire(Decimal::from_str("0.5600").unwrap(), 4).unwrap(),
            "0.5600"
        );
        assert!(decimal_wire(Decimal::from_str("0.12345").unwrap(), 4).is_err());
    }
}
