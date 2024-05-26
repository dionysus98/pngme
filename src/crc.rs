#[derive(Debug, Copy, Clone)]
//  crc32idohdlc {
//     width: 32,
//     poly: 0x04c11db7,
//     init: 0xffffffff,
//     refin: true,
//     refout: true,
//     xorout: 0xffffffff,
//     check: 0xcbf43926,
//     residue: 0xdebb20e3,
// };
pub struct CRC([u8; 256]);

fn make_crc_table(mut crc: CRC) -> CRC {
    for n in 0..crc.0.len() {
        let mut c = n as u32;

        for _ in 0..8 {
            if (c & 1) == 1 {
                c = 0xedb88320 ^ (c >> 1);
            } else {
                c >>= 1;
            }
        }
        crc.0[n] = c as u8;
    }
    crc
}

fn update_crc(table: CRC, crca: u32, buf: &[u8], len: u8) -> u32 {
    let mut c = crca;
    // println!("{crca}");
    for n in 0..len {
        let ch = buf[n as usize] as u32;
        let pos = ((c ^ ch) & 0xff) as usize;
        let tbl = table.0[pos] as u32;
        c = tbl ^ (c >> 8);
        // println!("{c}");
    }
    c
}

pub fn crc(buf: &[u8], length: u8) -> Result<u32, ()> {
    let table = make_crc_table(CRC([0; 256]));
    let crca = 0xffffffff;
    let crc = update_crc(table, crca, buf, length) ^ crca;
    Ok(crc)
}
