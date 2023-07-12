use once_cell::sync::Lazy;

#[derive(Debug, PartialEq)]
pub enum ASCIIError {
    InvalidChar,
}

static ASCII_LIST: Lazy<[char; 128]> = Lazy::new(|| {
    let mut arr = ['\0'; 128];
    for i in 0..128 {
        arr[i] = i as u8 as char;
    }
    arr
});

pub fn ascii_code_list(start_char: char, end_char: char) -> Result<Vec<u8>, ASCIIError> {
    // let arr: Vec<char> = (0..128).map(|i| i as u8 as char).collect();
    let mut result = Vec::new();
    if let (Some(start_index), Some(end_index)) = (
        ASCII_LIST.iter().position(|&c| c == start_char),
        ASCII_LIST.iter().position(|&c| c == end_char),
    ) {
        let mut current_index = start_index;
        while current_index != end_index {
            result.push(ASCII_LIST[current_index] as u8);
            current_index += 1;
            if current_index == ASCII_LIST.len() {
                current_index = 0;
            }
        }
        result.push(ASCII_LIST[end_index] as u8);
        Ok(result)
    } else {
        Err(ASCIIError::InvalidChar)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_chars_Ok() {
        assert_eq!(
            ascii_code_list('a', 'Z'),
            Ok(vec![
                97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113,
                114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 0, 1, 2, 3,
                4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
                26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46,
                47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67,
                68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88,
                89, 90
            ])
        );
        assert_eq!(
            ascii_code_list('A', 'a'),
            Ok(vec![
                65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85,
                86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97
            ])
        );
    }

    #[test]
    fn test_print_chars_Err() {
        let start_char = '\u{1F600}'; // 😀 emoji, not in the ASCII_LIST
        let end_char = 'C';

        let result = ascii_code_list(start_char, end_char);

        assert_eq!(result, Err(ASCIIError::InvalidChar));
    }
}
