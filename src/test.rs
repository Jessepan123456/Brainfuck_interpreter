#[cfg(test)]
mod tests {
    use crate::InputMode;
    use crate::scan::{inital_scan, scan_result};
    use std::collections::HashMap;

    #[test]
    fn inital_scan_valid() {
        let input = "+++[-]";
        let mut map: HashMap<usize, usize> = HashMap::new();
        let mut my_stack: Vec<usize> = Vec::new();

        let result = inital_scan(input, &mut map, &mut my_stack);
        assert_eq!(result, true)
    }

    #[test]
    fn inital_scan_valid_longer() {
        let input = "++[>++[>+<-]<-]";
        let mut map: HashMap<usize, usize> = HashMap::new();
        let mut my_stack: Vec<usize> = Vec::new();

        let result = inital_scan(input, &mut map, &mut my_stack);
        assert_eq!(result, true)
    }

    #[test]
    fn inital_scan_invalid_right_bracket() {
        let input = "[[]";
        let mut map: HashMap<usize, usize> = HashMap::new();
        let mut my_stack: Vec<usize> = Vec::new();

        let result = inital_scan(input, &mut map, &mut my_stack);
        assert_eq!(result, false)
    }

    #[test]
    fn inital_scan_invalid_left_bracket() {
        let input = "[]]";
        let mut map: HashMap<usize, usize> = HashMap::new();
        let mut my_stack: Vec<usize> = Vec::new();

        let result = inital_scan(input, &mut map, &mut my_stack);
        assert_eq!(result, false)
    }

    #[test]
    fn scan_result_valid() {
        let mut brainfuck: Vec<u8> = vec![0, 0, 0, 0, 0];
        let mut ptr = 0;
        let mut i = 0;
        let map: HashMap<usize, usize> = HashMap::new();
        let mode = InputMode::Ascii;

        let command: char = '+';

        scan_result(&mut brainfuck, &mut ptr, &command, &mut i, &map, &mode);

        let brainfuck_check: Vec<u8> = vec![1, 0, 0, 0, 0];
        assert_eq!(brainfuck, brainfuck_check)
    }
}
