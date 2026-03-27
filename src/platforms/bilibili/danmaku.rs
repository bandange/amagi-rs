use crate::error::AppError;

use super::types::BilibiliDanmakuElem;

/// Parse one Bilibili `DmSegMobileReply` protobuf payload.
///
/// This parser intentionally implements the small subset of protobuf needed by
/// the migrated `videoDanmaku` endpoint instead of adding a new dependency.
pub fn parse_dm_seg_mobile_reply(bytes: &[u8]) -> Result<Vec<BilibiliDanmakuElem>, AppError> {
    let mut cursor = Cursor::new(bytes);
    let mut elems = Vec::new();

    while !cursor.is_eof() {
        let tag = cursor.read_varint()?;
        let field_number = tag >> 3;
        let wire_type = (tag & 0x07) as u8;

        match (field_number, wire_type) {
            (1, 2) => {
                let payload = cursor.read_length_delimited()?;
                elems.push(parse_danmaku_elem(payload)?);
            }
            _ => cursor.skip_field(wire_type)?,
        }
    }

    Ok(elems)
}

fn parse_danmaku_elem(bytes: &[u8]) -> Result<BilibiliDanmakuElem, AppError> {
    let mut cursor = Cursor::new(bytes);
    let mut elem = BilibiliDanmakuElem::default();

    while !cursor.is_eof() {
        let tag = cursor.read_varint()?;
        let field_number = tag >> 3;
        let wire_type = (tag & 0x07) as u8;

        match (field_number, wire_type) {
            (1, 0) => elem.id = cursor.read_varint()?.to_string(),
            (2, 0) => elem.progress = cursor.read_varint()? as i32,
            (3, 0) => elem.mode = cursor.read_varint()? as i32,
            (4, 0) => elem.fontsize = cursor.read_varint()? as i32,
            (5, 0) => elem.color = cursor.read_varint()? as u32,
            (6, 2) => elem.mid_hash = cursor.read_string()?,
            (7, 2) => elem.content = cursor.read_string()?,
            (8, 0) => elem.ctime = cursor.read_varint()?.to_string(),
            (9, 0) => elem.weight = cursor.read_varint()? as i32,
            (10, 2) => elem.action = cursor.read_string()?,
            (11, 0) => elem.pool = cursor.read_varint()? as i32,
            (12, 2) => elem.id_str = cursor.read_string()?,
            (13, 0) => elem.attr = cursor.read_varint()? as i32,
            (22, 2) => elem.animation = cursor.read_string()?,
            _ => cursor.skip_field(wire_type)?,
        }
    }

    Ok(elem)
}

struct Cursor<'a> {
    bytes: &'a [u8],
    position: usize,
}

impl<'a> Cursor<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, position: 0 }
    }

    fn is_eof(&self) -> bool {
        self.position >= self.bytes.len()
    }

    fn read_varint(&mut self) -> Result<u64, AppError> {
        let mut value = 0_u64;
        let mut shift = 0_u32;

        loop {
            if self.position >= self.bytes.len() {
                return Err(AppError::InvalidRequestConfig(
                    "invalid bilibili danmaku protobuf: unexpected eof".to_owned(),
                ));
            }

            let byte = self.bytes[self.position];
            self.position += 1;

            value |= u64::from(byte & 0x7f) << shift;

            if byte & 0x80 == 0 {
                return Ok(value);
            }

            shift += 7;
            if shift >= 64 {
                return Err(AppError::InvalidRequestConfig(
                    "invalid bilibili danmaku protobuf: varint overflow".to_owned(),
                ));
            }
        }
    }

    fn read_length_delimited(&mut self) -> Result<&'a [u8], AppError> {
        let length = self.read_varint()? as usize;
        let end = self.position.saturating_add(length);

        if end > self.bytes.len() {
            return Err(AppError::InvalidRequestConfig(
                "invalid bilibili danmaku protobuf: truncated field".to_owned(),
            ));
        }

        let slice = &self.bytes[self.position..end];
        self.position = end;
        Ok(slice)
    }

    fn read_string(&mut self) -> Result<String, AppError> {
        let bytes = self.read_length_delimited()?;
        Ok(String::from_utf8_lossy(bytes).into_owned())
    }

    fn skip_field(&mut self, wire_type: u8) -> Result<(), AppError> {
        match wire_type {
            0 => {
                let _ = self.read_varint()?;
                Ok(())
            }
            1 => self.advance(8),
            2 => {
                let _ = self.read_length_delimited()?;
                Ok(())
            }
            5 => self.advance(4),
            _ => Err(AppError::InvalidRequestConfig(format!(
                "invalid bilibili danmaku protobuf: unsupported wire type `{wire_type}`"
            ))),
        }
    }

    fn advance(&mut self, count: usize) -> Result<(), AppError> {
        let end = self.position.saturating_add(count);
        if end > self.bytes.len() {
            return Err(AppError::InvalidRequestConfig(
                "invalid bilibili danmaku protobuf: truncated fixed field".to_owned(),
            ));
        }

        self.position = end;
        Ok(())
    }
}
