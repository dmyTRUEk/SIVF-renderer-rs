//! Simple Expr Eval, but with sqrt, sin, cos, etc

use evalexpr::eval;

use crate::utils::extensions::str::ExtensionCountChars;



const LB: char = '(';
const RB: char = ')';



trait ExtensionToStr {
    fn to_str(&self) -> &str;
}
impl ExtensionToStr for String {
    fn to_str(&self) -> &str { &self }
}
// is this even possible?
// impl ExtensionToStr for f64 {
//     fn to_str(&self) -> &str { &self.to_string() }
// }



trait ExtensionToF64 {
    fn to_f64(&self) -> f64;
}
impl ExtensionToF64 for evalexpr::Value {
    fn to_f64(&self) -> f64 {
        self.as_number().unwrap_or_else(|_| {
            let _value_string: String = self.as_string().unwrap();
            todo!()
        })
    }
}



trait ExtensionBracketBounds {
    fn bracket_bouns(&self) -> Option<(usize, usize)>;
}
impl ExtensionBracketBounds for &str {
    fn bracket_bouns(&self) -> Option<(usize, usize)> {
        let lbracket_count: usize = self.count_chars(LB);
        let rbracket_count: usize = self.count_chars(RB);
        assert_eq!(lbracket_count, rbracket_count);
        if lbracket_count == 0 && rbracket_count == 0 {
            return None;
        }
        assert!(self.find(LB) < self.find(RB));   // left bracket is LEFTEST than right
        let l: usize = self.find(LB).unwrap();
        let r: usize = {
            let mut bracket_count: i32 = 0;
            let mut res: Option<usize> = None;
            for (i, c) in self.char_indices() {
                match c {
                    LB => { bracket_count += 1; }
                    RB => {
                        bracket_count -= 1;
                        if bracket_count == 0 {
                            res = Some(i);
                            break;
                        }
                    }
                    _ => { continue; }
                }
            }
            res.unwrap()
        };
        Some((l, r))
    }
}



trait ExtensionExtractFromBrackets {
    fn extract_from_brackets(&self) -> Option<&str>;
}
impl ExtensionExtractFromBrackets for &str {
    fn extract_from_brackets(&self) -> Option<&str> {
        if let Some((l, r)) = self.bracket_bouns() {
            Some(&self[l+1..r])
        }
        else {
            None
        }
    }
}



trait ExtensionFindSubstr {
    fn find_substr(&self, substr: &str) -> Option<(usize, String)>;
}
impl ExtensionFindSubstr for &str {
    fn find_substr(&self, substr: &str) -> Option<(usize, String)> {
        assert_ne!(0, substr.len());
        if let Some(index) = self.find(substr) {
            Some((index, substr.to_string()))
        }
        else {
            None
        }
    }
}



const SQRT  : &str = "sqrt";
const SIN   : &str = "sin";
const COS   : &str = "cos";
const RANDOM: &str = "random";

const FUNCS: [&str; 4] = [
    SQRT,
    SIN,
    COS,
    RANDOM,
];

pub fn eval_expr(expr: &str) -> f64 {
    let expr: &str = expr.trim_start_matches('+');
    if !FUNCS.iter().any(|func_name| expr.contains(func_name)) {
        // simple case
        // println!("eval_expr.first : expr = {expr:?}");
        eval(expr).unwrap().to_f64()
    }
    else {
        // println!("eval_expr.second: expr = {expr:?}");
        let evaling_func: String = FUNCS.iter()
            .filter_map(|fname| expr.find_substr(fname))
            .min_by_key(|fpos_fname| fpos_fname.0)
            .unwrap().1;
        let brackets_insides: &str = expr.extract_from_brackets().unwrap();
        // println!("brackets_insides = {brackets_insides}");
        let brackets_insides_res: f64 = eval_expr(brackets_insides);
        let func_res: f64 = match evaling_func.to_str() {
            SQRT => { brackets_insides_res.sqrt() }
            SIN => { brackets_insides_res.sin() }
            COS => { brackets_insides_res.cos() }
            RANDOM => { todo!() }
            _ => { panic!() }
        };
        let expr: &str = &expr.replacen(&evaling_func, "", 1);
        let expr: &str = &expr.replacen(&format!("{LB}{brackets_insides}{RB}"), func_res.to_string().to_str(), 1);
        eval_expr(expr)
    }
}





#[cfg(test)]
mod tests {
    use super::{LB, RB, ExtensionBracketBounds, ExtensionExtractFromBrackets, eval_expr};

    #[test]
    fn bracket_bouns() {
        {
            let s: &str = "";
            assert_eq!(
                None,
                s.bracket_bouns()
            );
        }
        {
            let s: &str = "()";
            let ans : Option<(usize, usize)> = Some((0_usize, 1_usize));
            let res1: Option<(usize, usize)> = Some((s.find(LB).unwrap(), s.find(RB).unwrap()));
            let res2: Option<(usize, usize)> = s.bracket_bouns();
            assert_eq!(ans, res1);
            assert_eq!(ans, res2);
        }
        {
            let s: &str = "(abc)";
            let ans : Option<(usize, usize)> = Some((0_usize, 4_usize));
            let res1: Option<(usize, usize)> = Some((s.find(LB).unwrap(), s.find(RB).unwrap()));
            let res2: Option<(usize, usize)> = s.bracket_bouns();
            assert_eq!(ans, res1);
            assert_eq!(ans, res2);
        }
        {
            //            "0123456789"
            let s: &str = "ab(def)hij";
            let ans : Option<(usize, usize)> = Some((2_usize, 6_usize));
            let res1: Option<(usize, usize)> = Some((s.find(LB).unwrap(), s.find(RB).unwrap()));
            let res2: Option<(usize, usize)> = s.bracket_bouns();
            assert_eq!(ans, res1);
            assert_eq!(ans, res2);
        }
        // TODO: write tests with nested brackets
    }

    #[test]
    fn extract_from_brackets() {
        {
            let s: &str = "";
            let ans: Option<&str> = None;
            let res: Option<&str> = s.extract_from_brackets();
            assert_eq!(ans, res);
        }
        {
            let s: &str = "()";
            let ans: Option<&str> = Some("");
            let res: Option<&str> = s.extract_from_brackets();
            assert_eq!(ans, res);
        }
        {
            let s: &str = "(abc)";
            let ans: Option<&str> = Some("abc");
            let res: Option<&str> = s.extract_from_brackets();
            assert_eq!(ans, res);
        }
        {
            let s: &str = "ab(def)hij";
            let ans: Option<&str> = Some("def");
            let res: Option<&str> = s.extract_from_brackets();
            assert_eq!(ans, res);
        }
        {
            let s: &str = "a(b(def)hi)j";
            let ans: Option<&str> = Some("b(def)hi");
            let res: Option<&str> = s.extract_from_brackets();
            assert_eq!(ans, res);
        }
        {
            let s: &str = "a(b()(def)hi(()))()j";
            let ans: Option<&str> = Some("b()(def)hi(())");
            let res: Option<&str> = s.extract_from_brackets();
            assert_eq!(ans, res);
        }
        {
            let s: &str = "ab()(def)hi(())()j";
            let ans: Option<&str> = Some("");
            let res: Option<&str> = s.extract_from_brackets();
            assert_eq!(ans, res);
        }
    }

    #[test]
    fn eval_expr_() {
        {
            let s: &str = "sqrt(0)";
            let ans: f64 = 0.0;
            let res: f64 = eval_expr(s);
            assert_eq!(ans, res);
        }
        {
            let s: &str = "sqrt(1)";
            let ans: f64 = 1.0;
            let res: f64 = eval_expr(s);
            assert_eq!(ans, res);
        }
        {
            let s: &str = "sqrt(4)";
            let ans: f64 = 2.0;
            let res: f64 = eval_expr(s);
            assert_eq!(ans, res);
        }
        {
            let s: &str = "sqrt(9)";
            let ans: f64 = 3.0;
            let res: f64 = eval_expr(s);
            assert_eq!(ans, res);
        }
        {
            let s: &str = "sqrt(16)";
            let ans: f64 = 4.0;
            let res: f64 = eval_expr(s);
            assert_eq!(ans, res);
        }
        {
            let s: &str = "sqrt(100)";
            let ans: f64 = 10.0;
            let res: f64 = eval_expr(s);
            assert_eq!(ans, res);
        }
        {
            let s: &str = "sqrt(2)";
            let ans: f64 = 1.4142135623730951;
            let res: f64 = eval_expr(s);
            assert_eq!(ans, res);
        }
        {
            let s: &str = "sqrt(2)+sqrt(3)";
            let ans: f64 = 3.1462643699419726;
            let res: f64 = eval_expr(s);
            assert_eq!(ans, res);
        }
        {
            let angles: [f64; 13] = [0.0, 1.0, 5.0, 10.0, 20.0, 30.0, 40.0, 42.145, 45.0, 60.0, 70.0, 80.0, 90.0];
            for angle in angles {
                assert_eq!(angle.sin(), eval_expr(&format!("sin({angle})")));
                assert_eq!(angle.cos(), eval_expr(&format!("cos({angle})")));
            }
        }
    }
}

