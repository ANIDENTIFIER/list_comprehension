/// # Haskell List Comprehension Macro
/// A macro for Haskell-like list comprehensions in Rust
///
/// # Examples:
///  ```rust
///  use list_comprehension::comp;
///
///  // example 1
///  let arr1: Vec<i32> = comp![n => n in [0, 1, 2, 3], n != 3];
///  assert_eq!(arr1, [0, 1, 2]);
///
///  // example 2
///  let a = [0, 1, 2];
///  let arr2: Vec<(i32, i32)> = comp![
///      (n, m)
///      , n in a
///      , m in [0, 1, 2]
///      , n != 2
///      , m != 2
///  ];
///  assert_eq!(arr2, [(0, 0), (0, 1), (1, 0), (1, 1)]);
///  ```
/// More details can be found in README.md
///
#[macro_export]
macro_rules! comp {
    ($out:expr => $( $unparsed:tt )+) => {
        {
            let mut res = Vec::new();
            $crate::parse!(res; $out; $( $unparsed )+);

            res
        }
    };

    ($out:expr ; $( $unparsed:tt )+) => {
        {
            let mut res = Vec::new();
            $crate::parse!(res; $out; $( $unparsed )+);

            res
        }
    };
    ($out:expr , $( $unparsed:tt )+) => {
        {
            let mut res = Vec::new();
            $crate::parse!(res; $out; $( $unparsed )+);

            res
        }
    };
}

#[macro_export]
macro_rules! parse {
    // 递归展开
    (
        $res:ident;
        $out:expr;
        $var:ident <- $iter:ident
        $(, $( $unparsed:tt )* )?
    ) => {
        for $var in $iter {
            $crate::parse!(
                $res; $out; $($( $unparsed )*)?
            );
        }
    };

    (
        $res:ident;
        $out:expr;
        $var:ident <- $iter:expr
        $(, $( $unparsed:tt )* )?
    ) => {
        for $var in $iter {
            $crate::parse!(
                $res; $out; $($( $unparsed )*)?
            );
        }
    };

    (
        $res:ident;
        $out:expr;
        let $var:ident $(: $ty:ty)? = $expr:expr
        $(, $( $unparsed:tt )* )?
    ) => {
        let $var $(: $ty)? = $expr;

        $crate::parse!(
            $res; $out; $($( $unparsed )*)?
        );
    };

    (
        $res:ident;
        $out:expr;
        let mut $var:ident $(: $ty:ty)? = $expr:expr
        $(, $( $unparsed:tt )* )?
    ) => {
        let mut $var $(: $ty)? = $expr;

        $crate::parse!(
            $res; $out; $($( $unparsed )*)?
        );
    };

    (
        $res:ident;
        $out:expr;
        let { $( $let_stmts:tt )* }
        $(, $( $unparsed:tt )* )?
    ) => {
        $crate::let_parse_entrance!($( $let_stmts )*);

        $crate::parse!(
            $res; $out; $($( $unparsed )*)?
        );
    };

    (
        $res:ident;
        $out:expr;
        let mut { $( $let_stmts:tt )* }
        $(, $( $unparsed:tt )* )?
    ) => {
        $crate::let_parse_entrance!(all_mut ; $( $let_stmts )*);

        $crate::parse!(
            $res; $out; $($( $unparsed )*)?
        );
    };

    (
        $res:ident;
        $out:expr;
        $pred:expr
        $(, $( $unparsed:tt )* )?
    ) => {
        if !($pred) {
            continue;
        }

        $crate::parse!(
            $res; $out; $($( $unparsed )*)?
        );
    };

    (
        $res:ident;
        $out:expr;
        $var:pat in $iter:ident
        $(, $( $unparsed:tt )* )?
    ) => {
        for $var in $iter {
            $crate::parse!(
                $res; $out; $($( $unparsed )*)?
            );
        }
    };

    (
        $res:ident;
        $out:expr;
        $var:pat in $iter:expr
        $(, $( $unparsed:tt )* )?
    ) => {
        for $var in $iter {
            $crate::parse!(
                $res; $out; $($( $unparsed )*)?
            );
        }
    };

    // 结束条件
    (
        $res:ident;
        $out:expr;
    ) => {
        $res.push($out);
    };
}

#[macro_export]
macro_rules! let_parse_entrance {
    (all_mut ; $( $let_stmts:tt )*) => {
        $crate::let_parse_all_mut!($($let_stmts )*);
    };

    ($( $let_stmts:tt )*) => {
        $crate::let_parse!($($let_stmts )*);
    };
}

#[macro_export]
macro_rules! let_parse {
    (
        mut $var:ident $(: $ty:ty)? = $expr:expr
        $(; $( $let_stmts:tt )+ )?
    ) => {
        let mut $var $(: $ty)? = $expr;
        $crate::let_parse!($( $( $let_stmts )+ )?);
    };

    (
        $var:ident $(: $ty:ty)? = $expr:expr
        $(; $( $let_stmts:tt )+ )?
    ) => {
        let $var $(: $ty)? = $expr;
        $crate::let_parse!($( $( $let_stmts )+ )?);
    };

    // 结束条件
    () => {};
}

#[macro_export]
macro_rules! let_parse_all_mut {
    (
        mut $var:ident $(: $ty:ty)? = $expr:expr
        $(; $( $let_stmts:tt )+ )?
    ) => {
        let mut $var $(: $ty)? = $expr;
        $crate::let_parse_all_mut!($( $( $let_stmts )+ )?);
    };

    (
        $var:ident $(: $ty:ty)? = $expr:expr
        $(; $( $let_stmts:tt )+ )?
    ) => {
        let mut $var $(: $ty)? = $expr;
        $crate::let_parse_all_mut!($( $( $let_stmts )+ )?);
    };

    // 结束条件
    () => {};
}