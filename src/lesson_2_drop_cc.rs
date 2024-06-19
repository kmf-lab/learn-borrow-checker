
        /******************************************/
        /* Lesson 2: Drop, Clone, and Copy Traits */
        /******************************************/

/// In this lesson, we delve deeper into Rust's memory management features by exploring
/// the Drop, Clone, and Copy traits. These traits are fundamental for understanding
/// how Rust handles resource cleanup, duplication, and efficient value transfers.
/// We'll also touch on the mut keyword and the definition and usage of structs.
/// Grasping these concepts is essential for managing resource lifetimes and ensuring
/// memory safety, which are cornerstones of Rust's design philosophy

        /********************/
        /*   Vocabulary     */
        /********************/

/// Drop:    A trait that allows for custom cleanup code to be run when a variable
///          goes out of scope, similar to Java's finally or dispose methods.
/// Clone:   A trait that allows for copying a value, providing deep copies when necessary.
/// Copy:    A trait that allows for copying a value, suitable for types that are simple
///          and can be copied bitwise.
/// Mut:     A keyword used to indicate mutable access to a variable.
/// Struct:  A keyword used to define a structure.

/*

//////////////////////////////////////////////////////////////////////
What derived creates for us:
//////////////////////////////////////////////////////////////////////
        // When we do this: #[derive(Debug, Clone, Copy)]
        struct Point {
            x: i32,
            y: i32,
        }

        // We get this:
        impl Debug for Point {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "Point {{ x: {}, y: {} }}", self.x, self.y)
            }
        }
        impl Clone for Point {
            fn clone(&self) -> Point {
                Point {
                    x: self.x,
                    y: self.y,
                }
            }
        }
        impl Copy for Point {}

 */

/////////////////////////////////////////////////////////
// lesson 2 drop, clone, and copy traits
/////////////////////////////////////////////////////////
use std::fmt::Debug;
use std::io::Write;

// Example function to demonstrate usage
pub(crate) fn examples() {

    #[derive(Debug)]
    struct SimpleStruct {
        data: String,
    }



    // 1) Immutability by default
    println!(" --------------- lesson 2 example 1 ---------------");
    {
        let my_struct = SimpleStruct { data: String::from("Immutable data") };
        println!("Immutable struct: {:?}", my_struct);
        // my_struct.data.push_str(" - this would cause an error"); // Uncommenting this line would cause a compilation error
    }



    // 2) Mutability
    println!(" --------------- lesson 2 example 2 ---------------");
    {
        let mut my_struct = SimpleStruct { data: String::from("Mutable data") };
        println!("Before mutation: {:?}", my_struct);
        my_struct.data.push_str(" has been mutated!");
        println!("After mutation: {:?}", my_struct);
    }


    /*
    When a value goes out of scope, Rust checks if the type implements the Drop trait.
     This check is necessary because if the type has a custom destructor (implemented via the Drop trait), Rust needs to call this destructor before deallocating the memory.

    The Rust compiler (LLVM) optimizes away unnecessary checks. If a type does not implement Drop, Rust will directly deallocate the memory without any additional runtime overhead.
     For types that do implement Drop, the destructor will be called before deallocation.

    For compound types (like structs and enums), Rust generates code to drop each field in the correct order.
    If any field implements Drop, the compiler ensures that the drop method is called for those fields.
     */

    // 3) Drop trait
    println!(" --------------- lesson 2 example 3 ---------------");
    #[derive(Debug)]
    struct MyStruct {
        data: String,
    }
    impl Drop for MyStruct {
        fn drop(&mut self) {
            println!("Dropping MyStruct with data: {}", self.data);
        }
    }
    {
        let my_struct = MyStruct { data: String::from("Hello, Rust!") };
        println!("Created: {:?}", my_struct);
        // my_struct goes out of scope here and Drop will be called
    }

    // 3.1) Domonstrating file close when it leaves scope
    {
        let file = std::fs::File::open("src/lesson_1_scope.tmp");
        // write a byte to the file
        match file {
            Ok(mut f) => {
                f.write_all(b"Hello, Rust!").expect("Unable to write to file");
            },
            Err(e) => {
                eprintln!("Error opening file: {:?}", e);
            }
        } // File is closed here
    }

/*
    On the Copy Trait:

    If a type implements the Copy trait, then assignments and function calls involving
    that type will copy the value rather than moving ownership. This means that after
    an assignment or function call, both the original and the new variable can be used
    independently. This is the pass-by-value behavior you may already be familiar with
    from other programming languages.

        Detailed Explanation

    Types That Can Be Copied:

        * Simple scalar values (integers, floating-point numbers, booleans, characters)
        * Slice references (e.g., &T)
        * Types composed entirely of Copy types, like tuples or arrays of Copy types.

        Types That Cannot Be Copied:

    Types that manage heap memory or other resources usually cannot be copied
    because copying would involve duplicating the resource management, which
    is more complex than a simple bitwise copy.

        Examples include:
        * String
        * Vec<T>
        * Types containing non-Copy types

    The Copy trait requires that the type also implements the Clone trait:

    The reason Copy requires Clone is that Copy is a specific kind of Clone.
    When a type implements Copy, it can be duplicated (cloned) by a simple
    bitwise copy. Other, more complex types that implement Clone may require
    memory allocations or other operations that are not suitable for Copy.
*/


    // 4) Copy trait
    println!(" --------------- lesson 2 example 4 ---------------");
    #[derive(Debug, Copy, Clone)]
    struct MyCopyableStruct {
        data: i32,
    }
    {
        let original = MyCopyableStruct { data: 42 };
        let copied = original; // Copies the value bitwise
        println!("Original: {:?}", original.data);
        println!("Copied: {:?}", copied.data);
    }
    {
        let s1 = 42;
        let s2 = s1; // s1 is copied to s2
        println!("{}", s2);
        println!("{}", s1);
    }


    // 5) Clone trait
    println!(" --------------- lesson 2 example 5 ---------------");
    #[derive(Debug, Clone)]
    struct MyCloneableStruct {
        data: String,
    }
    {
        let original = MyCloneableStruct { data: String::from("Clone me!") };
        let cloned = original.clone(); // Creates a deep copy
        println!("Original: {:?}", original.data);
        println!("Cloned: {:?}", cloned.data);
    }
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2 vs copy in this case.
    println!("{}", s2);
    //println!("{}", s1); // This line would cause a compile-time error because s1 is no longer valid


    // 6) Demonstrate what happens if we add a non-Copyable field
    println!(" --------------- lesson 2 example 6 ---------------");
    // Uncommenting the following line will cause a compile-time error
    // #[derive(Debug, Copy, Clone)]
    // struct InvalidCopyStruct {
    //     non_copyable_data: MyCloneableStruct,
    // }


    // 7) Combining traits with struct
    println!(" --------------- lesson 2 example 7 ---------------");
    #[derive(Debug, Clone, Copy)]
    struct Point {
        x: i32,
        y: i32,
    }
    {
        let point1 = Point { x: 1, y: 2 };
        let point2 = point1; // Copy trait allows for bitwise copy
        println!("Point1: {} {}", point1.x, point1.y);
        println!("Point2: {} {}", point2.x, point2.y);

        let mut point3 = Point { x: 3, y: 4 };
        println!("Point3 before mutation: {:?}", point3);
        point3.x = 5; // Mut keyword allows for mutation
        println!("Point3 after mutation: {:?}", point3);
    }

   // 8) Performance Implications of Cloning Large Data - on to borrow.
   println!(" --------------- lesson 2 example 8 ---------------");

    use std::time::Instant;

    #[derive(Debug, Clone)]
    struct LargeStruct {
        data: Vec<i32>,
    }
    {
        let large_data = LargeStruct { data: vec![0; 1_000_000] };

        // Measure the time taken to clone the large struct
        let start = Instant::now();
        let cloned_data = large_data.clone();
        let duration = start.elapsed();

        println!("Time taken to clone: {:?}", duration);

        // Demonstrate that cloned_data is a deep copy
        println!("Original data length: {}", large_data.data.len());
        println!("Cloned data length: {}", cloned_data.data.len());
    }
}


