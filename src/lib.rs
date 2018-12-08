macro_rules! whered {
    ($n:ident = $e:expr) => {
        macro_rules! $n {
            () => {
                $e
            };
        }
    };
    ($n:ident $arg_name:ident = $e:expr) => {
        macro_rules! $n {
            ($arg_expr:expr) => {{
                let $arg_name = $arg_expr;
                $e
            }};
        }
    };
}

#[test]
fn whered_macro() {
    let nums_plus1 = vec![2, 3, 4];

    let nums = [1, 2, 3];
    let iter = nums.iter();

    let mut nums = [1, 2, 3];
    let iter_mut = nums.iter_mut();

    let nums = [1, 2, 3];
    let into_iter = nums.into_iter();

    whered! { plus_one i = i.map(|n| *n + 1) };

    assert_eq!(nums_plus1, plus_one!(iter).collect::<Vec<_>>());
    assert_eq!(nums_plus1, plus_one!(iter_mut).collect::<Vec<_>>());
    assert_eq!(nums_plus1, plus_one!(into_iter).collect::<Vec<_>>());
}
