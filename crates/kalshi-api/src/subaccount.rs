use std::fmt;

/// List calls whose omitted query means every subaccount.
///
/// `All` omits the query parameter. `Only` sends one subaccount, and `0` is the primary account.
/// A value above 63 is rejected.
///
/// The official Python SDK leaves this query optional. This typed API requires the enum so
/// "every subaccount" and "one subaccount" are different values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubaccountFilter {
    All,
    Only(u8),
}

/// Calls whose omitted query means the primary account.
///
/// `Primary` sends `0`. `Numbered` is `1..=63`. A missing argument cannot stand in for the primary account.
///
/// The official Python SDK leaves this query optional. This typed API requires the enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Subaccount {
    Primary,
    Numbered(u8),
}

/// Body field for an order or a batch cancellation.
///
/// `Primary` sends `0`. `Numbered` sends `1..=63`. `KeyRestricted` omits the field, which is what a
/// key locked to one subaccount must do.
///
/// The official Python SDK leaves this field optional. [`crate::CreateOrderV2::build`] and
/// [`crate::BatchCancelItem::build`] require the enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubaccountChoice {
    Primary,
    Numbered(u8),
    KeyRestricted,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubaccountError {
    pub value: i64,
}

impl fmt::Display for SubaccountError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "subaccount {} is outside 0 for primary and 1-63 for numbered subaccounts",
            self.value
        )
    }
}

impl std::error::Error for SubaccountError {}

impl SubaccountFilter {
    /// `All` omits the query. `Only(0)` sends the primary account and is not the same as `All`.
    pub fn query_value(self) -> Result<Option<i64>, SubaccountError> {
        match self {
            Self::All => Ok(None),
            Self::Only(n) if n <= 63 => Ok(Some(i64::from(n))),
            Self::Only(n) => Err(SubaccountError {
                value: i64::from(n),
            }),
        }
    }
}

impl Subaccount {
    /// Always sends a number. `Primary` is `0`, not an omitted query.
    pub fn query_value(self) -> Result<i64, SubaccountError> {
        match self {
            Self::Primary => Ok(0),
            Self::Numbered(n) if (1..=63).contains(&n) => Ok(i64::from(n)),
            Self::Numbered(n) => Err(SubaccountError {
                value: i64::from(n),
            }),
        }
    }
}

impl SubaccountChoice {
    /// `KeyRestricted` is the absent JSON field. `Primary` is `0`.
    pub fn body_value(self) -> Result<Option<i64>, SubaccountError> {
        match self {
            Self::Primary => Ok(Some(0)),
            Self::Numbered(n) if (1..=63).contains(&n) => Ok(Some(i64::from(n))),
            Self::Numbered(n) => Err(SubaccountError {
                value: i64::from(n),
            }),
            Self::KeyRestricted => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Subaccount, SubaccountChoice, SubaccountFilter};

    #[test]
    fn filter_all_omits_and_only_zero_is_primary() {
        assert_eq!(SubaccountFilter::All.query_value().unwrap(), None);
        assert_eq!(SubaccountFilter::Only(0).query_value().unwrap(), Some(0));
        assert_eq!(SubaccountFilter::Only(4).query_value().unwrap(), Some(4));
        assert!(SubaccountFilter::Only(64).query_value().is_err());
    }

    #[test]
    fn primary_sends_zero_and_numbered_starts_at_one() {
        assert_eq!(Subaccount::Primary.query_value().unwrap(), 0);
        assert_eq!(Subaccount::Numbered(1).query_value().unwrap(), 1);
        assert!(Subaccount::Numbered(0).query_value().is_err());
        assert!(Subaccount::Numbered(64).query_value().is_err());
    }

    #[test]
    fn key_restricted_omits_the_body_field() {
        assert_eq!(SubaccountChoice::Primary.body_value().unwrap(), Some(0));
        assert_eq!(SubaccountChoice::Numbered(2).body_value().unwrap(), Some(2));
        assert_eq!(SubaccountChoice::KeyRestricted.body_value().unwrap(), None);
        assert!(SubaccountChoice::Numbered(0).body_value().is_err());
        assert!(SubaccountChoice::Numbered(99).body_value().is_err());
    }
}
