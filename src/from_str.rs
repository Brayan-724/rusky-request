use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub trait MyFromStr: FromStr {
    type Err_;

    fn from_str(s: &str) -> Result<Self, Self::Err_>;
}

#[derive(Debug)]
pub struct BoolParseErr;

impl BoolParseErr {
    pub fn get_err_message() -> &'static str {
        "Provided value is invalid"
    }
}

impl Display for BoolParseErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        BoolParseErr::get_err_message().fmt(f)
    }
}

impl MyFromStr for bool {
    type Err_ = BoolParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err_> {
        match s.to_lowercase().as_str() {
            "yes" | "y" | "true" => Ok(true),
            "no" | "n" | "false" => Ok(false),
            _ => Err(BoolParseErr),
        }
    }
}

macro_rules! pre_from_str {
    ($($t:tt),+$(,)?) => {
        $(
            impl MyFromStr for $t {
                type Err_ = <$t as FromStr>::Err;

                fn from_str(s: &str) -> Result<Self, Self::Err_> {
                    FromStr::from_str(s)
                }
            }
        )+
    };
}

pre_from_str!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64, char, String);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn basic() {
        let my_val: bool = MyFromStr::from_str("yEs").unwrap();
        assert_eq!(my_val, true);

        let my_val: bool = MyFromStr::from_str("Y").unwrap();
        assert_eq!(my_val, true);

        let my_val: bool = MyFromStr::from_str("TrUe").unwrap();
        assert_eq!(my_val, true);

        let my_val: bool = MyFromStr::from_str("nO").unwrap();
        assert_eq!(my_val, false);

        let my_val: bool = MyFromStr::from_str("n").unwrap();
        assert_eq!(my_val, false);

        let my_val: bool = MyFromStr::from_str("fAlsE").unwrap();
        assert_eq!(my_val, false);
    }

    #[test]
    pub fn errors() {
        let my_val = <bool as MyFromStr>::from_str("nou");
        assert!(my_val.is_err(), "'nou' Will throw error");

        let my_val = <bool as MyFromStr>::from_str("truw");
        assert!(my_val.is_err(), "'truw' Will throw error");

        let my_val = <bool as MyFromStr>::from_str("folse");
        assert!(my_val.is_err(), "'folse' Will throw error");

        let my_val = <bool as MyFromStr>::from_str("yis");
        assert!(my_val.is_err(), "'yis' Will throw error");
    }
}
