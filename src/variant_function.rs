use super::Variant;
use byteorder::WriteBytesExt;
use std::io::Cursor;

#[derive(Debug)]
pub struct VariantFunction {
    name: Option<Variant>,
    args: Vec<Variant>
}

impl VariantFunction {
    pub fn new<S: AsRef<str>>(name: S) -> VariantFunction {
        VariantFunction {
            name: Some(Variant::from(name.as_ref())),
            args: Vec::new()
        }
    }

    pub fn new_none() -> VariantFunction {
        VariantFunction {
            name: None,
            args: Vec::new()
        }
    }

    pub fn indices(&self) -> usize {
        let mut i = 0;
        if self.name.is_some() {
            i = 1;
        }

        self.args.len() + i
    }

    pub fn push_arg<V>(mut self, v: V) -> VariantFunction
        where V : Into<Variant>
    {
        self.args.push(v.into());

        self
    }

    pub fn push_arg_borrow<V>(&mut self, v: V)
        where V : Into<Variant>
    {
        self.args.push(v.into());
    }

    pub fn push_arg_v(mut self, v: Variant) -> VariantFunction {
        self.args.push(v);

        self
    }

    pub fn push_arg_v_borrow(&mut self, v: Variant) {
        self.args.push(v);
    }

    pub fn pack<W>(&self, w: &mut W) -> std::io::Result<()>
        where W :
            std::io::Write + std::io::Seek
    {
        let mut i = 0;

        w.write_u8(self.indices() as u8)?;

        if self.name.is_some() {
            w.write_u8(i)?;
            self.name.as_ref().unwrap().pack(w)?;
            i += 1;
        }

        for arg in &self.args {
            w.write_u8(i)?;
            arg.pack(w)?;

            i += 1;
        };

        Ok(())
    }

    pub fn to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut cursor = Cursor::new(Vec::<u8>::new());

        self.pack(&mut cursor)?;

        Ok(cursor.into_inner())
    }
}
