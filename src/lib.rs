#![forbid(unsafe_code)]
#![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, clippy::todo, clippy::unwrap_used)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! A Rust implementation of the Progress `ENCODE` function
//!
//! Progress' documentation on `ENCODE` can be found [here][psc]. This particular
//! implementation is based on the [C# implementation by pvginkel][csharp]
//!
//! [psc]: https://docs.progress.com/bundle/openedge-abl-reference-117/page/ENCODE-function.html
//! [csharp]: https://github.com/pvginkel/ProgressEncode

static LOOKUP: &[u16] = &[
    0, 49345, 49537, 320, 49921, 960, 640, 49729, 50689, 1728, 1920, 51009, 1280, 50625, 50305,
    1088, 52225, 3264, 3456, 52545, 3840, 53185, 52865, 3648, 2560, 51905, 52097, 2880, 51457,
    2496, 2176, 51265, 55297, 6336, 6528, 55617, 6912, 56257, 55937, 6720, 7680, 57025, 57217,
    8000, 56577, 7616, 7296, 56385, 5120, 54465, 54657, 5440, 55041, 6080, 5760, 54849, 53761,
    4800, 4992, 54081, 4352, 53697, 53377, 4160, 61441, 12480, 12672, 61761, 13056, 62401, 62081,
    12864, 13824, 63169, 63361, 14144, 62721, 13760, 13440, 62529, 15360, 64705, 64897, 15680,
    65281, 16320, 16000, 65089, 64001, 15040, 15232, 64321, 14592, 63937, 63617, 14400, 10240,
    59585, 59777, 10560, 60161, 11200, 10880, 59969, 60929, 11968, 12160, 61249, 11520, 60865,
    60545, 11328, 58369, 9408, 9600, 58689, 9984, 59329, 59009, 9792, 8704, 58049, 58241, 9024,
    57601, 8640, 8320, 57409, 40961, 24768, 24960, 41281, 25344, 41921, 41601, 25152, 26112, 42689,
    42881, 26432, 42241, 26048, 25728, 42049, 27648, 44225, 44417, 27968, 44801, 28608, 28288,
    44609, 43521, 27328, 27520, 43841, 26880, 43457, 43137, 26688, 30720, 47297, 47489, 31040,
    47873, 31680, 31360, 47681, 48641, 32448, 32640, 48961, 32000, 48577, 48257, 31808, 46081,
    29888, 30080, 46401, 30464, 47041, 46721, 30272, 29184, 45761, 45953, 29504, 45313, 29120,
    28800, 45121, 20480, 37057, 37249, 20800, 37633, 21440, 21120, 37441, 38401, 22208, 22400,
    38721, 21760, 38337, 38017, 21568, 39937, 23744, 23936, 40257, 24320, 40897, 40577, 24128,
    23040, 39617, 39809, 23360, 39169, 22976, 22656, 38977, 34817, 18624, 18816, 35137, 19200,
    35777, 35457, 19008, 19968, 36545, 36737, 20288, 36097, 19904, 19584, 35905, 17408, 33985,
    34177, 17728, 34561, 18368, 18048, 34369, 33281, 17088, 17280, 33601, 16640, 33217, 32897,
    16448,
];

/// Return a 16-character encoding of the `input` bytes.
///
/// ### Example usage
///
/// ```rust
/// use progress_encode::encode;
///
/// let password = String::from("my-passw0rd");
/// let encoded_password = encode(password.as_bytes());
/// assert_eq!(encoded_password, "lEsdklcFaOOjlbma");
/// ```
pub fn encode(input: &[u8]) -> String {
    let mut scratch: [u8; 16] = [0; 16];

    let mut hash: u16 = 17;

    for _ in 0..5 {
        for j in 0..input.len() {
            scratch[15 - (j % 16)] ^= input[j];
        }

        for j in (0..16).step_by(2) {
            hash = pr_hash(&scratch, hash);

            scratch[j] = (hash & 0xff) as u8;
            scratch[j + 1] = ((hash >> 8) & 0xff) as u8;
        }
    }

    let mut target = String::new();

    for byte in scratch {
        let lower = (byte & 0x7f) as char;

        if lower.is_ascii_uppercase() || lower.is_ascii_lowercase() {
            target.push(lower);
        } else {
            target.push(((byte >> 4) + 0x61) as char);
        }
    }

    target
}

#[inline]
fn pr_hash(scratch: &[u8; 16], mut hash: u16) -> u16 {
    for byte in scratch.iter().rev() {
        hash = hash >> 8 ^ LOOKUP[(hash & 0xff) as usize] ^ LOOKUP[*byte as usize];
    }
    hash
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use crate::encode;

    #[derive(Debug, Clone, Deserialize, Serialize)]
    struct TestCase {
        encoded: String,
        input: Vec<u8>,
    }

    #[test]
    fn test_encode() {
        let json = include_str!("tests.json");

        let test_cases: Vec<TestCase> = serde_json::from_str(json).expect("Deserialize tests");

        for (idx, test) in test_cases.iter().enumerate() {
            println!("Test {idx}, expecting encoded value {}", test.encoded);
            if let Ok(input_str) = String::from_utf8(test.input.clone()) {
                println!("Input string: \"{input_str}\"");
            }
            assert_eq!(test.encoded, encode(&test.input));
        }
    }

    #[test]
    fn test_example_password() {
        let password = String::from("my-passw0rd");
        let encoded_password = encode(password.as_bytes());
        assert_eq!(encoded_password, "lEsdklcFaOOjlbma");
    }
}
