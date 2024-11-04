fn gray(n: u8) -> Vec<String> {
    // Базовий випадок для 0 бітів
    if n == 0 {
        return vec![String::from("")];
    }
    
    // Базовий випадок для 1 біта
    if n == 1 {
        return vec![String::from("0"), String::from("1")];
    }

    // Рекурсивне побудування Gray code
    let previous_gray = gray(n - 1);
    let mut result = Vec::new();
    
    // Додаємо "0" до кожного елемента попереднього результату
    for code in &previous_gray {
        result.push(format!("0{}", code));
    }

    // Додаємо "1" до кожного елемента попереднього результату в зворотному порядку
    for code in previous_gray.iter().rev() {
        result.push(format!("1{}", code));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_data = [
            (0, vec!("")),
            (1, vec!("0", "1")),
            (2, vec!("00", "01", "11", "10")),
            (3, vec!("000", "001", "011", "010", 
                     "110", "111", "101", "100")),
            (4, vec!("0000", "0001", "0010", "0011", 
                     "0100", "0101", "0110", "0111", 
                     "1000", "1001", "1010", "1011",
                     "1100", "1101", "1110", "1111")),
        ];

        test_data
            .iter()
            .for_each(|(n, out)| 
                assert_eq!(gray(*n), *out)
            );
    }
}

fn main() {
    // Приклад використання
    let n = 3;
    let gray_code = gray(n);
    println!("Gray code for {} bits: {:?}", n, gray_code);
}