
use std::collections::HashMap;

lazy_static! {
    static ref DEST: HashMap<String, String> = {
        let mut m = HashMap::new();
        m.insert("M".to_string(), "001".to_string());
        m.insert("D".to_string(), "010".to_string());
        m.insert("MD".to_string(), "011".to_string());
        m.insert("A".to_string(), "100".to_string());
        m.insert("AM".to_string(), "101".to_string());
        m.insert("AD".to_string(), "110".to_string());
        m.insert("AMD".to_string(), "111".to_string());
        m.insert("".to_string(), "000".to_string());
        m
    };
    static ref JUMP: HashMap<String, String> = {
        let mut m = HashMap::new();
        m.insert("JGT".to_string(), "001".to_string());
        m.insert("JEQ".to_string(), "010".to_string());
        m.insert("JGE".to_string(), "011".to_string());
        m.insert("JLT".to_string(), "100".to_string());
        m.insert("JNE".to_string(), "101".to_string());
        m.insert("JLE".to_string(), "110".to_string());
        m.insert("JMP".to_string(), "111".to_string());
        m.insert("".to_string(), "000".to_string());
        m
    };
    static ref COMP: HashMap<String, String> = {
        let mut m = HashMap::new();
        m.insert("0".to_string(), "0101010".to_string());
        m.insert("1".to_string(), "0111111".to_string());
        m.insert("-1".to_string(), "0111010".to_string());
        m.insert("D".to_string(), "0001100".to_string());
        m.insert("A".to_string(), "0110000".to_string());
        m.insert("M".to_string(), "1110000".to_string());
        m.insert("!D".to_string(), "0001101".to_string());
        m.insert("!A".to_string(), "0110001".to_string());
        m.insert("!M".to_string(), "1110001".to_string());
        m.insert("-D".to_string(), "0001111".to_string());
        m.insert("-A".to_string(), "0110011".to_string());
        m.insert("-M".to_string(), "1110011".to_string());
        m.insert("D+1".to_string(), "0011111".to_string());
        m.insert("A+1".to_string(), "0110111".to_string());
        m.insert("M+1".to_string(), "1110111".to_string());
        m.insert("D-1".to_string(), "0001110".to_string());
        m.insert("A-1".to_string(), "0110010".to_string());
        m.insert("M-1".to_string(), "1110010".to_string());
        m.insert("D+A".to_string(), "0000010".to_string());
        m.insert("D+M".to_string(), "1000010".to_string());
        m.insert("D-A".to_string(), "0010011".to_string());
        m.insert("D-M".to_string(), "1010011".to_string());
        m.insert("A-D".to_string(), "0000111".to_string());
        m.insert("M-D".to_string(), "1000111".to_string());
        m.insert("D&A".to_string(), "0000000".to_string());
        m.insert("D&M".to_string(), "1000000".to_string());
        m.insert("D|A".to_string(), "0101010".to_string());
        m.insert("D|M".to_string(), "1101010".to_string());
        m.insert("".to_string(), "0000000".to_string());
        m
    };

}

pub fn dest(code: String) -> String {
    let d = &"".to_string();
    return DEST.get(&code).unwrap_or(d).to_string();
}

pub fn jump(code: String) -> String {
    let d = &"".to_string();
    return JUMP.get(&code).unwrap_or(d).to_string();
}

pub fn comp(code: String) -> String {
    let d = &"".to_string();
    return COMP.get(&code).unwrap_or(d).to_string();
}


#[cfg(test)]
mod tests {
    use crate::code::dest;
    use crate::code::jump;
    use crate::code::comp;
    #[test]
    fn dest_W01() {
        let actual = dest("M".to_string());
        assert_eq!(actual, "001".to_string());
    }
    #[test]
    fn dest_W02() {
        let actual = dest("".to_string());
        assert_eq!(actual, "000".to_string());
    }
    #[test]
    fn jump_W01() {
        let actual = jump("JGT".to_string());
        assert_eq!(actual, "001".to_string());
    }
    #[test]
    fn jump_W02() {
        let actual = jump("".to_string());
        assert_eq!(actual, "000".to_string());
    }
    #[test]
    fn comp_W01() {
        let actual = comp("D+1".to_string());
        assert_eq!(actual, "0011111".to_string());
    }
    #[test]
    fn comp_W02() {
        let actual = comp("".to_string());
        assert_eq!(actual, "0000000".to_string());
    }
}
