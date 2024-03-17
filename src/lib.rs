use std::{collections::HashMap, fmt};

#[derive(Debug, Clone)]
pub struct OutOfRangeError;

impl fmt::Display for OutOfRangeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Index out of vocab range")
    }
}

pub fn tokenize_decode(input_id: u8) -> Result<String, OutOfRangeError> {
    let char_list = String::from("aAàÀảẢãÃáÁạẠăĂằẰẳẲẵẴắẮặẶâÂầẦẩẨẫẪấẤậẬbBcCdDđĐeEèÈẻẺẽẼéÉẹẸêÊềỀểỂễỄếẾệỆfFgGhHiIìÌỉỈĩĨíÍịỊjJkKlLmMnNoOòÒỏỎõÕóÓọỌôÔồỒổỔỗỖốỐộỘơƠờỜởỞỡỠớỚợỢpPqQrRsStTuUùÙủỦũŨúÚụỤưƯừỪửỬữỮứỨựỰvVwWxXyYỳỲỷỶỹỸýÝỵỴzZ0123456789");
    let mut vocab = HashMap::new();
    vocab.insert(0, String::from("<s>"));
    vocab.insert(1, String::from("</s>"));
    vocab.insert(2, String::from(" "));
    for (i, c) in char_list.chars().enumerate() {
        let mut tmp = [0u8; 4];
        vocab.insert(i as u8 + 3, c.encode_utf8(&mut tmp).to_owned());
    }
    if let Some(s) = vocab.get(&input_id) {
        return Ok(s.to_owned());
    } else {
        return Err(OutOfRangeError);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bdt_01() {
        let result = tokenize_decode(0);
        assert_eq!(result.unwrap(), String::from("<s>"));
    }
    #[test]
    fn bdt_02() {
        let result = tokenize_decode(198);
        assert_eq!(result.unwrap(), String::from("9"));
    }
    #[test]
    fn bdt_03() {
        let result = tokenize_decode(1);
        assert_eq!(result.unwrap(), String::from("</s>"));
    }
    #[test]
    fn bdt_04() {
        let result = tokenize_decode(197);
        assert_eq!(result.unwrap(), String::from("8"));
    }
    #[test]
    fn bdt_05() {
        let result = tokenize_decode(100);
        assert_eq!(result.unwrap(), String::from("O"));
    }
    #[test]
    #[should_panic]
    fn dt_01() {
        let result = tokenize_decode((-10 as i8).try_into().unwrap());
        assert!(result.is_err());
    }
    #[test]
    fn dt_02() {
        let result = tokenize_decode(50);
        assert_eq!(result.unwrap(), String::from("È"));
    }
    #[test]
    fn dt_03() {
        let result = tokenize_decode(195);
        assert_eq!(result.unwrap(), String::from("6"));
    }
    #[test]
    fn dt_04() {
        let result = tokenize_decode(200);
        assert!(result.is_err());
    }

}
