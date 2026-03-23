use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum CreditCard {
    #[serde(rename = "amex")]
    #[default]
    AmericanExpress,
    #[serde(rename = "argencard")]
    Argencard,
    #[serde(rename = "cabal")]
    Cabal,
    #[serde(rename = "cencosud")]
    Cencosud,
    #[serde(rename = "diners")]
    DinersClub,
    #[serde(rename = "discover")]
    Discover,
    #[serde(rename = "elo")]
    Elo,
    #[serde(rename = "hipercard")]
    Hipercard,
    #[serde(rename = "jcb")]
    JCB,
    #[serde(rename = "mastercard")]
    Mastercard,
    #[serde(rename = "naranja")]
    Naranja,
    #[serde(rename = "targeta-shopping")]
    TarjetaShopping,
    #[serde(rename = "unionpay")]
    UnionPay,
    #[serde(rename = "visa")]
    Visa,
    #[serde(rename = "mir")]
    MIR,
    #[serde(rename = "maestro")]
    Maestro,
    #[serde(rename = "rupay")]
    Rupay,
}

impl CreditCard {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            CreditCard::AmericanExpress => "amex",
            CreditCard::Argencard => "argencard",
            CreditCard::Cabal => "cabal",
            CreditCard::Cencosud => "cencosud",
            CreditCard::DinersClub => "diners",
            CreditCard::Discover => "discover",
            CreditCard::Elo => "elo",
            CreditCard::Hipercard => "hipercard",
            CreditCard::JCB => "jcb",
            CreditCard::Mastercard => "mastercard",
            CreditCard::Naranja => "naranja",
            CreditCard::TarjetaShopping => "targeta-shopping",
            CreditCard::UnionPay => "unionpay",
            CreditCard::Visa => "visa",
            CreditCard::MIR => "mir",
            CreditCard::Maestro => "maestro",
            CreditCard::Rupay => "rupay",
        }
    }
}

impl std::fmt::Display for CreditCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
