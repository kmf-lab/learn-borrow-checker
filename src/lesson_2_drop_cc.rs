
/*
Lesson 2: Drop, Clone, and Copy Traits

Introduction:

In this lesson, we will delve deeper into Rust's memory management features by exploring the
 Drop, Clone, and Copy traits. We will also touch on the mut keyword and how to define and use
 structs. Understanding these concepts is crucial for managing resource lifetimes and ensuring memory safety in Rust.

Vocabulary:
Drop: A trait that allows for custom cleanup code to be run when a variable goes out of scope,
      similar to Java's finally or dispose methods.
Clone: A trait that allows for copying a value, providing deep copies when necessary.
Copy: A trait that allows for copying a value, suitable for types that are simple and can be copied bitwise.
Mut: A keyword used to indicate mutable access to a variable.
Struct: A keyword used to define a structure.

Key Point:
    If a type implements the Copy trait, then assignments and function calls involving that type will
    copy the value rather than moving ownership. This means that after an assignment or function call,
    both the original and the new variable can be used independently.

    Detailed Explanation
    Types That Can Be Copied:

    Simple scalar values (integers, floating-point numbers, booleans, characters).
    Types composed entirely of Copy types, like tuples or arrays of Copy types.
    Types That Cannot Be Copied:

    Types that manage heap memory or other resources usually cannot be copied because copying would
     involve duplicating the resource management, which is more complex than a simple bitwise copy.
      Examples include:
    String
    Vec<T>
    Types containing non-Copy types


Suggestions: Highlight potential pitfalls of using Copy and Clone inappropriately (e.g., performance implications of unnecessary cloning). Include interactive examples where students can toggle between Copy and non-Copy fields to see the compiler's behavior.

 */

/////////////////////////////////////////////////////////
// lesson 2 drop, clone, and copy traits
/////////////////////////////////////////////////////////

// Import necessary traits
use std::fmt::Debug;

#[derive(Debug)]
struct SimpleStruct {
    data: String,
}

#[derive(Debug)]
struct MyStruct {
    data: String,
}

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Dropping MyStruct with data: {}", self.data);
    }
}

#[derive(Debug, Clone)]
struct MyCloneableStruct {
    data: String,
}

#[derive(Debug, Copy, Clone)]
struct MyCopyableStruct {
    data: i32,
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

// Example function to demonstrate usage
pub(crate) fn examples() {
    // Immutability
    {
        let my_struct = SimpleStruct { data: String::from("Immutable data") };
        println!("Immutable struct: {:?}", my_struct);
        // my_struct.data.push_str(" - this would cause an error"); // Uncommenting this line would cause a compilation error
    }

    // Mutability
    {
        let mut my_struct = SimpleStruct { data: String::from("Mutable data") };
        println!("Before mutation: {:?}", my_struct);
        my_struct.data.push_str(" has been mutated!");
        println!("After mutation: {:?}", my_struct);
    }

    // Drop trait
    {
        let my_struct = MyStruct { data: String::from("Hello, Rust!") };
        println!("Created: {:?}", my_struct);
        // my_struct goes out of scope here and Drop will be called
    }

    // Clone trait
    {
        let original = MyCloneableStruct { data: String::from("Clone me!") };
        let cloned = original.clone(); // Creates a deep copy
        println!("Original: {:?}", original);
        println!("Cloned: {:?}", cloned);
    }

    // Copy trait
    {
        let original = MyCopyableStruct { data: 42 };
        let copied = original; // Copies the value bitwise
        println!("Original: {:?}", original);
        println!("Copied: {:?}", copied);
    }

    // Demonstrate what happens if we add a non-Copyable field
    #[derive(Debug, Clone)]
    struct NonCopyableStruct {
        data: String,
    }

    // Uncommenting the following line will cause a compile-time error
    // #[derive(Debug, Copy, Clone)]
    // struct InvalidCopyStruct {
    //     non_copyable_data: NonCopyableStruct,
    // }

    // Combining traits with struct
    {
        let point1 = Point { x: 1, y: 2 };
        let point2 = point1; // Copy trait allows for bitwise copy
        println!("Point1: {:?}", point1);
        println!("Point2: {:?}", point2);

        let mut point3 = Point { x: 3, y: 4 };
        println!("Point3 before mutation: {:?}", point3);
        point3.x = 5; // Mut keyword allows for mutation
        println!("Point3 after mutation: {:?}", point3);
    }
}
