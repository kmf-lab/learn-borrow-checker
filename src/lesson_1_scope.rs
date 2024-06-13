                /***********************************************************/
                /* Lesson 1: Ownership and Dropping - It's All About Scope */
                /***********************************************************/

/// Rust fundamentally emphasizes scope management, moving beyond traditional
/// object-oriented (OO) or functional paradigms, though it supports both. The real
/// power of Rust comes from understanding and leveraging its ownership model, the
/// concept of borrowing, and the lifetimes of variables. By focusing on when resources
/// are dropped, who owns what, and the scope of lifetimes, you can unlock
/// the full potential of Rust.  Using scopes is how Rust ensures memory safety guarantees.

                    /********************/
                    /*   Vocabulary     */
                    /********************/

/// Own,Owned,Ownership: The concept of a variable owning a resource and being responsible
///                      for cleaning it up.
/// Lifetime:            A scope of time during which a variable is valid.
/// From:                A trait which allows for type conversion from one type to another
///                      by consuming the original variable. Once converted, the original
///                      is no longer available.
/// Into:                A reciprocal of From, used for consuming self to convert into
///                      another type. The original variable is consumed and becomes
///                      unavailable after conversion.
/// As:                  Used for cheap reference-to-reference conversions or explicit
///                      primitive type casting.
/// Move:                A keyword used to transfer ownership of a variable to another
///                      variable.
/// Drop:                A trait which allows for custom cleanup code to be run when a
///                      variable goes out of scope.
///

/////////////////////////////////////////////////////////
// lesson 1 ownership and dropping, it's all about the scope
/////////////////////////////////////////////////////////
use std::collections::VecDeque;


pub(crate) fn examples() {
    // 1) Scope and Ownership
    {
        let my_data1 = vec![1, 2, 3, 4, 5];
        println!("data1: {:?}", my_data1);
    }
    // Uncommenting the next line will cause a compilation error because my_data1 is out of scope
    // println!("data1: {:?}", my_data1);

    // 2) Passing Ownership
    let my_data2 = vec![1, 2, 3, 4, 5];
    println!("data2: {:?}", my_data2);
    consume(my_data2);
    // Uncommenting the next line will cause a compilation error because my_data2 has been moved
    //println!("data: {:?}", my_data2);

    // 2.5) Ownership transfer in a single line
    consume(vec![1, 2, 3, 4, 5]);

    // 3) Reclaiming ownership after passing
    let my_data3 = vec![1, 2, 3, 4, 5];
    let my_data3 = pass_thru(my_data3); // Lost ownership but regained it
    //the use of let with the same name is called shadowing
    println!("data3: {:?}", my_data3);

    // 4) Using 'From' trait for type conversion
    let my_data4 = vec![1, 2, 3, 4, 5];
    let both_ends: VecDeque<i32> = VecDeque::from(my_data4); // Lost ownership, it 'moved'
    // Uncommenting the next line will cause a compilation error because my_data4 has been moved
    //println!("data4: {:?}", my_data4);
    println!("both_ends (From): {:?}", both_ends);

    // 5) Using 'Into' trait for type conversion
    let my_data5 = vec![1, 2, 3, 4, 5]; // Note: 'Into' is auto-generated based on 'From' implementations
    let both_ends: VecDeque<i32> = my_data5.into(); // Lost ownership, it 'moved'
    // Uncommenting the next line will cause a compilation error because my_data5 has been moved
    // println!("data4: {:?}", my_data5);
    println!("both_ends (Into): {:?}", both_ends);

    // 6) Using 'into_iter' to consume and iterate over the collection
    let my_data6 = vec![1, 2, 3, 4, 5];
    for item in my_data6.into_iter() { // Lost ownership, it 'moved'
        println!("Iterated item: {:?}", item);
    }
    // Uncommenting the next line will cause a compilation error because my_data6 has been moved
    //println!("data6: {:?}", my_data6);

    // 7) Demonstrating 'as' for type conversion (i32 to i64)
    let number: i32 = 42;
    let number_as_i64: i64 = number as i64;
    println!("number (i32): {:?}", number);
    println!("number_as_i64 (i64): {:?}", number_as_i64);

    // 8) Demonstrating 'to_owned' method
    let my_data7 = vec![1, 2, 3, 4, 5];
    let my_data7_owned = my_data7.to_owned(); // 'Clones' the data, creating a new owned instance
    println!("my_data7 (original): {:?}", my_data7);
    println!("my_data7_owned (to_owned): {:?}", my_data7_owned);

    // 9) Demonstrating 'Drop' trait
    {
        let my_data1 = vec![1, 2, 3, 4, 5];
        println!("data1: {:?}", my_data1);
        drop(my_data1); // Explicitly dropping, although it would happen at the end of scope anyway
        // Uncommenting the next line will cause a compilation error because my_data1 has been dropped
        // println!("data1: {:?}", my_data1);
    }
}


//helper method passing ownership thru the call
pub fn pass_thru(data:Vec<i32>) -> Vec<i32>{
    data
}

//helper method consuming the data
pub fn consume(_data:Vec<i32>) {
}
