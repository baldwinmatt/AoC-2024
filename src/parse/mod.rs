pub trait ToDigit {
    fn to_digit(&self) -> Option<u8>;
}

impl ToDigit for u8 {
    fn to_digit(&self) -> Option<u8> {
        if self.is_ascii_digit() {
            return Some(*self - b'0');
        }
        None
    }
}

pub trait Parsable<T>: Iterator {
    fn next_number(&mut self) -> Option<T>;
    fn next_number_strict(&mut self) -> (Option<T>, Option<u8>);
}

macro_rules! parsable_number {
    ($type:ident) => {
        impl<T: Iterator<Item = u8>> Parsable<$type> for T {
            fn next_number(&mut self) -> Option<$type> {
                let mut value: Option<$type> = None;
                for byte in self {
                    if let Some(digit) = byte.to_digit() {
                        if let Some(current) = value {
                            value = Some(current * 10 + digit as $type);
                        } else {
                            value = Some(digit as $type);
                        }
                    } else if value.is_some() {
                        return value;
                    }
                }

                value
            }
            fn next_number_strict(&mut self) -> (Option<$type>, Option<u8>) {
                let mut value: Option<$type> = None;
                for byte in self {
                    if let Some(digit) = byte.to_digit() {
                        if let Some(current) = value {
                            value = Some(current * 10 + digit as $type);
                        } else {
                            value = Some(digit as $type);
                        }
                    } else {
                        return (value, Some(byte));
                    }
                }

                (value, None)
            }
        }
    };
}

parsable_number!(u8);
parsable_number!(i8);
parsable_number!(u16);
parsable_number!(i16);
parsable_number!(u32);
parsable_number!(i32);
parsable_number!(u64);
parsable_number!(i64);
parsable_number!(u128);
parsable_number!(i128);
parsable_number!(usize);
parsable_number!(isize);
