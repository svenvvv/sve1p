use binrw::{BinRead, BinResult, BinWrite, Endian};
use std::{
    fmt,
    io::{Read, Seek, Write},
};
use yore::code_pages::CP1251;

use crate::utils::cipher;

#[derive(Clone, Eq, PartialEq, Default)]
pub struct E1pString<LengthType, const DO_CIPHER: bool = true, const C_STR_LENGTH: bool = false>(
    pub String,
    std::marker::PhantomData<LengthType>,
);

impl<T, const CIPHER: bool, const HACK: bool> E1pString<T, CIPHER, HACK> {
    pub fn from(value: &str) -> E1pString<T, CIPHER, HACK> {
        E1pString(value.to_string(), std::marker::PhantomData)
    }
}

impl<T, const CIPHER: bool, const HACK: bool> BinRead for E1pString<T, CIPHER, HACK>
where
    for<'a> T: BinRead<Args<'a> = ()>,
    T: TryInto<usize>,
    <T as TryInto<usize>>::Error: std::fmt::Debug,
{
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: Endian,
        (): Self::Args<'_>,
    ) -> BinResult<Self> {
        let length_raw = T::read_options(reader, endian, ())?;
        let mut length: usize = length_raw.try_into().expect("Length exceeds usize bounds");

        // Inline strings in AST store the length for a C string (zero terminated)
        if HACK {
            length -= 1;
        }

        let mut values = Vec::with_capacity(length);
        for _ in 0..length {
            let val = <u8>::read_options(reader, endian, ())?;
            values.push(val);
        }

        if CIPHER {
            cipher::inplace(&mut values, cipher::INITIAL_E1P_KEY);
        }
        let string = CP1251.decode(&values).to_string();

        Ok(Self(string, std::marker::PhantomData))
    }
}

impl<T, const CIPHER: bool, const HACK: bool> BinWrite for E1pString<T, CIPHER, HACK>
where
    for<'a> T: BinWrite<Args<'a> = ()>,
    T: TryFrom<usize>,
    <T as TryFrom<usize>>::Error: std::fmt::Debug,
{
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        endian: Endian,
        args: Self::Args<'_>,
    ) -> BinResult<()> {
        let mut bytes = match CP1251.encode(&self.0) {
            Ok(str) => str.into_owned(),
            Err(err) => {
                panic!(
                    "Failed to encode string as CP 1251: {}\nContents: \"{}\"",
                    err, self.0
                );
            }
        };
        if CIPHER {
            cipher::inplace(&mut bytes, cipher::INITIAL_E1P_KEY);
        }

        let mut length = bytes.len();
        // Inline strings in AST store the length for a C string (zero terminated)
        if HACK {
            length += 1;
        }

        let length_raw = T::try_from(length)
            .expect("String byte length is too large to fit into the prefix type T");

        length_raw.write_options(writer, endian, args)?;
        bytes.write_options(writer, endian, args)?;

        Ok(())
    }
}

impl<T, const CIPHER: bool, const HACK: bool> fmt::Debug for E1pString<T, CIPHER, HACK> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "E1pString(\"{}\")", self.0)
    }
}

impl<T, const CIPHER: bool, const HACK: bool> fmt::Display for E1pString<T, CIPHER, HACK> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
