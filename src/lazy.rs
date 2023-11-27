use genawaiter::sync::{Co, Gen};

use std::future::Future;
use std::marker::PhantomData;

pub struct LazyComp<Y, F, Fu>
where
    F: Clone + FnOnce(Co<Y>) -> Fu,
    Fu: Future<Output = ()>,
{
    func: F,
    _m: PhantomData<(Y, Fu)>,
}

impl<Y, F, Fu> LazyComp<Y, F, Fu>
where
    F: Clone + FnOnce(Co<Y>) -> Fu,
    Fu: Future<Output = ()>,
{
    pub fn new(f: F) -> LazyComp<Y, F, Fu> {
        LazyComp {
            func: f,
            _m: PhantomData,
        }
    }
}

impl<Y, F, Fu> IntoIterator for LazyComp<Y, F, Fu>
where
    F: Clone + FnOnce(Co<Y>) -> Fu,
    Fu: Future<Output = ()>,
{
    type Item = Y;
    type IntoIter = IntoIter<Y, Fu>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            generator: Gen::new(self.func),
        }
    }
}

impl<Y, F, Fu> Clone for LazyComp<Y, F, Fu>
where
    F: Clone + FnOnce(Co<Y>) -> Fu,
    Fu: Future<Output = ()>,
{
    fn clone(&self) -> Self {
        LazyComp {
            func: self.func.clone(),
            _m: PhantomData,
        }
    }
}

pub struct IntoIter<Y, F: Future<Output = ()>> {
    generator: Gen<Y, (), F>,
}

impl<Y, F: Future<Output = ()>> Iterator for IntoIter<Y, F> {
    type Item = Y;

    fn next(&mut self) -> Option<Self::Item> {
        match self.generator.resume() {
            genawaiter::GeneratorState::Yielded(x) => Some(x),
            genawaiter::GeneratorState::Complete(()) => None,
        }
    }
}

#[macro_export]
macro_rules! lazy_comp {
    ($out:expr => $( $unparsed:tt )+) => {
        {
            LazyComp::new(|co: genawaiter::sync::Co<_>| async move {
                $crate::lazy_parse!(co; $out; $( $unparsed )+);
            })
        }
    };

    ($out:expr , $( $unparsed:tt )+) => {
        {
            LazyComp::new(|co: genawaiter::sync::Co<_>| async move {
                $crate::lazy_parse!(co; $out; $( $unparsed )+);
            })
        }
    };

    ($out:expr ; $( $unparsed:tt )+) => {
        {
            LazyComp::new(|co: genawaiter::sync::Co<_>| async move {
                $crate::lazy_parse!(co; $out; $( $unparsed )+);
            })
        }
    };
}

#[macro_export]
macro_rules! lazy_parse {
    // 递归展开
    (
        $co:ident;
        $out:expr;
        $var:ident in $iter:expr
        $(, $( $unparsed:tt )* )?
    ) => {
        for $var in $iter {
            $crate::lazy_parse!(
                $co; $out; $($( $unparsed )*)?
            );
        }
    };

    (
        $co:ident;
        $out:expr;
        $var:ident <- $iter:expr
        $(, $( $unparsed:tt )* )?
    ) => {
        for $var in $iter {
            $crate::lazy_parse!(
                $co; $out; $($( $unparsed )*)?
            );
        }
    };

    (
        $co:ident;
        $out:expr;
        let $var:ident $(: $ty:ty)? = $expr:expr
        $(, $( $unparsed:tt )* )?
    ) => {
        let $var $(: $ty)? = $expr;

        $crate::lazy_parse!(
            $co; $out; $($( $unparsed )*)?
        );
    };

    (
        $co:ident;
        $out:expr;
        let mut $var:ident $(: $ty:ty)? = $expr:expr
        $(, $( $unparsed:tt )* )?
    ) => {
        let mut $var $(: $ty)? = $expr;

        $crate::lazy_parse!(
            $co; $out; $($( $unparsed )*)?
        );
    };

    (
        $co:ident;
        $out:expr;
        let { $( $let_stmts:tt )* }
        $(, $( $unparsed:tt )* )?
    ) => {
        $crate::let_parse_entrance!($( $let_stmts )*);

        $crate::lazy_parse!(
            $co; $out; $($( $unparsed )*)?
        );
    };

    (
        $co:ident;
        $out:expr;
        let mut { $( $let_stmts:tt )* }
        $(, $( $unparsed:tt )* )?
    ) => {
        $crate::let_parse_entrance!(all_mut @@ $( $let_stmts )*);

        $crate::lazy_parse!(
            $co; $out; $($( $unparsed )*)?
        );
    };

    (
        $co:ident;
        $out:expr;
        let $var:pat = $expr:expr , else { $( $else_code:tt )* }
        $(, $( $unparsed:tt )* )?
    ) => {
        let $var = $expr else { $( $else_code )* };

        $crate::lazy_parse!(
            $co; $out; $($( $unparsed )*)?
        );
    };

    (
        $co:ident;
        $out:expr;
        let $var:pat = $expr:expr
        $(, $( $unparsed:tt )* )?
    ) => {
        let $var = $expr;

        $crate::lazy_parse!(
            $co; $out; $($( $unparsed )*)?
        );
    };

    (
        $co:ident;
        $out:expr;
        for $var:pat in $iter:expr
        $(, $( $unparsed:tt )* )?
    ) => {
        for $var in $iter {
            $crate::lazy_parse!(
                $co; $out; $($( $unparsed )*)?
            );
        }
    };

    (
        $co:ident;
        $out:expr;
        $pred:expr
        $(, $( $unparsed:tt )* )?
    ) => {
        if !($pred) {
            continue;
        }

        $crate::lazy_parse!(
            $co; $out; $($( $unparsed )*)?
        );
    };

    (
        $co:ident;
        $out:expr;
        $let_stmt:stmt
        $(, $( $unparsed:tt )* )?
    ) => {
        $let_stmt;

        $crate::lazy_parse!(
            $co; $out; $($( $unparsed )*)?
        );
    };

    (
        $co:ident;
        $out:expr;
        $var:pat in $iter:expr
        $(, $( $unparsed:tt )* )?
    ) => {
        for $var in $iter {
            $crate::lazy_parse!(
                $co; $out; $($( $unparsed )*)?
            );
        }
    };


    // 结束条件
    (
        $co:ident;
        $out:expr;
    ) => {
        $co.yield_($out).await;
    };
}
