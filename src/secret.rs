use std::collections::HashSet;

use json::JsonValue;
use reqwest::header::HeaderMap;

#[allow(unused)]
const SALT_TABLE_LEN: usize = 62;
#[allow(unused)]
const SALT_CODE_TABLE: [&'static str; SALT_TABLE_LEN] = ["5", "b", "f", "A", "J", "Q", "g", "a", "l", "p", "s", "q", "H", "4", "L", "Q", "g", "1", "6", "Q", "Z", "v", "w", "b", "c", "e", "2", "2", "m", "l", "E", "g", "G", "H", "I", "r", "o", "s", "d", "5", "7", "x", "t", "J", "S", "T", "F", "v", "w", "4", "8", "9", "0", "K", "E", "3", "4", "0", "m", "r", "i", "n"];

const SALT_LEN: usize = 20;
const PLAIN_SALT: [u8; SALT_LEN] = [43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 0];

const REG_SPECIAL: &'static str = r"[`~!@#$%^&*()+=|{}':;',\\[\\].<>/?~！@#￥%……&*（）——+|{}【】‘；：”“’。，、？]";

lazy_static::lazy_static! {
    /// these keys are not included in hash check
    static ref UNCHECKED_KEYS: HashSet<&'static str> = HashSet::from_iter([
        "content",
        "deviceName",
        "keyWord",
        "blogBody",
        "blogTitle",
        "getType",
        "responsibilities",
        "street",
        "text",
        "reason",
        "searchvalue",
        "key",
        "answers",
        "leaveReason",
        "personRemark",
        "selfAppraisal",
        "imgUrl",
        "wxname",
        "deviceId",
        "avatarTempPath",
        "file",
        "file",
        "model",
        "brand",
        "system",
        "deviceId",
        "platform",
        "code",
        "openId",
        "unionid",
    ].into_iter());
}

#[derive(Debug)]
pub struct TokenData {
    /// md5 hash of the plain
    hash: String,
    timestamp: String,
    plain_salt: String
}

impl TokenData {
    pub fn add_to_headers(self, headers: &mut HeaderMap) {
        headers.insert("t", self.timestamp.try_into().unwrap());
        headers.insert("m", self.hash.try_into().unwrap());
        headers.insert("s", self.plain_salt.try_into().unwrap());
        headers.insert("v", "1.7.30".try_into().unwrap());
        headers.insert("n", "n: content,deviceName,keyWord,blogBody,blogTitle,getType,responsibilities,street,text,reason,searchvalue,key,answers,leaveReason,personRemark,selfAppraisal,imgUrl,wxname,deviceId,avatarTempPath,file,file,model,brand,system,deviceId,platform,code,openId,unionid".try_into().unwrap());
    }

    /// do not use it directly, test only 
    fn with_config(req: JsonValue, salt: [u8; SALT_LEN], timestamp: String) -> Self {
        let mut entries: Vec<(&str, &JsonValue)> = req.entries().collect();
        entries.sort_by(|a, b| {
            a.0.cmp(b.0)
        });

        let encoded_salt = encode_salt(salt);

        let re = regex::Regex::new(REG_SPECIAL).unwrap();
        let mut body = entries.into_iter().filter_map(|(key, value)| {
            let value = value.to_string();
            match UNCHECKED_KEYS.contains(key) {
                false if !re.is_match(&value)=> Some(value.to_string()),
                _ => None
            }
        }).fold(String::new(), |acc, value| acc + &value);
        body += &timestamp;
        body += &encoded_salt;

        let re = regex::Regex::new(
            r"(\s+|<|>|\r+|-|&)"
        ).unwrap();
        re.replace_all(&mut body, "");

        let body = urlencoding::encode(&body);

        Self {
            hash: format!("{:x}", md5::compute(body.as_bytes())),
            timestamp,
            plain_salt: salt.map(|n| n.to_string()).join("_")
        }
    }

    pub fn new(req: JsonValue) -> Self {
        let timestamp = format!("{}", chrono::Utc::now().timestamp());
        Self::with_config(req, PLAIN_SALT, timestamp)
    }
}

#[allow(unused)]
fn parse_salt_str(string: &str) -> [u8; SALT_LEN] {
    let mut res = [0; SALT_LEN];
    string.split("_").into_iter().enumerate().for_each(|(idx, num)| {
        res[idx] = u8::from_str_radix(num, 10).unwrap()
    });
    res
}

fn encode_salt(plain: [u8; SALT_LEN]) -> String {
    plain.map(|e| SALT_CODE_TABLE[e as usize]).join("")
}

