#![allow(unused)]

use list_comprehension::*;

#[derive(Clone, Eq, PartialEq)]
struct TestS {
    name: String,
    age: i32,
}

#[test]
fn test_comp() {
    let shared_arr = [0, 1];

    let first_generator_syntax = comp![
        (a1, a2)
        , a1 <- shared_arr
        , a2 <- [2, 3]
    ];
    assert_eq!(first_generator_syntax, [(0, 2), (0, 3), (1, 2), (1, 3)]);

    let second_generator_syntax = comp![
        (a1, a2)
        , a1 in shared_arr
        , a2 in [2, 3]
    ];
    assert_eq!(second_generator_syntax, [(0, 2), (0, 3), (1, 2), (1, 3)]);

    let second_generator_syntax2 = comp![
        (name.clone(), age)
        , let t = TestS { name: "LiHua".to_string(), age: 114 }
        , TestS { name , ..} in [t.clone()]
        , TestS { age , ..}  in [t.clone()]
    ];
    assert_eq!(second_generator_syntax2, [("LiHua".to_string(), 114)]);

    let _empty_decls = comp![
        (),
        let { }
    ];

    let _all_syntax = comp![
        ()
        , a1 <- shared_arr
        , a2 <- [2, 3]
        , b1 in shared_arr
        , b2 in [2, 3]
        , let t = TestS { name: "LiHua".to_string(), age: 114 }
        , TestS { name , ..} in [t.clone()]
        , TestS { age , ..}  in [t.clone()]

        , n in shared_arr
        , m in [0, 1, 2]
        , n != 2
        , m != 2

        , let { a1 = 1; mut b1 = 2; c1: i8 = 3; mut d1: i8 = 4 }
        , let mut { a2 = 1; mut b2 = 2; c2: i8 = 3; mut d2: i8 = 4 }
        , let a3 = 1
        , let mut b3: i8 = 1
    ];
}