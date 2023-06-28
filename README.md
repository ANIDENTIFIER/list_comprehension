# Haskell List Comprehension Macro
Macro for Haskell-like list comprehensions in Rust

# Grammars:
```
list-comprehension-exp:
    [ exp , qual1 , . . . , qualn ]  (list comprehension, n ≥ 1 )
    [ exp ; qual1 , . . . , qualn ]  (list comprehension, n ≥ 1 )
    [ exp => qual1 , . . . , qualn ] (list comprehension, n ≥ 1 )

qual:
    pat <- exp     (generator)
  | pat in exp     (generator like rust)
  | let decl       (local declaration)
  | let decls      (local declaration)
  | exp(bool)      (boolean guard)
 
decls:   // In the future, decl will be allowed to be used in decls
    { partial-decl1 ; . . . ; partial-decln }     (n ≥ 0)

partial-decl:  
    (mut)? ident = exp
    
decl:
    let (mut)? ident( : Type )? = exp
```

# Examples:
```rust
fn main() {
    // example 1
    let arr1: Vec<i32> = comp![n => n in [0, 1, 2, 3], n != 3];
    assert_eq!(arr1, [0, 1, 2]);
    
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
}
```