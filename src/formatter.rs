use serde::Serialize;
use serde_json::ser::CompactFormatter;
use std::{any::TypeId, io};

/// Serializes a given Yy file.
#[cfg(target_os = "macos")]
pub fn serialize_file<T: Serialize + 'static>(value: &T) -> String {
    ser(value)
}

/// Serializes a given Yy file.
#[cfg(target_os = "windows")]
pub fn serialize_file<T: Serialize + 'static>(value: &T) -> String {
    ser(value).replace('\n', "\r\n")
}

fn ser<T: Serialize + 'static>(value: &T) -> String {
    let mut writer = Vec::with_capacity(128);
    if TypeId::of::<T>() == TypeId::of::<crate::Sprite>() {
        let mut ser =
            serde_json::ser::Serializer::with_formatter(&mut writer, crate::SpriteFormatter::new());
        value.serialize(&mut ser).unwrap();
    } else {
        let formatter = Formatter {
            real_indentation_count: 0,
            has_value: false,
            array_depth: 0,
        };

        let mut ser = serde_json::ser::Serializer::with_formatter(&mut writer, formatter);
        value.serialize(&mut ser).unwrap();
    };
    unsafe {
        // We do not emit invalid UTF-8.
        String::from_utf8_unchecked(writer)
    }
}

#[derive(Debug)]
pub(crate) struct Formatter {
    pub real_indentation_count: usize,
    pub has_value: bool,
    pub array_depth: usize,
}

impl Formatter {
    pub fn indent<W>(&self, wr: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        for _ in 0..self.real_indentation_count {
            wr.write_all(b" ")?;
        }

        Ok(())
    }

    pub fn use_compact(&self) -> bool {
        self.array_depth > 0
    }
}

impl serde_json::ser::Formatter for Formatter {
    #[inline]
    fn begin_array<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.real_indentation_count += 2;
        self.array_depth += 1;
        self.has_value = false;
        writer.write_all(b"[")
    }

    #[inline]
    fn end_array<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.real_indentation_count -= 2;
        self.array_depth -= 1;

        if self.has_value {
            writer.write_all(b",\n")?;
            self.indent(writer)?;
        }

        writer.write_all(b"]")
    }

    #[inline]
    fn begin_array_value<W>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        if first {
            writer.write_all(b"\n")?;
        } else {
            writer.write_all(b",\n")?;
        }
        self.indent(writer)?;
        Ok(())
    }

    #[inline]
    fn end_array_value<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.has_value = true;

        Ok(())
    }

    #[inline]
    fn begin_object<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.has_value = false;
        if self.use_compact() {
            CompactFormatter.begin_object(writer)?;

            return Ok(());
        }

        self.real_indentation_count += 2;
        writer.write_all(b"{")
    }

    #[inline]
    fn end_object<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        if self.use_compact() {
            writer.write_all(b",")?;
            CompactFormatter.end_object(writer)?;

            return Ok(());
        }

        self.real_indentation_count -= 2;

        if self.has_value {
            writer.write_all(b",\n")?;
            self.indent(writer)?;
        }

        writer.write_all(b"}")?;
        Ok(())
    }

    #[inline]
    fn begin_object_key<W>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        if self.use_compact() {
            CompactFormatter.begin_object_key(writer, first)?;

            return Ok(());
        }

        if first {
            writer.write_all(b"\n")?;
        } else {
            writer.write_all(b",\n")?;
        }
        self.indent(writer)?;

        Ok(())
    }

    fn end_object_key<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        Ok(())
    }

    #[inline]
    fn begin_object_value<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        if self.use_compact() {
            CompactFormatter.begin_object_value(writer)?;

            return Ok(());
        }
        writer.write_all(b": ")
    }

    #[inline]
    fn end_object_value<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.has_value = true;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn yyp_serialization() {
        let x = include_str!("./../data/formatting/yyp.yyp");
        let json: crate::Yyp =
            serde_json::from_str(&crate::TrailingCommaUtility::clear_trailing_comma_once(x))
                .unwrap();

        let o = serialize_file(&json);

        assert_eq!(x, o);
    }

    #[test]
    fn object_serialization() {
        let x = include_str!("./../data/formatting/game.yy");
        let json: crate::Object =
            serde_json::from_str(&crate::TrailingCommaUtility::clear_trailing_comma_once(x))
                .unwrap();

        let o = serialize_file(&json);

        assert_eq!(x, o);
    }

    #[test]
    fn object_with_properties() {
        let x = include_str!("./../data/formatting/obj_sound_emitter.yy");
        let json: crate::Object =
            serde_json::from_str(&crate::TrailingCommaUtility::clear_trailing_comma_once(x))
                .unwrap();

        let o = serialize_file(&json);

        println!("{}", o);

        assert_eq!(x, o);
    }

    #[test]
    fn object_with_list() {
        let x = include_str!("./../data/formatting/par_modifiable_building.yy");
        let json: crate::Object =
            serde_json::from_str(&crate::TrailingCommaUtility::clear_trailing_comma_once(x))
                .unwrap();

        let o = serialize_file(&json);

        println!("us:");
        println!("{}", o);
        println!("-----");
        println!("them:");
        println!("{}", x);

        assert_eq!(x, o);
    }

    #[test]
    fn script() {
        let x = include_str!("./../data/formatting/Anchor.yy");
        let json: crate::Script =
            serde_json::from_str(&crate::TrailingCommaUtility::clear_trailing_comma_once(x))
                .unwrap();

        let o = serialize_file(&json);

        assert_eq!(x, o);
    }
}
