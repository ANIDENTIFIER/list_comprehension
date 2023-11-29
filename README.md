# List Comprehension
A macro for Haskell-like list comprehensions in Rust

# Syntax:
```
ListComprehensionExp:
    comp![ Exp , Qual1 , . . . , Qualn ]  (list comprehension, n ≥ 1 )
  | comp![ Exp ; Qual1 , . . . , Qualn ]  (list comprehension, n ≥ 1 )
  | comp![ Exp => Qual1 , . . . , Qualn ] (list comprehension, n ≥ 1 )
  | lazy_comp![ Exp , Qual1 , . . . , Qualn ]  (lazy list comprehension, n ≥ 1 )
  | lazy_comp![ Exp ; Qual1 , . . . , Qualn ]  (lazy list comprehension, n ≥ 1 )
  | lazy_comp![ Exp => Qual1 , . . . , Qualn ] (lazy list comprehension, n ≥ 1 )

Qual:
    Ident <- Exp            (generator)
  | (for)? Pattern in Exp   (generator[2])
  | let Decl                (local declaration)
  | let DeclWithElse        (local declaration)
  | let (mut)? Decls        (local declaration[1])
  | Exp(bool)               (boolean guard)
 
Decls:
    { Decl1 ; . . . ; Decln }     (n ≥ 0)
    
Decl:
    (mut)? Ident ( : Type )? = Exp
  | Pat = Exp ( , else { ... } )?

DeclWithElse:
   Pat = Exp else { ... }
  
  
[1] if `mut` is used, then all declarations will be added with `mut` unless pattern matching is used
[2] Since parsing the `Pattern in Exp` syntax and the `Exp` (boolean guard) syntax can be ambiguous,
    you can now force the `Pattern in Exp` syntax by prefixing the `Pattern` with `for`
```


# Examples:
```rust
fn main() {
    // example 1
    let arr1: Vec<i32> = comp![n; n in [0, 1, 2, 3], n != 3];
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
    let arr3 = lazy_comp![
        { println!("{i}"); i } 
        , i in 0..3
    ];
    
    for _ in arr3 {
        println!("------")
    }
  
    // console output:
    // 0
    // ------
    // 1
    // ------
    // 2
    // ------
  
    // You can see more examples in tests/test_comp.rs
}
```

# Update
* v0.2.0:
  * Added `lazy_comp` macro, which supports lazy evaluation. Its syntax is the same as `comp!`.
* v0.1.5:
  * Supports original `let else` syntax, but you can't use it in the `let { ... }` syntax.
  * Now you can force the `Pattern in Exp` syntax by prefixing the `Pattern` with `for` (See Syntax for details).
* v0.1.4: 
  * Allow `pattern matching` and a bit different `let else` in `local declaration`.
  * Made some optimizations.
  * Corrected a little bit of mistakes in README.