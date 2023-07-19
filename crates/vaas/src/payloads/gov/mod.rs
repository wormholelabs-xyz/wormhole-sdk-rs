use hex_literal::hex;

use alloy_primitives::FixedBytes;

use crate::{Payload, Readable, Writeable};

pub const GOVERNANCE_CHAIN: u16 = 1;
pub const GOVERNANCE_EMITTER: FixedBytes<32> = FixedBytes(hex!(
    "0000000000000000000000000000000000000000000000000000000000000004"
));

pub const GOVERNANCE_MODULE: FixedBytes<32> = FixedBytes(hex!(
    "00000000000000000000000000000000000000000000000000000000436f7265"
));

impl Payload for GovernanceMessage {}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GovernanceHeader {
    pub module: FixedBytes<32>,
    pub action: u8,
    pub target: u16,
}

impl Readable for GovernanceHeader {
    fn read<R>(reader: &mut R) -> std::io::Result<Self>
    where
        R: std::io::Read,
    {
        Ok(Self {
            module: FixedBytes::read(reader)?,
            action: u8::read(reader)?,
            target: u16::read(reader)?,
        })
    }
}

impl Writeable for GovernanceHeader {
    fn write<W>(&self, writer: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        self.module.write(writer)?;
        self.action.write(writer)?;
        self.target.write(writer)?;
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GovernanceMessage {
    pub header: GovernanceHeader,
    pub decree: Vec<u8>,
}

impl Readable for GovernanceMessage {
    fn read<R>(reader: &mut R) -> std::io::Result<Self>
    where
        R: std::io::Read,
    {
        Ok(Self {
            header: GovernanceHeader::read(reader)?,
            decree: {
                let mut buf = Vec::new();
                reader.read_to_end(&mut buf)?;
                buf
            },
        })
    }
}

impl Writeable for GovernanceMessage {
    fn write<W>(&self, writer: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        self.header.write(writer)?;
        writer.write_all(&self.decree)?;
        Ok(())
    }
}

impl GovernanceMessage {
    pub fn read_decree<R: Readable>(&self) -> Option<R> {
        R::read(&mut self.decree.as_slice()).ok()
    }
}
