#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let ints = [
            2134123213213u64,
            2313,
            3,
            3213,
            21321,
            3213,
            213,
            213,
            5435,
            5654,
            6,
            5437,
            567,
            3465241345,
            677,
            90,
            98765,
            4,
            324567897654321,
            3456,
            7754,
            32,
            4567,
            432,
            56789654321,
            4,
            5678906543,
            256,
            7895432,
            56789654,
            3256,
            78543,
        ];

        for i in ints {
            let c = compress(i);
            let (d, rest) = decompress(&c);

            assert_eq!(d, i);
            assert_eq!(rest, Vec::new());
        }

        // now test if rest is parsed correctly

        for i in ints {
            let mut c = compress(i);
            c.push(1);
            c.push(2);
            c.push(3);
            c.push(4);
            let (d, rest) = decompress(&c);

            assert_eq!(d, i);
            assert_eq!(rest, vec![1, 2, 3, 4]);
        }
    }
}

pub fn compress(mut val: u64) -> Vec<u8> {
    let mut v = Vec::new();

    while val > 0 {
        let mut byte = (val & 0b111_1111) as u8;
        val = val >> 7;

        if val > 0 {
            byte |= 0b1000_0000;
        }

        v.push(byte);
    }

    v.shrink_to_fit();
    v
}

pub fn decompress(data: &[u8]) -> (u64, &[u8]) {
    let mut val = 0u64;

    for i in 0..data.len() {
        let byte = data[i];
        let byte_index = i * 7;

        {
            // cut of leading byte, if present
            let byte = byte & 0b0111_1111
            val |= byte << byte_index
        }
    }
}
