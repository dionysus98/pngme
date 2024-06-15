use crate::chunk_type::ChunkType;
use crc::Crc;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chunk {
    length: [u8; 4],
    ctype: ChunkType,
    data: Vec<u8>,
    crc: [u8; 4],
}

impl TryFrom<&Vec<u8>> for Chunk {
    type Error = &'static str;

    fn try_from(value: &Vec<u8>) -> Result<Self, Self::Error> {
        let sep_at = 4;
        let (_, value) = value.split_at(sep_at);
        let (ctype, value) = value.split_at(sep_at);
        let next_split = value.len() - sep_at;
        let (data, crc) = value.split_at(next_split);

        let new = Self::new(ChunkType::new(ctype), data.into());

        if new.crc == crc {
            Ok(new)
        } else {
            Err("Invalid Crc provided")
        }
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\x1b[93m- length {}\n- crc    {}\n- type   {}\n- data   {}\x1b[0m",
            self.length(),
            self.crc(),
            self.chunk_type().to_string(),
            self.data_as_string().unwrap_or("".to_owned())
        )
    }
}

#[allow(unused)]
impl Chunk {
    pub fn to_u32(v: [u8; 4]) -> u32 {
        u32::from_be_bytes(v)
    }

    pub fn to_bytes(v: usize) -> [u8; 4] {
        (v as u32).to_be_bytes()
    }

    pub fn length(&self) -> u32 {
        Self::to_u32(self.length)
    }

    pub fn crc(&self) -> u32 {
        Self::to_u32(self.crc)
    }

    pub fn chunk_type(&self) -> ChunkType {
        self.ctype
    }

    pub fn type_as_string(&self) -> Option<String> {
        String::from_utf8(self.chunk_type().0.to_vec()).ok()
    }

    pub fn data_as_string(&self) -> Option<String> {
        String::from_utf8(self.data.clone()).ok()
    }

    pub fn as_bytes(self) -> Vec<u8> {
        let l = self.length.clone().to_vec();
        let ct = self.chunk_type().bytes().clone().to_vec();
        let d = self.data.clone();
        let crc = self.crc.clone().to_vec();

        [l, ct, d, crc].concat()
    }

    pub fn new(ctype: ChunkType, data: Vec<u8>) -> Self {
        let length = Self::to_bytes(data.len());
        let type_with_data = [ctype.0.as_slice(), data.clone().as_slice()].concat();
        let crc = Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
        let crc = Crc::<u32>::checksum(&crc, &type_with_data);
        let crc = Self::to_bytes(crc as usize);
        Self {
            data,
            ctype,
            length,
            crc,
        }
    }
}

#[allow(unused)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();
        println!("{:?}", chunk);
        chunk
    }

    #[test]
    fn base() {
        // testing_chunk();
        assert!(true);
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();
        let _chunk_string = format!("{}", chunk);
    }
}
