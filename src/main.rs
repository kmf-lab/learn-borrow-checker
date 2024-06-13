
mod lesson_1_scope;
mod lesson_2_drop_cc;
mod lesson_3_borrow;
mod lesson_4_bonus;


fn main() {

    lesson_1_scope::examples();
    lesson_2_drop_cc::examples();
    lesson_3_borrow::examples();
    lesson_4_bonus::examples();


    //drop is what we need to cover first

  // then drop trait
    //lesson 2 drop trait clone and copy

    //lesson 3then borrow.

     //lesson 4 self, avoid lifetimes


    /*
    ### Group 1: Ownership and Borrowing
    1. **`move`** - Used in closures to force the closure to take ownership of the captured variables.
        2. **`borrow`** - Implies taking a reference to data which can either be mutable or immutable.
        3. **`own`** - Relates to taking ownership of a value, which is a central feature of Rust's memory safety guarantees.

    ### Group 2: Conversion Traits
    1. **`from`** - A trait that allows for type conversion from one type to another.
        2. **`into`** - A reciprocal of `from`, used for consuming self to convert into another type.
    3. **`as`** - Used for cheap reference-to-reference conversions or explicit primitive type casting.
*/


}


/*
When a value goes out of scope, Rust checks if the type implements the Drop trait.
 This check is necessary because if the type has a custom destructor (implemented via the Drop trait), Rust needs to call this destructor before deallocating the memory.
 */

/*
The Rust compiler (LLVM) optimizes away unnecessary checks. If a type does not implement Drop, Rust will directly deallocate the memory without any additional runtime overhead.
 For types that do implement Drop, the destructor will be called before deallocation.
 */

/*
For compound types (like structs and enums), Rust generates code to drop each field in the correct order.
If any field implements Drop, the compiler ensures that the drop method is called for those fields.
 */

//  show the { } trick and use println! to show when our struct is dropped.






//tests
#[cfg(test)]

mod borrow_lessons {
    #[test]
    fn test_1() {
        let input = vec![1, 2, 3, 4, 5];
        let output = vec![1, 2, 3, 4, 5];
        assert_eq!(output, input);
    }

    #[test]
    fn test_2() {
        let input = vec![1, 2, 3, 4, 5];
        let output = vec![1, 2, 3, 4, 5];
        assert_eq!(output, input);
    }

}
