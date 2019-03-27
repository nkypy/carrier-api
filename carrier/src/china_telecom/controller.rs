use crate::china_telecom::ChinaTelecomClient;

impl ChinaTelecomClient {
    pub fn hash(&self, mut texts: Vec<&'static str>) -> String {
        texts.sort_by(|a, b| a.cmp(&b));
        let text_str = texts.join(",");
        let length = text_str.len();
        let keys = (
            self.hash_str_to_bytes(&self.license[..3]),
            self.hash_str_to_bytes(&self.license[3..6]),
            self.hash_str_to_bytes(&self.license[6..9]),
        );
        let mut iterator = 0usize;
        let mut remainder = 0usize;
        if length >= 4 {
            iterator = length / 4;
            remainder = length % 4;
        } else {
            iterator = 1;
            remainder = 0;
        }
        let mut enc_data: Vec<String> = vec![];
        for i in 0..iterator {
            let tmp_str = &text_str[i * 4..i * 4 + 4];
            let tmp_bytes = self.hash_str_to_bytes(tmp_str);
            let mut tmp_bt = tmp_bytes;
            tmp_bt = self.hash_enc(tmp_bt, keys.0);
            tmp_bt = self.hash_enc(tmp_bt, keys.1);
            tmp_bt = self.hash_enc(tmp_bt, keys.2);
            let enc_bytes = tmp_bt;
            enc_data.push(self.hash_bt64_to_hex(enc_bytes));
        }
        if remainder > 0 {
            let remain_data = &text_str[iterator*4..length];
            let tmp_bytes = self.hash_str_to_bytes(remain_data);
            let mut tmp_bt = tmp_bytes;
            tmp_bt = self.hash_enc(tmp_bt, keys.0);
            tmp_bt = self.hash_enc(tmp_bt, keys.1);
            tmp_bt = self.hash_enc(tmp_bt, keys.2);
            let enc_bytes = tmp_bt;
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
        let loopData: [u8; 16] = [1, 1, 2, 2, 2, 2, 2, 2, 1, 2, 2, 2, 2, 2, 2, 1];
        for i in 0..7usize {
            let mut k = 8;
            for j in 0..8 {
                k = k - 1;
                key_tmp[i * 8 + j] = key_in[8 * k + i];
            }
        }
        for i in 0..16 {
            let mut tmp_left = 0u8;
            let mut tmp_right = 0u8;
            for j in 0..loopData[i] {
                tmp_left = key_tmp[0];
                tmp_right = key_tmp[28];
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
            for n in 0..48 {
                keys[i][n] = tmp_keys[n]
            }
        }
        keys
    }
    fn hash_init_permute(&self, data: [u8; 64]) -> [u8; 64] {
        let mut ip_byte = [0u8; 64];
        let mut m = 1;
        let mut n = 0;
        for i in 0..4 {
            let mut k = 0;
            for j in 0..8 {
                let tmp_j = 7 - j;
                ip_byte[i * 8 + k] = data[tmp_j * 8 + m];
                ip_byte[i * 8 + k + 32] = data[tmp_j * 8 + n];
                k = k + 1;
            }
            m = m + 2;
            n = n + 2;
        }
        ip_byte
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
    fn hash_xor(&self, data: [u8; 48], key: [u8; 48]) -> [u8; 64] {
        let mut xor_bytes = [0u8; 64];
        for i in 0..48 {
            xor_bytes[i] = data[i] ^ key[i];
        }
        xor_bytes
    }
    fn hash_sbox_permute(&self, data: [u8; 64]) -> [u8; 32] {
        let mut sbox_bytes = [0u8; 32];
        let s1: [[u8; 16]; 4] = [
            [14, 4, 13, 1, 2, 15, 11, 8, 3, 10, 6, 12, 5, 9, 0, 7],
            [0, 15, 7, 4, 14, 2, 13, 1, 10, 6, 12, 11, 9, 5, 3, 8],
            [4, 1, 14, 8, 13, 6, 2, 11, 15, 12, 9, 7, 3, 10, 5, 0],
            [15, 12, 8, 2, 4, 9, 1, 7, 5, 11, 3, 14, 10, 0, 6, 13],
        ];
        let s2: [[u8; 16]; 4] = [
            [15, 1, 8, 14, 6, 11, 3, 4, 9, 7, 2, 13, 12, 0, 5, 10],
            [3, 13, 4, 7, 15, 2, 8, 14, 12, 0, 1, 10, 6, 9, 11, 5],
            [0, 14, 7, 11, 10, 4, 13, 1, 5, 8, 12, 6, 9, 3, 2, 15],
            [13, 8, 10, 1, 3, 15, 4, 2, 11, 6, 7, 12, 0, 5, 14, 9],
        ];
        let s3: [[u8; 16]; 4] = [
            [10, 0, 9, 14, 6, 3, 15, 5, 1, 13, 12, 7, 11, 4, 2, 8],
            [13, 7, 0, 9, 3, 4, 6, 10, 2, 8, 5, 14, 12, 11, 15, 1],
            [13, 6, 4, 9, 8, 15, 3, 0, 11, 1, 2, 12, 5, 10, 14, 7],
            [1, 10, 13, 0, 6, 9, 8, 7, 4, 15, 14, 3, 11, 5, 2, 12],
        ];
        let s4: [[u8; 16]; 4] = [
            [7, 13, 14, 3, 0, 6, 9, 10, 1, 2, 8, 5, 11, 12, 4, 15],
            [13, 8, 11, 5, 6, 15, 0, 3, 4, 7, 2, 12, 1, 10, 14, 9],
            [10, 6, 9, 0, 12, 11, 7, 13, 15, 1, 3, 14, 5, 2, 8, 4],
            [3, 15, 0, 6, 10, 1, 13, 8, 9, 4, 5, 11, 12, 7, 2, 14],
        ];
        let s5: [[u8; 16]; 4] = [
            [2, 12, 4, 1, 7, 10, 11, 6, 8, 5, 3, 15, 13, 0, 14, 9],
            [14, 11, 2, 12, 4, 7, 13, 1, 5, 0, 15, 10, 3, 9, 8, 6],
            [4, 2, 1, 11, 10, 13, 7, 8, 15, 9, 12, 5, 6, 3, 0, 14],
            [11, 8, 12, 7, 1, 14, 2, 13, 6, 15, 0, 9, 10, 4, 5, 3],
        ];
        let s6: [[u8; 16]; 4] = [
            [12, 1, 10, 15, 9, 2, 6, 8, 0, 13, 3, 4, 14, 7, 5, 11],
            [10, 15, 4, 2, 7, 12, 9, 5, 6, 1, 13, 14, 0, 11, 3, 8],
            [9, 14, 15, 5, 2, 8, 12, 3, 7, 0, 4, 10, 1, 13, 11, 6],
            [4, 3, 2, 12, 9, 5, 15, 10, 11, 14, 1, 7, 6, 0, 8, 13],
        ];
        let s7: [[u8; 16]; 4] = [
            [4, 11, 2, 14, 15, 0, 8, 13, 3, 12, 9, 7, 5, 10, 6, 1],
            [13, 0, 11, 7, 4, 9, 1, 10, 14, 3, 5, 12, 2, 15, 8, 6],
            [1, 4, 11, 13, 12, 3, 7, 14, 10, 15, 6, 8, 0, 5, 9, 2],
            [6, 11, 13, 8, 1, 4, 10, 7, 9, 5, 0, 15, 14, 2, 3, 12],
        ];
        let s8: [[u8; 16]; 4] = [
            [13, 2, 8, 4, 6, 15, 11, 1, 10, 9, 3, 14, 5, 0, 12, 7],
            [1, 15, 13, 8, 10, 3, 7, 4, 12, 5, 6, 11, 0, 14, 9, 2],
            [7, 11, 4, 1, 9, 12, 14, 2, 0, 6, 10, 13, 15, 3, 5, 8],
            [2, 1, 14, 7, 4, 10, 8, 13, 15, 12, 9, 0, 3, 5, 6, 11],
        ];
        let sList: [[[u8; 16]; 4]; 8] = [s1, s2, s3, s4, s5, s6, s7, s8];
        for m in 0..8usize {
            let i = (data[m * 6 + 0] * 2 + data[m * 6 + 5]) as usize;
            let j = (data[m * 6 + 1] * 2 * 2 * 2
                + data[m * 6 + 2] * 2 * 2
                + data[m * 6 + 3] * 2
                + data[m * 6 + 4]) as usize;
            let bin: u8 = sList[m][i][j];
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
        let mut p_bytes = [0u8; 48];
        p_bytes[0] = data[15];
        p_bytes[1] = data[6];
        p_bytes[2] = data[19];
        p_bytes[3] = data[20];
        p_bytes[4] = data[28];
        p_bytes[5] = data[11];
        p_bytes[6] = data[27];
        p_bytes[7] = data[16];
        p_bytes[8] = data[0];
        p_bytes[9] = data[14];
        p_bytes[10] = data[22];
        p_bytes[11] = data[25];
        p_bytes[12] = data[4];
        p_bytes[13] = data[17];
        p_bytes[14] = data[30];
        p_bytes[15] = data[9];
        p_bytes[16] = data[1];
        p_bytes[17] = data[7];
        p_bytes[18] = data[23];
        p_bytes[19] = data[13];
        p_bytes[20] = data[31];
        p_bytes[21] = data[26];
        p_bytes[22] = data[2];
        p_bytes[23] = data[8];
        p_bytes[24] = data[18];
        p_bytes[25] = data[12];
        p_bytes[26] = data[29];
        p_bytes[27] = data[5];
        p_bytes[28] = data[21];
        p_bytes[29] = data[10];
        p_bytes[30] = data[3];
        p_bytes[31] = data[24];
        p_bytes
    }
    fn hash_final_permute(&self, data: [u8; 64]) -> [u8; 64] {
        let mut final_bytes = [0u8; 64];
        final_bytes[0] = data[39];
        final_bytes[1] = data[7];
        final_bytes[2] = data[47];
        final_bytes[3] = data[15];
        final_bytes[4] = data[55];
        final_bytes[5] = data[23];
        final_bytes[6] = data[63];
        final_bytes[7] = data[31];
        final_bytes[8] = data[38];
        final_bytes[9] = data[6];
        final_bytes[10] = data[46];
        final_bytes[11] = data[14];
        final_bytes[12] = data[54];
        final_bytes[13] = data[22];
        final_bytes[14] = data[62];
        final_bytes[15] = data[30];
        final_bytes[16] = data[37];
        final_bytes[17] = data[5];
        final_bytes[18] = data[45];
        final_bytes[19] = data[13];
        final_bytes[20] = data[53];
        final_bytes[21] = data[21];
        final_bytes[22] = data[61];
        final_bytes[23] = data[29];
        final_bytes[24] = data[36];
        final_bytes[25] = data[4];
        final_bytes[26] = data[44];
        final_bytes[27] = data[12];
        final_bytes[28] = data[52];
        final_bytes[29] = data[20];
        final_bytes[30] = data[60];
        final_bytes[31] = data[28];
        final_bytes[32] = data[35];
        final_bytes[33] = data[3];
        final_bytes[34] = data[43];
        final_bytes[35] = data[11];
        final_bytes[36] = data[51];
        final_bytes[37] = data[19];
        final_bytes[38] = data[59];
        final_bytes[39] = data[27];
        final_bytes[40] = data[34];
        final_bytes[41] = data[2];
        final_bytes[42] = data[42];
        final_bytes[43] = data[10];
        final_bytes[44] = data[50];
        final_bytes[45] = data[18];
        final_bytes[46] = data[58];
        final_bytes[47] = data[26];
        final_bytes[48] = data[33];
        final_bytes[49] = data[1];
        final_bytes[50] = data[41];
        final_bytes[51] = data[9];
        final_bytes[52] = data[49];
        final_bytes[53] = data[17];
        final_bytes[54] = data[57];
        final_bytes[55] = data[25];
        final_bytes[56] = data[32];
        final_bytes[57] = data[0];
        final_bytes[58] = data[40];
        final_bytes[59] = data[8];
        final_bytes[60] = data[48];
        final_bytes[61] = data[16];
        final_bytes[62] = data[56];
        final_bytes[63] = data[24];
        final_bytes
    }
    fn hash_enc(&self, data: [u8; 64], key: [u8; 64]) -> [u8; 64] {
        let keys = self.hash_gen_keys(key);
        let ip_bytes = self.hash_init_permute(data);
        let mut ip_left = [0u8; 32];
        let mut ip_right = [0u8; 32];
        let mut tmp_left = [0u8; 48];
        for k in 0..32 {
            ip_left[k] = ip_bytes[k];
            ip_right[k] = ip_bytes[32 + k];
        }
        for i in 0..16 {
            for j in 0..32 {
                tmp_left[j] = ip_left[j];
                ip_left[j] = ip_right[j];
            }
            let mut key = [0u8; 48];
            for m in 0..48 {
                key[m] = keys[i][m];
            }
            let a1 = self.hash_expand_permute(ip_right);
            let a2 = self.hash_xor(a1, key);
            let a3 = self.hash_sbox_permute(a2);
            let a4 = self.hash_p_permute(a3);
            let tmp_right = self.hash_xor(a4, tmp_left);
            for n in 0..32 {
                ip_right[n] = tmp_right[n];
            }
        }
        let mut final_data = [0u8; 64];
        for i in 0..32 {
            final_data[i] = ip_right[i];
            final_data[32 + i] = ip_left[i];
        }
        self.hash_final_permute(final_data)
    }
}
