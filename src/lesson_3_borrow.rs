

/*
Lesson 3: Borrow Checking in Rust

Introduction:
In this lesson, we will focus on Rust's borrow checking mechanism. Borrowing allows a variable
 to temporarily use a resource without taking ownership, ensuring safe memory management and
  preventing data races. We will start with simple examples and progressively move to more
   complex ones, leveraging our understanding of scope, Copy, Clone, and Drop traits from previous lessons.

Vocabulary:
Borrow, Borrowed, Borrowing: The concept of a variable temporarily using a resource without taking ownership.
Reference: A pointer to a value that does not own the value.
Mutable Reference: A reference to a value that allows mutation.
Immutable Reference: A reference to a value that does not allow mutation.

Suggestions: Add more examples that show common borrowing mistakes and how to resolve them. Encourage students to experiment with mutable and immutable references to understand the compiler's error messages better.
 */

/////////////////////////////////////////////////////////
// lesson 3 borrow checking in Rust
/////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
struct MyCloneableStruct {
    data: String,
}

#[derive(Debug, Copy, Clone)]
struct MyCopyableStruct {
    data: i32,
}

impl Drop for MyCloneableStruct {
    fn drop(&mut self) {
        println!("Dropping MyCloneableStruct with data: {}", self.data);
    }
}

pub(crate) fn examples() {
    // Immutable References
    {
        let data = String::from("Hello, Rust!");
        let reference1 = &data;
        let reference2 = &data;
        println!("reference1: {}", reference1);
        println!("reference2: {}", reference2);
        // data can still be used here because it's just borrowed immutably
        println!("data: {}", data);
    }

    // Mutable References
    {
        let mut data = String::from("Hello");
        let reference = &mut data;
        reference.push_str(", Rust!");
        println!("reference: {}", reference);
        // data cannot be used here directly as it's borrowed mutably
    }

    // Problem with Mutable and Immutable References Together
    {
        let mut data = String::from("Hello");
        let reference1 = &data;
        let reference2 = &data;
        // let reference3 = &mut data; // Uncommenting this line will cause a compilation error
        println!("reference1: {}", reference1);
        println!("reference2: {}", reference2);
        // println!("reference3: {}", reference3); // reference3 cannot coexist with reference1 and reference2
    }

    // Demonstrating the Problem
    {
        let mut data = String::from("Hello");
        let reference1 = &data;
        // println!("Trying to modify data while immutable reference is held");
        // Uncommenting the next lines will cause a compilation error
        // let reference2 = &mut data;
        // reference2.push_str(", Rust!");
        // println!("reference2: {}", reference2);
        println!("reference1: {}", reference1); // reference1 still in scope
    }

    // Using Scopes for References
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

    // Using Clone with Borrowing
    {
        let original = MyCloneableStruct { data: String::from("Hello") };
        let borrowed = &original;
        let cloned = original.clone(); // original can still be used because it's cloned, not moved
        println!("original: {:?}", original);
        println!("borrowed: {:?}", borrowed);
        println!("cloned: {:?}", cloned);
    }

    // Using Copy with Borrowing
    {
        let original = MyCopyableStruct { data: 42 };
        let borrowed = &original;
        let copied = original; // original can still be used because it's copied, not moved
        println!("original: {:?}", original);
        println!("borrowed: {:?}", borrowed);
        println!("copied: {:?}", copied);
    }

    // Using Drop with Borrowing
    {
        let data = MyCloneableStruct { data: String::from("Hello, Rust!") };
        let reference = &data;
        println!("reference: {:?}", reference);
        // data will be dropped here, but only after reference goes out of scope
    }
}
