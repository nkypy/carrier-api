use crate::china_telecom::ChinaTelecomClient;

impl ChinaTelecomClient {
    // hash 部分
    pub fn hash(&self, mut texts: Vec<&str>) -> String {
        texts.sort_by(|a, b| a.cmp(&b));
        let text_str = texts.join(",");
        let length = text_str.len();
        let keys = (
            self.hash_str_to_bytes(&self.license[..3]),
            self.hash_str_to_bytes(&self.license[3..6]),
            self.hash_str_to_bytes(&self.license[6..9]),
        );
        let mut iterator = 1usize;
        let mut remainder = 0usize;
        if length >= 4 {
            iterator = length / 4;
            remainder = length % 4;
        }
        let mut enc_data: Vec<String> = vec![];
        for i in 0..iterator {
            let tmp_bytes = self.hash_str_to_bytes(&text_str[i * 4..i * 4 + 4]);
            let enc_bytes = self.hash_enc(
                self.hash_enc(self.hash_enc(tmp_bytes, keys.0), keys.1),
                keys.2,
            );
            enc_data.push(self.hash_bt64_to_hex(enc_bytes));
        }
        if remainder > 0 {
            let tmp_bytes = self.hash_str_to_bytes(&text_str[iterator * 4..length]);
            let enc_bytes = self.hash_enc(
                self.hash_enc(self.hash_enc(tmp_bytes, keys.0), keys.1),
                keys.2,
            );
            enc_data.push(self.hash_bt64_to_hex(enc_bytes));
        }
        enc_data.join("")
    }
    fn hash_bt64_to_hex(&self, data: [u8; 64]) -> String {
        let mut out: Vec<String> = vec![];
        for i in 0..16 {
            let mut bt: Vec<String> = vec![];
            for j in 0..4 {
                let d = format!("{}", data[i * 4 + j]);
                bt.push(d);
            }
            let bin_idx = bt.join("");
            let bin_int = isize::from_str_radix(&bin_idx, 2).unwrap();
            out.push(format!("{:X}", bin_int));
        }
        out.join("").to_uppercase()
    }
    fn hash_str_to_bytes(&self, text: &str) -> [u8; 64] {
        let length = text.len();
        let mut bt = [0u8; 64];
        for i in 0..length {
            let k = text.as_bytes()[i] as u32;
            for j in 0..16usize {
                // 求幂
                let m = (15usize - j) as u32;
                let pow = 2u32.pow(m);
                bt[16 * i + j] = ((k / pow) % 2) as u8;
            }
        }
        bt
    }
    fn hash_gen_keys(&self, key_in: [u8; 64]) -> [[u8; 48]; 16] {
        let mut key_tmp = [0u8; 56];
        let mut keys = [[0u8; 48]; 16];
        let loop_data: [u8; 16] = [1, 1, 2, 2, 2, 2, 2, 2, 1, 2, 2, 2, 2, 2, 2, 1];
        for i in 0..7 {
            for j in 0..8 {
                key_tmp[8 * i + j] = key_in[8 * (7 - j) + i];
            }
        }
        for i in 0..16 {
            for j in 0..loop_data[i] {
                let tmp_left = key_tmp[0];
                let tmp_right = key_tmp[28];
                for k in 0..27 {
                    key_tmp[k] = key_tmp[k + 1];
                    key_tmp[28 + k] = key_tmp[29 + k];
                }
                key_tmp[27] = tmp_left;
                key_tmp[55] = tmp_right;
            }
            let tmp_keys: [u8; 48] = [
                key_tmp[13],
                key_tmp[16],
                key_tmp[10],
                key_tmp[23],
                key_tmp[0],
                key_tmp[4],
                key_tmp[2],
                key_tmp[27],
                key_tmp[14],
                key_tmp[5],
                key_tmp[20],
                key_tmp[9],
                key_tmp[22],
                key_tmp[18],
                key_tmp[11],
                key_tmp[3],
                key_tmp[25],
                key_tmp[7],
                key_tmp[15],
                key_tmp[6],
                key_tmp[26],
                key_tmp[19],
                key_tmp[12],
                key_tmp[1],
                key_tmp[40],
                key_tmp[51],
                key_tmp[30],
                key_tmp[36],
                key_tmp[46],
                key_tmp[54],
                key_tmp[29],
                key_tmp[39],
                key_tmp[50],
                key_tmp[44],
                key_tmp[32],
                key_tmp[47],
                key_tmp[43],
                key_tmp[48],
                key_tmp[38],
                key_tmp[55],
                key_tmp[33],
                key_tmp[52],
                key_tmp[45],
                key_tmp[41],
                key_tmp[49],
                key_tmp[35],
                key_tmp[28],
                key_tmp[31],
            ];
            keys[i].copy_from_slice(&tmp_keys);
        }
        keys
    }
    fn hash_init_permute(&self, data: [u8; 64]) -> [u8; 64] {
        let mut init_bytes = [0u8; 64];
        for i in 0..4 {
            for j in 0..8 {
                init_bytes[i * 8 + j] = data[(7 - j) * 8 + 2 * i + 1];
                init_bytes[i * 8 + j + 32] = data[(7 - j) * 8 + 2 * i];
            }
        }
        init_bytes
    }
    fn hash_expand_permute(&self, data: [u8; 32]) -> [u8; 48] {
        let mut ep_bytes = [0u8; 48];
        for i in 0..8 {
            if i == 0 {
                ep_bytes[0] = data[31];
            } else {
                ep_bytes[i * 6] = data[i * 4 - 1];
            }
            for n in 0..4 {
                ep_bytes[i * 6 + n + 1] = data[i * 4 + n];
            }
            if i == 7 {
                ep_bytes[47] = data[0];
            } else {
                ep_bytes[i * 6 + 5] = data[i * 4 + 4];
            }
        }
        ep_bytes
    }
    fn hash_xor_permute(&self, data: [u8; 48], key: [u8; 48]) -> [u8; 48] {
        let mut xor_bytes = [0u8; 48];
        for i in 0..48 {
            xor_bytes[i] = data[i] ^ key[i];
        }
        xor_bytes
    }
    fn hash_sbox_permute(&self, data: [u8; 48]) -> [u8; 32] {
        let mut sbox_bytes = [0u8; 32];
        let s_list: [[[u8; 16]; 4]; 8] = [
            [
                [14, 4, 13, 1, 2, 15, 11, 8, 3, 10, 6, 12, 5, 9, 0, 7],
                [0, 15, 7, 4, 14, 2, 13, 1, 10, 6, 12, 11, 9, 5, 3, 8],
                [4, 1, 14, 8, 13, 6, 2, 11, 15, 12, 9, 7, 3, 10, 5, 0],
                [15, 12, 8, 2, 4, 9, 1, 7, 5, 11, 3, 14, 10, 0, 6, 13],
            ],
            [
                [15, 1, 8, 14, 6, 11, 3, 4, 9, 7, 2, 13, 12, 0, 5, 10],
                [3, 13, 4, 7, 15, 2, 8, 14, 12, 0, 1, 10, 6, 9, 11, 5],
                [0, 14, 7, 11, 10, 4, 13, 1, 5, 8, 12, 6, 9, 3, 2, 15],
                [13, 8, 10, 1, 3, 15, 4, 2, 11, 6, 7, 12, 0, 5, 14, 9],
            ],
            [
                [10, 0, 9, 14, 6, 3, 15, 5, 1, 13, 12, 7, 11, 4, 2, 8],
                [13, 7, 0, 9, 3, 4, 6, 10, 2, 8, 5, 14, 12, 11, 15, 1],
                [13, 6, 4, 9, 8, 15, 3, 0, 11, 1, 2, 12, 5, 10, 14, 7],
                [1, 10, 13, 0, 6, 9, 8, 7, 4, 15, 14, 3, 11, 5, 2, 12],
            ],
            [
                [7, 13, 14, 3, 0, 6, 9, 10, 1, 2, 8, 5, 11, 12, 4, 15],
                [13, 8, 11, 5, 6, 15, 0, 3, 4, 7, 2, 12, 1, 10, 14, 9],
                [10, 6, 9, 0, 12, 11, 7, 13, 15, 1, 3, 14, 5, 2, 8, 4],
                [3, 15, 0, 6, 10, 1, 13, 8, 9, 4, 5, 11, 12, 7, 2, 14],
            ],
            [
                [2, 12, 4, 1, 7, 10, 11, 6, 8, 5, 3, 15, 13, 0, 14, 9],
                [14, 11, 2, 12, 4, 7, 13, 1, 5, 0, 15, 10, 3, 9, 8, 6],
                [4, 2, 1, 11, 10, 13, 7, 8, 15, 9, 12, 5, 6, 3, 0, 14],
                [11, 8, 12, 7, 1, 14, 2, 13, 6, 15, 0, 9, 10, 4, 5, 3],
            ],
            [
                [12, 1, 10, 15, 9, 2, 6, 8, 0, 13, 3, 4, 14, 7, 5, 11],
                [10, 15, 4, 2, 7, 12, 9, 5, 6, 1, 13, 14, 0, 11, 3, 8],
                [9, 14, 15, 5, 2, 8, 12, 3, 7, 0, 4, 10, 1, 13, 11, 6],
                [4, 3, 2, 12, 9, 5, 15, 10, 11, 14, 1, 7, 6, 0, 8, 13],
            ],
            [
                [4, 11, 2, 14, 15, 0, 8, 13, 3, 12, 9, 7, 5, 10, 6, 1],
                [13, 0, 11, 7, 4, 9, 1, 10, 14, 3, 5, 12, 2, 15, 8, 6],
                [1, 4, 11, 13, 12, 3, 7, 14, 10, 15, 6, 8, 0, 5, 9, 2],
                [6, 11, 13, 8, 1, 4, 10, 7, 9, 5, 0, 15, 14, 2, 3, 12],
            ],
            [
                [13, 2, 8, 4, 6, 15, 11, 1, 10, 9, 3, 14, 5, 0, 12, 7],
                [1, 15, 13, 8, 10, 3, 7, 4, 12, 5, 6, 11, 0, 14, 9, 2],
                [7, 11, 4, 1, 9, 12, 14, 2, 0, 6, 10, 13, 15, 3, 5, 8],
                [2, 1, 14, 7, 4, 10, 8, 13, 15, 12, 9, 0, 3, 5, 6, 11],
            ],
        ];
        for m in 0..8usize {
            let i = (data[m * 6 + 0] * 2 + data[m * 6 + 5]) as usize;
            let j = (data[m * 6 + 1] * 2 * 2 * 2
                + data[m * 6 + 2] * 2 * 2
                + data[m * 6 + 3] * 2
                + data[m * 6 + 4]) as usize;
            let bin: u8 = s_list[m][i][j];
            let bin_str = format!("{:04b}", bin);
            let bin_vec: Vec<char> = bin_str.chars().collect();
            for n in 0..4 {
                if bin_vec[n] == '1' {
                    sbox_bytes[m * 4 + n] = 1;
                } else {
                    sbox_bytes[m * 4 + n] = 0;
                }
            }
        }
        sbox_bytes
    }
    fn hash_p_permute(&self, data: [u8; 32]) -> [u8; 48] {
        [
            data[15], data[6], data[19], data[20], data[28], data[11], data[27], data[16], data[0],
            data[14], data[22], data[25], data[4], data[17], data[30], data[9], data[1], data[7],
            data[23], data[13], data[31], data[26], data[2], data[8], data[18], data[12], data[29],
            data[5], data[21], data[10], data[3], data[24], 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ]
    }
    fn hash_final_permute(&self, data: [u8; 64]) -> [u8; 64] {
        [
            data[39], data[7], data[47], data[15], data[55], data[23], data[63], data[31],
            data[38], data[6], data[46], data[14], data[54], data[22], data[62], data[30],
            data[37], data[5], data[45], data[13], data[53], data[21], data[61], data[29],
            data[36], data[4], data[44], data[12], data[52], data[20], data[60], data[28],
            data[35], data[3], data[43], data[11], data[51], data[19], data[59], data[27],
            data[34], data[2], data[42], data[10], data[50], data[18], data[58], data[26],
            data[33], data[1], data[41], data[9], data[49], data[17], data[57], data[25], data[32],
            data[0], data[40], data[8], data[48], data[16], data[56], data[24],
        ]
    }
    fn hash_enc(&self, data: [u8; 64], key: [u8; 64]) -> [u8; 64] {
        let keys = self.hash_gen_keys(key);
        let ip_bytes = self.hash_init_permute(data);
        let mut ip_left = [0u8; 32];
        let mut ip_right = [0u8; 32];
        let mut tmp_left = [0u8; 48];
        ip_left.copy_from_slice(&ip_bytes[..32]);
        ip_right.copy_from_slice(&ip_bytes[32..64]);
        for i in 0..16 {
            tmp_left[..32].copy_from_slice(&ip_left);
            ip_left.copy_from_slice(&ip_right);
            let mut key = [0u8; 48];
            key.copy_from_slice(&keys[i]);
            let tmp_right = self.hash_xor_permute(
                self.hash_p_permute(self.hash_sbox_permute(
                    self.hash_xor_permute(self.hash_expand_permute(ip_right), key),
                )),
                tmp_left,
            );
            ip_right.copy_from_slice(&tmp_right[..32]);
        }
        let mut final_bytes = [0u8; 64];
        final_bytes[..32].copy_from_slice(&ip_right);
        final_bytes[32..64].copy_from_slice(&ip_left[..32]);
        self.hash_final_permute(final_bytes)
    }
}

#[cfg(test)]
mod tests {
    use crate::china_telecom::ChinaTelecomClient;

    #[test]
    fn test_china_telecom_client_hash() {
        let client = ChinaTelecomClient::new("test", "test", "abcdefghi");
        assert_eq!(client.hash(vec!["test"]), "41894168BD86A2CC".to_string());
        assert_eq!(
            client.hash(vec!["14914000000", "test", "test", "queryPakage"]),
            "45E8B9924DE397A8F7E5764767810CF774CC7E1685BA702C9C4C367EFDAE5D932B37C0C8F0F8EB0CAD6372289F407CA941894168BD86A2CC32E5804EA05BAA5099649468B9418E52".to_string(),
        );
    }
}
