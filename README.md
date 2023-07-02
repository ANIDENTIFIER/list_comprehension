# Haskell List Comprehension Macro
A macro for Haskell-like list comprehensions in Rust

# Grammars:
```
ListComprehensionExp:
    comp![ Exp , Qual1 , . . . , Qualn ]  (list comprehension, n ≥ 1 )
  | comp![ Exp ; Qual1 , . . . , Qualn ]  (list comprehension, n ≥ 1 )
  | comp![ Exp => Qual1 , . . . , Qualn ] (list comprehension, n ≥ 1 )

Qual:
    Ident <- Exp          (generator)
  | Pattern in Exp        (generator)
  | let Decl              (local declaration)
  | let (mut)? Decls      (local declaration[1])
  | Exp(bool)             (boolean guard)
 
Decls:
    { Decl1 ; . . . ; Decln }     (n ≥ 0)
    
Decl:
    (mut)? Ident ( : Type )? = Exp
  | Pat = Exp ( , else { ... } )?
  
  
  
[1] if `mut` is used, then all declarations will be added with `mut` unless pattern matching is used
```

# Examples:
```rust
fn main() {
    // example 1
    let arr1: Vec<i32> = comp![n => n in [0, 1, 2, 3], n != 3];
    assert_eq!(arr1, [0, 1, 2]);
    
    // expand the macro:
    let arr1: Vec<i32> = {
        let mut res = Vec::new();
        for n in [0, 1, 2, 3] {
            if !(n != 3) {
                continue;
            }
            res.push(n);
        }
        res
    };

// ----------------------------------------------------------------------------------------------------

    // example 2
    let a = [0, 1, 2];
    let arr2: Vec<(i32, i32)> = comp![
        (n, m)
        , n in a
        , m in [0, 1, 2]
        , n != 2
        , m != 2
    ];
    assert_eq!(arr2, [(0, 0), (0, 1), (1, 0), (1, 1)]);
    
    // expand the macro:
    let arr2: Vec<(i32, i32)> = {
        let mut res = Vec::new();
        for n in a {
            for m in [0, 1, 2] {
                if !(n != 2) {
                    continue;
                }
                if !(m != 2) {
                    continue;
                }
                res.push((n, m));
            }
        }
        res
    };

// ----------------------------------------------------------------------------------------------------
    
    // example 3
    let arr = comp![
        ()
        , let { a1 = 1; mut b1 = 2; c1: i8 = 3; mut d1: i8 = 4 }
        , let mut { a2 = 1; mut b2 = 2; c2: i8 = 3; mut d2: i8 = 4 }
        , let a3 = 1
        , let mut b3: i8 = 1
        , let Some(num) = Some(114) , else { panic!("Actually this panic shouldn't be called") }
    ];
    
    // expand the macro: 
    let arr = {
        let mut res = Vec::new();
        
        let a1 = 1;
        let mut b1 = 2;
        let c1: i8 = 3;
        let mut d1: i8 = 4;
        
        let mut a2 = 1;
        let mut b2 = 2;
        let mut c2: i8 = 3;
        let mut d2: i8 = 4;
        
        let a3 = 1;
        let mut b3: i8 = 1;
        
        let Some(num) = Some(114) else {
            panic!("Actually this panic shouldn't be called")
        };
        
        res.push(());
        res
    };
  
    // You can see more examples in tests/test_comp.rs
}
```

# Update
* v0.1.4: 
  * Allow `pattern matching` and a bit different `let else` in `local declaration`.
  * Made some optimizations.
  * Corrected a little bit of mistakes in README.