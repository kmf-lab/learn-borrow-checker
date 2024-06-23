/******************************************/
/* Lesson 3: Borrow Checking in Rust      */
/******************************************/

/// Rust's borrow checker is what makes it stand out among modern programming languages.
/// By strictly enforcing ownership and borrowing rules, Rust guarantees memory safety
/// without a garbage collector. This means developers can write high-performance,
/// concurrent applications with confidence. In this lesson, we will explore the intricacies
/// of borrowing, mutable and immutable references, and the common pitfalls you may encounter.
/// Mastering these concepts is essential for writing efficient and safe Rust code.

/********************/
/*   Vocabulary     */
/********************/

/// Borrow, Borrowed, Borrowing: The concept of a scope temporarily using a resource
///                              without taking ownership.
/// Reference:                   A pointer to a value that does not own the value.
/// Mutable Reference:           A reference to a value that allows mutation.
/// Immutable Reference:         A reference to a value that does not allow mutation.
/// Box:                         A heap-allocated pointer type that provides ownership
///                              and moves values off the stack.

/////////////////////////////////////////////////////////
// Lesson 3: Borrow Checking in Rust
/////////////////////////////////////////////////////////

pub(crate) fn examples() {



    // 1) Immutable References
    println!(" --------------- lesson 3 example 1 ---------------");
    {
        let data = String::from("Hello, Rust!");
        let reference1 = &data;
        let reference2 = &data;
        println!("reference1: {}", reference1);
        println!("reference2: {}", reference2);
        // data can still be read here because it's just borrowed immutably
        println!("data: {}", data);
    }



    // 2) Mutable References
    println!(" --------------- lesson 3 example 2 ---------------");
    {
        let mut data = String::from("Hello");
        let reference = &mut data;

        // data cannot be used here directly as it's borrowed mutably
        // println!("data: {}", data); // Uncommenting this line will cause a compilation error

        reference.push_str(", Rust!");
        println!("reference: {}", reference); // Scope ends after last usage of the borrow

        println!("data: {}", data); // Data can be used after the mutable borrow ends
    }



    // 3) Problem with Mutable and Immutable References Together
    println!(" --------------- lesson 3 example 3 ---------------");
    {
        let mut data = String::from("Hello");
        data.push_str(" World");
        let reference1 = &data;
        let reference2 = &data;
        // data.push_str(" World"); // Uncommenting this line will cause a compilation error
        // let reference3 = &mut data; // Uncommenting this line will cause a compilation error
        // reference3.push_str(", Rust!");
        println!("reference1: {}", reference1);
        println!("reference2: {}", reference2);
        // println!("reference3: {}", reference3); // reference3 cannot coexist with reference1 and reference2
        let reference4 = &mut data; // Mutable borrow after immutable references go out of scope
        reference4.push_str("!");
        println!("reference4: {}", reference4);
    }



    // 4) Using Scopes for References
    println!(" --------------- lesson 3 example 4 ---------------");
    {
        let mut data = String::from("Hello");
        {
            let reference1 = &data;
            println!("reference1: {}", reference1);
        } // reference1 goes out of scope here
        let reference2 = &mut data;
        reference2.push_str(", Rust!");
        println!("reference2: {}", reference2);
    }



    // 5) Using Clone with Borrowing
    println!(" --------------- lesson 3 example 5 ---------------");
    #[derive(Debug, Clone)]
    struct MyCloneableStruct {
        data: String,
    }
    impl Drop for MyCloneableStruct {
        fn drop(&mut self) {
            println!("Dropping MyCloneableStruct with data: {}", self.data);
        }
    }
    {
        let original = MyCloneableStruct { data: String::from("Hello") };
        let borrowed = &original;
        let cloned = original.clone(); // original can still be used because it's cloned, not moved
        println!("original: {:?}", original);
        println!("borrowed: {:?}", borrowed);
        println!("cloned: {:?}", cloned);
        // Note that we have two drops here, one for the original and one for the cloned
    }



    // 6) Using Copy with Borrowing
    println!(" --------------- lesson 3 example 6 ---------------");
    #[derive(Debug, Copy, Clone)]
    struct MyCopyableStruct {
        my_number: i32,
    }
    // impl Drop for MyCopyableStruct { // This would cause a compile error, as a type cannot implement both Copy and Drop.
    //     fn drop(&mut self) {
    //         println!("Dropping MyCopyableStruct with data: {}", self.data);
    //     }
    // }
    {
        let original = MyCopyableStruct { my_number: 42 };
        let borrowed = &original;
        let copied = original; // original can still be used because it's copied, not moved
        println!("original: {:?}", original.my_number);
        println!("borrowed: {:?}", borrowed.my_number);
        println!("copied: {:?}", copied.my_number);
    }



    // 7) Using Box to Move Data to the Heap
    println!(" --------------- lesson 3 example 7 ---------------");
    {
        let data = Box::new(MyCopyableStruct { my_number: 42 });
        let reference = &data;
        println!("reference: {:?}", reference);
        // Box moves data to the heap, useful for large data structures
    }



    // 8) Using Box::leak to Extend Lifetime
    println!(" --------------- lesson 3 example 8 ---------------");
    {
        let s = {
            let data = Box::new(MyCloneableStruct { data: String::from("Hello") });
            let static_ref: &'static mut MyCloneableStruct = Box::leak(data);
            static_ref.data.push_str(" - Extended Lifetime");
            static_ref
        };
        println!("static_ref: {:?}", s);
        // No drop here as expected, since the data has an extended lifetime
    }



    // 9) Function Demonstrating Borrowing
    println!(" --------------- lesson 3 example 9 ---------------");
    fn print_data(data: &String) {
        println!("Data: {}", data);
    }
    let data = String::from("Hello, Rust!");
    print_data(&data); // Borrowing data immutably
    println!("After function call: {}", data);



    // 10) Function Demonstrating Mutable Borrowing
    println!(" --------------- lesson 3 example 10 ---------------");
    fn append_data(data: &mut String) {
        data.push_str(", Rust!");
    }
    let mut data = String::from("Hello");
    append_data(&mut data); // Borrowing data mutably
    println!("After function call: {}", data);



    // 11) Demonstrating Borrowing in Threads Problem
    println!(" --------------- lesson 3 example 11 ---------------");
    let data = String::from("Hello");
    let reference1 = &data; // Possible fix: .clone();
    // Uncommenting the next lines will cause a compilation error
    // use std::thread;
    // let handle = thread::spawn(move || {
    //     let reference2 = &data;
    //     println!("Thread reference: {}", reference2);
    // });
    // handle.join().unwrap();
    println!("Main thread reference: {}", reference1);
}


