#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Numeric {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl std::fmt::Display for Numeric {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Numeric::Zero => write!(formatter, "0"),
            Numeric::One => write!(formatter, "1"),
            Numeric::Two => write!(formatter, "2"),
            Numeric::Three => write!(formatter, "3"),
            Numeric::Four => write!(formatter, "4"),
            Numeric::Five => write!(formatter, "5"),
            Numeric::Six => write!(formatter, "6"),
            Numeric::Seven => write!(formatter, "7"),
            Numeric::Eight => write!(formatter, "8"),
            Numeric::Nine => write!(formatter, "9"),
        }
    }
}

impl TryFrom<char> for Numeric {
    type Error = char;
    fn try_from(value: char) -> Result<Self, char> {
        match value {
            '0' => Ok(Numeric::Zero),
            '1' => Ok(Numeric::One),
            '2' => Ok(Numeric::Two),
            '3' => Ok(Numeric::Three),
            '4' => Ok(Numeric::Four),
            '5' => Ok(Numeric::Five),
            '6' => Ok(Numeric::Six),
            '7' => Ok(Numeric::Seven),
            '8' => Ok(Numeric::Eight),
            '9' => Ok(Numeric::Nine),
            wrong_char => Err(wrong_char),
        }
    }
}
