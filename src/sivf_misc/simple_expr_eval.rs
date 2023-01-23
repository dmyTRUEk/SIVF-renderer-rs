//! Simple Expr Eval, but with sqrt, sin, cos, etc

use evalexpr::eval;

use crate::{
    sivf_misc::vals::Vals,
    utils::{
        extensions::{
            str::ExtensionCountChars,
            vec::ExtensionCollectToVec,
        },
        random::random,
    }
};



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
    eval_expr_with_vals(expr, None)
}

pub fn eval_expr_with_vals(expr: impl ToString, vals: Option<&Vals>) -> f64 {
    let expr = expr.to_string();
    let mut expr: &str = expr.trim();
    expr = expr.trim_start_matches('+');
    expr = expr.trim();
    let mut expr: String = expr.to_string();
    if let Some(vals) = vals {
        expr = vals.iter().fold(expr.to_string(),
            |acc, el| {
                acc.replace(&el.0, &format!("({})", &el.1.to_string()))
            }
        )
    }
    let expr: &str = expr.trim();
    if !FUNCS.iter().any(|func_name| expr.contains(func_name)) {
        // simple case: eval from evalexpr can handle it
        let expr: &str = &expr.replace("/", "*1.0/");
        eval(expr).unwrap().to_f64()
    }
    else {
        // complex case: we have to handle it by ourself
        let evaling_func: (usize, String) = FUNCS.iter()
            .filter_map(|fname| expr.find_substr(fname))
            .min_by_key(|fpos_fname| fpos_fname.0)
            .unwrap();
        if expr.bracket_bouns() == Some((0, expr.len())) {
            // case `(…)`
            eval_expr_with_vals(&expr[1..expr.len()-1], vals)
        }
        else if expr.find(LB).unwrap() < evaling_func.0 {
            // case `…(…func(…)…)…`
            let brackets_insides: &str = expr.extract_from_brackets().unwrap();
            let brackets_insides_res: f64 = eval_expr_with_vals(brackets_insides, vals);
            let br_bounds: (usize, usize) = expr.bracket_bouns().unwrap();
            let expr: &str = &format!(
                "{l}{fres}{r}",
                l = expr[..br_bounds.0].to_string(),
                fres = brackets_insides_res.to_string(),
                r = expr[br_bounds.1+1..].to_string()
            );
            eval_expr_with_vals(expr, vals)
        }
        else {
            // case `…func(…)`
            let brackets_insides: &str = expr.extract_from_brackets().unwrap();
            let func_res: f64 = match evaling_func.1.to_str() {
                SQRT => { eval_expr_with_vals(brackets_insides, vals).sqrt() }
                SIN => { eval_expr_with_vals(brackets_insides, vals).sin() }
                COS => { eval_expr_with_vals(brackets_insides, vals).cos() }
                RANDOM => {
                    assert_eq!(1, brackets_insides.count_chars(','));
                    let minmax = brackets_insides.splitn(2, ',').collect_to_vec();
                    let (min, max): (f64, f64) = (
                        eval_expr_with_vals(minmax[0], vals),
                        eval_expr_with_vals(minmax[1], vals)
                    );
                    random(min, max)
                }
                _ => { panic!() }
            };
            let expr: &str = &format!(
                "{l}{fres}{r}",
                l = expr[..evaling_func.0].to_string(),
                fres = func_res.to_string(),
                r = expr[evaling_func.0+evaling_func.1.len()+1+brackets_insides.len()+1..].to_string()
            );
            eval_expr_with_vals(expr, vals)
        }
    }
}





#[cfg(test)]
mod tests {
    use super::{
        ExtensionBracketBounds,
        ExtensionExtractFromBrackets,
        ExtensionFindSubstr,
        eval_expr,
    };

    #[test]
    fn bracket_bouns() {
        let test_cases: Vec<(Option<(usize, usize)>, &str)> = vec![
            (None, ""),
            (None, "abc"),
            (Some((0, 1)), "()"),
            (Some((0, 4)), "(abc)"),
            (Some((2, 6)), "ab(def)hij"),
            (Some((2, 12)), "ab(()def(()))hi(())j"),
            (Some((0, 8)), "(((abc)))def"),
            (Some((0, 8)), "(((abc)))def(((ghi)))"),
            (Some((0, 8)), "(((abc)))(def)(((ghi)))"),
        ];
        for (ans, input) in test_cases {
            assert_eq!(ans, input.bracket_bouns());
        }
    }

    #[test]
    fn extract_from_brackets() {
        let test_cases: Vec<(Option<&str>, &str)> = vec![
            (None, ""),
            (Some(""), "()"),
            (Some("abc"), "(abc)"),
            (Some("def"), "ab(def)hij"),
            (Some("b(def)hi"), "a(b(def)hi)j"),
            (Some("b()(def)hi(())"), "a(b()(def)hi(()))()j"),
            (Some(""), "ab()(def)hi(())()j"),
            (Some("((abc))"), "(((abc)))"),
            (Some("((abc))"), "(((abc)))+(def)"),
            (Some("((abc))"), "(((abc)))+(((def)))"),
        ];
        for (ans, input) in test_cases {
            assert_eq!(ans, input.extract_from_brackets());
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

            (8.122417494872465 , "(((sqrt(2)+3*sqrt(5))))"),
            (8.122417494872465 , "((((((sqrt(2))))+(((3)))*(((sqrt(5)))))))"),

            ( 43.30127018922193, "(sqrt(3)/4)*100"),
            (-43.30127018922193, "-(sqrt(3)/4)*100"),
        ];
        for (ans, input) in test_cases {
            assert_eq!(ans, eval_expr(input));
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
        for (ans, input1, input2) in test_cases {
            if let Some(ans) = ans.clone() {
                assert_eq!(ans.0, input1.find(input2).unwrap());
            }
            assert_eq!(ans, input1.find_substr(input2));
        }
    }
}

