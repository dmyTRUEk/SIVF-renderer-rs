/// Macros.
///
/// Debug they by [`trace_macros!`]:
/// ```
/// trace_macros!(true);
/// your_macro!(…);
/// trace_macros!(false);
/// ```

#[macro_export]
macro_rules! swap {
    ($a:expr, $b:expr) => { {
        let t = $a;
        $a = $b;
        $b = t;
    } };
}


#[macro_export]
macro_rules! unmut {
    ($x:tt) => {
        let $x = $x;
    };
}



#[cfg(test)]
mod tests {
    #[test]
    fn swap() {
        {
            let mut x: i32 = 4;
            let mut y: i32 = 5;
            assert!(x < y);
            swap!(x, y);
            assert!(x > y);
        }
        {
            let mut x: i32 = 4;
            let mut y: i32 = 5;
            assert!(x < y);
            swap!(y, x);
            assert!(x > y);
        }
        {
            let mut x: f32 = 4.0;
            let mut y: f32 = 5.0;
            assert!(x < y);
            swap!(x, y);
            assert!(x > y);
        }
        {
            let mut array: [i32; 5] = [0, 1, 2, 3, 4];
            assert_eq!([0, 1, 2, 3, 4], array);
            swap!(array[1], array[3]);
            assert_eq!([0, 3, 2, 1, 4], array);
        }
    }

    #[test]
    fn unmut() {
        {
            let x = 42;
            unmut!(x);
            let x = 145;
        }
        {
            let mut x = 42;
            x = 137;
            unmut!(x);
            let x = 145;
        }
    }
}

