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
            Self::Zero => write!(formatter, "0"),
            Self::One => write!(formatter, "1"),
            Self::Two => write!(formatter, "2"),
            Self::Three => write!(formatter, "3"),
            Self::Four => write!(formatter, "4"),
            Self::Five => write!(formatter, "5"),
            Self::Six => write!(formatter, "6"),
            Self::Seven => write!(formatter, "7"),
            Self::Eight => write!(formatter, "8"),
            Self::Nine => write!(formatter, "9"),
        }
    }
}

impl TryFrom<char> for Numeric {
    type Error = char;
    fn try_from(value: char) -> Result<Self, char> {
        match value {
            '0' => Ok(Self::Zero),
            '1' => Ok(Self::One),
            '2' => Ok(Self::Two),
            '3' => Ok(Self::Three),
            '4' => Ok(Self::Four),
            '5' => Ok(Self::Five),
            '6' => Ok(Self::Six),
            '7' => Ok(Self::Seven),
            '8' => Ok(Self::Eight),
            '9' => Ok(Self::Nine),
            wrong_char => Err(wrong_char),
        }
    }
}
