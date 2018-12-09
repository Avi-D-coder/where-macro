macro_rules! whered {
    ($($n:ident $($arg_name:ident)* = $e:expr)+) => {
        $( macro_rules! $n {
            ($($arg_name: $arg_expr:expr)*) => {{
                $(let $arg_name = $arg_expr;)*
                $e
            }};
        })+
    };
}

#[test]
fn map_plus_one() {
    let nums_plus1 = vec![2, 3, 4];

    let nums = [1, 2, 3];
    let iter = nums.iter();

    let mut nums = [1, 2, 3];
    let iter_mut = nums.iter_mut();

    let nums = [1, 2, 3];
    let into_iter = nums.into_iter();

    whered! { plus_one i = i.map(|n| *n + 1) };

    assert_eq!(nums_plus1, plus_one!(i: iter).collect::<Vec<_>>());
    assert_eq!(nums_plus1, plus_one!(i: iter_mut).collect::<Vec<_>>());
    assert_eq!(nums_plus1, plus_one!(i: into_iter).collect::<Vec<_>>());
}

#[test]
fn no_arg() {
    whered! { foo = "foo"};
    assert_eq!("foo", foo!())
}

// FIXME
#[test]
fn two_args() {
    whered! {
        add n1 n2 = {n1 + n2}
    }
}

#[test]
fn multiple_definitions() {
    whered! {
        one = 1
        two = 2
        one_plus_two = one!() + two!()

        two_plus n = n + 2
    }

    assert_eq!(3, one_plus_two!());
    assert_eq!(4, two_plus!(n: 2));
}
