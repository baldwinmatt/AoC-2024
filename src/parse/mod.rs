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

pub trait Parseable<T>: Iterator {
    fn next_number(&mut self) -> Option<T>;
}

macro_rules! Parseable_number {
    ($type:ident) => {
        impl<T: Iterator<Item = u8>> Parseable<$type> for T {
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
        }
    };
}

macro_rules! Parseable_signed_number {
    ($type:ident) => {
        impl<T: Iterator<Item = u8>> Parseable<$type> for T {
            fn next_number(&mut self) -> Option<$type> {
                let mut negative = false;
                let mut value: Option<$type> = None;
                for byte in self {
                    if let Some(digit) = byte.to_digit() {
                        if let Some(current) = value {
                            value = Some(current * 10 + digit as $type);
                        } else {
                            value = Some(digit as $type);
                        }
                    } else if let Some(value) = value {
                        if negative {
                            return Some(-value);
                        }
                        return Some(value);
                    } else if byte == b'-' {
                        negative = true;
                    } else {
                        negative = false;
                    }
                }

                if let Some(value) = value {
                    if negative {
                        return Some(-value);
                    }
                    return Some(value);
                }
                None
            }
        }
    };
}

Parseable_number!(u8);
Parseable_signed_number!(i8);
Parseable_number!(u16);
Parseable_signed_number!(i16);
Parseable_number!(u32);
Parseable_signed_number!(i32);
Parseable_number!(u64);
Parseable_signed_number!(i64);
Parseable_number!(u128);
Parseable_signed_number!(i128);
Parseable_number!(usize);
Parseable_signed_number!(isize);
