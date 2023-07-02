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

    let second_generator_syntax_with_ident = comp![
        (a1, a2)
        , a1 in shared_arr
        , a2 in [2, 3]
    ];
    assert_eq!(second_generator_syntax_with_ident, [(0, 2), (0, 3), (1, 2), (1, 3)]);

    let second_generator_syntax_with_pat = comp![
        (name.clone(), age)
        , let t = TestS { name: "LiHua".to_string(), age: 114 }
        , TestS { name , ..} in [t.clone()]
        , TestS { age , ..}  in [t.clone()]
    ];
    assert_eq!(second_generator_syntax_with_pat, [("LiHua".to_string(), 114)]);

    let decl_with_pattern_matching = comp![
        name.clone()
        , let TestS { name, .. } = TestS { name: "LiHua".to_string(), age: 114 }
    ];
    assert_eq!(decl_with_pattern_matching, ["LiHua".to_string()]);

    let decl_with_mut_pattern_matching = comp![
        name.clone()
        , let TestS { mut name, .. } = TestS { name: "LiHua".to_string(), age: 114 }
        , _ in 0..1
        , { name = "Jack".to_string(); true }
    ];
    assert_eq!(decl_with_mut_pattern_matching, ["Jack".to_string()]);

    let decl_with_pattern_matching_and_let_else = comp![
        num
        , let Some(num) = Some(114) , else { panic!("Actually this panic shouldn't be called") }
    ];
    assert_eq!(decl_with_pattern_matching_and_let_else, [114]);

    let decls_with_pattern_matching = comp![
        (name.clone(), age)
        , let {
            TestS { name, .. } = TestS { name: "LiHua".to_string(), age: 114 };
            TestS { age, .. } = TestS { name: "LiHua".to_string(), age: 114 }
        }
    ];
    assert_eq!(decls_with_pattern_matching, [("LiHua".to_string(), 114)]);

    let decls_with_mut_pattern_matching = comp![
        (name.clone(), age)
        , let {
            TestS { mut name, .. } = TestS { name: "LiHua".to_string(), age: 114 };
            TestS { mut age, .. } = TestS { name: "LiHua".to_string(), age: 114 }
        }
        , _ in 0..1
        , {
            name = "Jack".to_string();
            age = 514;
            true
        }
    ];
    assert_eq!(decls_with_mut_pattern_matching, [("Jack".to_string(), 514)]);

    let decls_with_pattern_matching_and_let_else = comp![
        (num1, num2)
        , let {
            Some(num1) = Some(114) , else { panic!("Actually this panic shouldn't be called") };
            Some(num2) = Some(514) , else { panic!("Actually this panic shouldn't be called") }
        }
    ];
    assert_eq!(decls_with_pattern_matching_and_let_else, [(114, 514)]);

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
        // , TestS { mut name2 , ..} in [t.clone()]
        // , TestS { mut age2 , ..}  in [t.clone()]

        , n in shared_arr
        , m in [0, 1, 2]
        , n != 2
        , m != 2

        , let {
            a1 = 1; mut b1 = 2; c1: i8 = 3; mut d1: i8 = 4;
            TestS { name, .. } = TestS { name: "LiHua".to_string(), age: 114 };
            Some(num1) = Some(114) , else { panic!("Actually this panic shouldn't be called") }
        }
        , let mut { a2 = 1; mut b2 = 2; c2: i8 = 3; mut d2: i8 = 4;
            TestS { name, .. } = TestS { name: "LiHua".to_string(), age: 114 };
            Some(num1) = Some(114) , else { panic!("Actually this panic shouldn't be called") }
        }
        , let a3 = 1
        , let mut b3: i8 = 1
        , let TestS { name, .. } = TestS { name: "LiHua".to_string(), age: 114 }
        , let TestS { mut name, .. } = TestS { name: "LiHua".to_string(), age: 114 }
        , let {
            TestS { name, .. } = TestS { name: "LiHua".to_string(), age: 114 };
            TestS { age, .. } = TestS { name: "LiHua".to_string(), age: 114 }
        }
        , let {
            TestS { mut name, .. } = TestS { name: "LiHua".to_string(), age: 114 };
            TestS { mut age, .. } = TestS { name: "LiHua".to_string(), age: 114 }
        }
        , let Some(num) = Some(114) , else { panic!("Actually this panic shouldn't be called") }
        , let {
            Some(num1) = Some(114) , else { panic!("Actually this panic shouldn't be called") };
            Some(num2) = Some(514) , else { panic!("Actually this panic shouldn't be called") }
        }
    ];
}