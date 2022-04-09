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

static mut IND: usize = 0;
fn make_ind() -> String { "    ".repeat(unsafe { IND }) }

pub fn eval_expr(expr: &str) -> f64 {
    let ind: String = make_ind();
    println!("{ind}----STARTING EVAL_EXPR with expr = {expr:?}");
    unsafe {
        IND += 1;
    }
    let ind: String = make_ind();

    let expr: &str = expr.trim();
    let expr: &str = expr.trim_start_matches('+');
    let expr: &str = expr.trim();
    let res: f64 = 
    if !FUNCS.iter().any(|func_name| expr.contains(func_name)) {
        // simple case
        println!("{ind}eval_expr -> SIMPLE");
        println!("{ind}expr = {expr:?}");
        let expr: &str = &expr.replace("/", "*1.0/");
        println!("{ind}expr = {expr:?}");
        eval(expr).unwrap().to_f64()
    }
    else {
        println!("{ind}eval_expr -> COMPLEX");
        println!("{ind}expr = {expr:?}");
        let evaling_func: (usize, String) = FUNCS.iter()
            .filter_map(|fname| expr.find_substr(fname))
            .min_by_key(|fpos_fname| fpos_fname.0)
            .unwrap();
        if expr.starts_with(LB) && expr.ends_with(RB) {
            println!("{ind}--CASE 0: `(…)`");
            eval_expr(&expr[1..expr.len()-1])
        }
        else if expr.find(LB).unwrap() < evaling_func.0 {
            println!("{ind}--CASE 1: `…(…func(…)…)…`");
            let brackets_insides: &str = expr.extract_from_brackets().unwrap();
            println!("{ind}brackets_insides = {brackets_insides}");
            let brackets_insides_res: f64 = eval_expr(brackets_insides);
            println!("{ind}expr: {expr:?}");
            let br_bounds: (usize, usize) = expr.bracket_bouns().unwrap();
            let expr: &str = &format!(
                "{l}{fres}{r}",
                l = expr[..br_bounds.0].to_string(),
                fres = brackets_insides_res.to_string(),
                r = expr[br_bounds.1+1..].to_string()
            );
            println!("{ind}expr after 'replace': {expr:?}");
            eval_expr(expr)
        }
        else {
            println!("{ind}--CASE 2: `…func(…)`");
            let brackets_insides: &str = expr.extract_from_brackets().unwrap();
            println!("{ind}brackets_insides = {brackets_insides}");
            let brackets_insides_res: f64 = eval_expr(brackets_insides);
            let func_res: f64 = match evaling_func.1.to_str() {
                SQRT => { brackets_insides_res.sqrt() }
                SIN => { brackets_insides_res.sin() }
                COS => { brackets_insides_res.cos() }
                RANDOM => { todo!() }
                _ => { panic!() }
            };
            println!("{ind}func_res: {func_res}");
            println!("{ind}expr: {expr:?}");
            let expr: &str = &format!(
                "{l}{fres}{r}",
                l = expr[..evaling_func.0].to_string(),
                fres = func_res.to_string(),
                r = expr[evaling_func.0+evaling_func.1.len()+1+brackets_insides.len()+1..].to_string()
            );
            println!("{ind}expr after 'replace': {expr:?}");
            eval_expr(expr)
        }
    };
    unsafe {
        IND -= 1;
    }
    let ind: String = make_ind();
    println!("{ind}----ENDING EVAL_EXPR with res = {res}");
    res
}





#[cfg(test)]
mod tests {
    use crate::utils::simple_expr_eval::ExtensionFindSubstr;

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
        {
            //            "01234567890123456789"
            let s: &str = "ab(()def(()))hi(())j";
            let ans: Option<(usize, usize)> = Some((2_usize, 12_usize));
            let res: Option<(usize, usize)> = s.bracket_bouns();
            assert_eq!(ans, res);
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
        let test_cases: Vec<(f64, &str)> = vec![
            (0.0, "0"),
            (1.0, "1"),
            (145.237, "145.237"),

            (0.0, "(0)"),
            (1.0, "(1)"),
            (145.237, "(145.237)"),
            (145.237, "(((((145.237)))))"),

            (5.0, "(1/2)*10"),

            (0.0, "sqrt(0)"),
            (1.0, "sqrt(1)"),
            (2.0, "sqrt(4)"),
            (3.0, "sqrt(9)"),
            (4.0, "sqrt(16)"),
            (10.0, "sqrt(100)"),
            (1.4142135623730951, "sqrt(2)"),

            (0.0, "(sqrt(0))"),
            (1.0, "(sqrt(1))"),
            (2.0, "(sqrt(4))"),
            (3.0, "(sqrt(9))"),
            (4.0, "(sqrt(16))"),
            (10.0, "(sqrt(100))"),
            (1.4142135623730951, "(sqrt(2))"),

            (2.0, "(((((((((((sqrt(4))))))))))))"),

            (3.1462643699419726, "sqrt(2)+sqrt(3)"),
            (8.122417494872465 , "sqrt(2)+3*sqrt(5)"),

            (3.1462643699419726, "(sqrt(2)+sqrt(3))"),
            (8.122417494872465 , "(sqrt(2)+3*sqrt(5))"),

            (8.122417494872465 , "((((((((((((((((sqrt(2)+3*sqrt(5)))))))))))))))))"),

            ( 43.30127018922193, "(sqrt(3)/4)*100"),
            (-43.30127018922193, "-(sqrt(3)/4)*100"),
        ];
        for test_case in test_cases {
            assert_eq!(test_case.0, eval_expr(test_case.1));
        }
        {
            let angles: [f64; 13] = [0.0, 1.0, 5.0, 10.0, 20.0, 30.0, 40.0, 42.145, 45.0, 60.0, 70.0, 80.0, 90.0];
            for angle in angles {
                assert_eq!(angle.sin(), eval_expr(&format!("sin({angle})")));
                assert_eq!(angle.cos(), eval_expr(&format!("cos({angle})")));
            }
        }
    }

    #[test]
    fn find_substr() {
        let test_cases: Vec<(Option<(usize, String)>, &str, &str)> = vec![
            (None, "abcdef", "xyz"),
            (Some((3, "def".to_string())), "abcdefgh", "def"),
        ];
        // todo: write map, so dont write .to_string every time
        for test_case in test_cases {
            assert_eq!(test_case.0, test_case.1.find_substr(test_case.2));
        }
    }
}

