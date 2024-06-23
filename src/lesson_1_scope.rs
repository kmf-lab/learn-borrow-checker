        /***********************************************************/
        /* Lesson 1: Ownership and Dropping - It's All About Scope */
        /***********************************************************/

/// Rust fundamentally emphasizes scope management, moving beyond traditional
/// object-oriented (OO) or functional paradigms, though it supports both. The real
/// power of Rust comes from understanding and leveraging its ownership model, the
/// concept of borrowing, and the lifetimes of variables. By focusing on when resources
/// are dropped, who owns what, and the scope of lifetimes, you can unlock
/// the full potential of Rust. Using scopes is how Rust ensures memory safety guarantees.

        /********************/
        /*   Vocabulary     */
        /********************/

/// Own, Owned, Ownership: The concept of a scope owning a resource and being responsible
///                        for cleaning it up. The location of the variable holding the resource
///                        up to its last use determines the scope.
/// Lifetime:              A scope of time during which a variable is valid.
/// From:                  A method which allows for type conversion from one type to another
///                        by consuming the original variable. Once converted, the original
///                        is no longer available.
/// Into:                  A reciprocal of From, used for consuming self to convert into
///                        another type. The original variable is consumed and becomes
///                        unavailable after conversion.
/// As:                    Used for cheap reference-to-reference conversions or explicit
///                        primitive type casting.
/// Move:                  A keyword used to transfer ownership of a resource to another
///                        scope.
/// Drop:                  To clean up and free resources when a resource goes out of scope.

////////////////////////////////////////////////////////////////
// Lesson 1: Ownership and Dropping - It's All About the Scope
////////////////////////////////////////////////////////////////

use std::collections::VecDeque;

pub(crate) fn examples() {
    // 1) Scope and Ownership
    println!(" --------------- lesson 1 example 1 ---------------");
    {
        let my_data1 = vec![1, 2, 3, 4, 5];
        println!("data1: {:?}", my_data1);
    }
    // Uncommenting the next line will cause a compilation error because my_data1 is out of scope
    // println!("data1: {:?}", my_data1);



    // 2) Passing Ownership
    println!(" --------------- lesson 1 example 2 ---------------");
    let my_data2 = vec![1, 2, 3, 4, 5];
    println!("data2: {:?}", my_data2);
    consume(my_data2);
    // Uncommenting the next line will cause a compilation error because my_data2 has been moved
    // println!("data: {:?}", my_data2);

    // 2.1) Ownership transfer in a single line
    consume(vec![1, 2, 3, 4, 5]);



    // 3) Ownership transfer with move
    println!(" --------------- lesson 1 example 3 ---------------");
    let s1 = vec![1, 2, 3, 4, 5];
    let s2 = s1; // s1 is moved to s2.
    println!("s2: {:?}", s2);
    // println!("s1: {:?}", s1); // moved value: `s1`



    // 4) Reclaiming ownership after passing
    println!(" --------------- lesson 1 example 4 ---------------");
    let my_data4 = vec![1, 2, 3, 4, 5];
    let my_data4 = pass_thru(my_data4); // Lost ownership but regained it
    // the use of let with the same name is called shadowing
    println!("data4: {:?}", my_data4);



    // 5) Using 'From' trait for type conversion
    println!(" --------------- lesson 1 example 5 ---------------");
    let my_data5 = vec![1, 2, 3, 4, 5];
    let both_ends: VecDeque<i32> = VecDeque::from(my_data5); // Lost ownership, it 'moved'
    // Uncommenting the next line will cause a compilation error because my_data5 has been moved
    // println!("data5: {:?}", my_data5);
    println!("both_ends (From): {:?}", both_ends);



    // 6) Using 'Into' trait for type conversion
    println!(" --------------- lesson 1 example 6 ---------------");
    let my_data6 = vec![1, 2, 3, 4, 5]; // Note: 'Into' is auto-generated based on 'From' implementations
    let both_ends: VecDeque<i32> = my_data6.into(); // Lost ownership, it 'moved'
    // Uncommenting the next line will cause a compilation error because my_data6 has been moved
    // println!("data6: {:?}", my_data6);
    println!("both_ends (Into): {:?}", both_ends);



    // 7) Using `into_boxed_slice` to convert Vec to Box<[T]>
    println!(" --------------- lesson 1 example 7 ---------------");
    let my_data6_1 = vec![1, 2, 3, 4, 5];
    let boxed_slice: Box<[i32]> = my_data6_1.into_boxed_slice(); // Lost ownership
    // Uncommenting the next line will cause a compilation error because my_data6_1 has been moved
    // println!("data6_1: {:?}", my_data6_1);
    println!("boxed_slice: {:?}", boxed_slice);



    // 8) Using 'into_iter' to consume and iterate over the collection
    println!(" --------------- lesson 1 example 8 ---------------");
    let my_data7 = vec![1, 2, 3, 4, 5];
    for item in my_data7.into_iter() { // Lost ownership, it 'moved'
        println!("Iterated item: {:?}", item);
    }
    // Uncommenting the next line will cause a compilation error because my_data7 has been moved
    // println!("data7: {:?}", my_data7);



    // 9.1) Using from_utf8 example of String::from_utf8
    println!(" --------------- lesson 1 example 9.1 ---------------");
    let data8 = vec![104, 101, 108, 108, 111]; // ASCII values for "hello"
    let result = String::from_utf8(data8);
    match result {
        Ok(s) => println!("String from UTF-8: {}", s),
        Err(e) => println!("Error: {:?}", e),
    }
    // println!("{:?}", data8); // error since data8 moved

    // 9.2) Due to practical or historical reasons, we have exceptions to the rule.
    //      Methods like from_utf16 take a ref and do not consume the original data
    let unicode_values = vec![104, 101, 108, 108, 111]; // Unicode scalar values for "hello"
    let result = String::from_utf16(&unicode_values); // Note: references are covered in following lessons
    match result {
        Ok(s) => println!("String from UTF-16: {}", s),
        Err(e) => println!("Error: {:?}", e),
    }
    println!("{:?}", unicode_values); // this is ok, note we did not use the From trait.



    // 10) Demonstrating 'as' for type conversion (i32 to i64)
    println!(" --------------- lesson 1 example 10 ---------------");
    let number: i32 = 42;
    let number_as_i64: i64 = number as i64;
    println!("number (i32): {:?}", number);
    println!("number_as_i64 (i64): {:?}", number_as_i64);



    // 11) Demonstrating 'Drop' trait
    println!(" --------------- lesson 1 example 11 ---------------");
    {
        let my_data_b = vec![1, 2, 3, 4, 5];
        println!("my_data_b: {:?}", my_data_b);
        drop(my_data_b); // Explicitly dropping, although it would happen at the end of scope anyway
        // Uncommenting the next line will cause a compilation error because my_data_b has been dropped
        // println!("my_data_b: {:?}", my_data_b);
    }
}

// Helper method passing ownership through the call
pub fn pass_thru(data: Vec<i32>) -> Vec<i32> {
    data
}

// Helper method consuming the data
pub fn consume(_data: Vec<i32>) {}

