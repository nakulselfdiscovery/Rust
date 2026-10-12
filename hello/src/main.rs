fn main() {
    println!("Hello, world!");
    let name = "Rust";
    println!("Hello, {name}!");          // inline formatting, like std::format
    println!("{} + {} = {}", 2, 3, 2 + 3);
    
    //NOTE: In C++, everything is mutable unless you write const. In Rust, everything is immutable unless you write mut.

    let x = 5;          // like: const auto x = 5;
    // x = 6;           // ERROR: cannot assign twice to immutable variable

    let mut y = 5;      // like: auto y = 5;
    println!("Old Value {}", y);
    y = 6;              // fine

    let z: i64 = 10;    // explicit type annotation (type comes after the name)
   
    println!("{} {} {}", x, y, z);	
    /*
       Shadowing:
       Rust lets you re-declare a name in the same scope. This is not mutation. 
       It creates a new variable that hides the old one.
    */ 
   
    let input = "Nakul";
    println!("Name= {}", input);
    //let input = "Chauhan";
    let input = 100;
	
   println!("Name= {}", input);
   //In C++ this would be a redefinition error. In Rust it’s idiomatic for transformations, because you avoid inventing names like input_str and input_int.


   /*
   In Rust, the primary difference is that const defines a compile-time constant, while an immutable variable (the default let binding) defines a runtime value that cannot be changed after initialization.
   While both prevent a value from being modified, they operate at completely different stages of a program's lifecycle.
   */
   const MAX_POINTS: u32 = 100_000;   // compile-time constant, like constexpr
   static GREETING:&str = "hi";       // a single global with a fixed address

  println!("{} {}", MAX_POINTS, GREETING);


  /*
   Two important differences from C++:
   1. No implicit conversions. let a: i64 = some_i32; is a compile error. You must write some_i32 as i64 (a raw cast, like static_cast), or better, i64::from(some_i32), which only exists for lossless conversions. This eliminates the whole class of silent narrowing and signed/unsigned comparison bugs.
   
  2. Integer overflow is defined. In debug builds it panics (crashes with a message). In release builds it wraps. Signed overflow is never undefined behavior. If you want explicit behavior, there are methods like x.wrapping_add(y), x.checked_add(y) (returns Option), and x.saturating_add(y). 
 
*/

}
