/*
Lesson 1: Ownership and Dropping - It's All About Scope

Introduction:

Rust fundamentally emphasizes scope management, moving beyond traditional object-oriented (OO)
or functional paradigms, though it supports both. The real power of Rust comes from understanding
 and leveraging its ownership model, the concept of borrowing, and the lifetimes of variables.
 By focusing on when resources are dropped, who owns what, and the scope of lifetimes, you can
 unlock the full potential of Rust.

Vocabulary:

Own, Owned, Ownership: The concept of a variable owning a resource and being responsible for
                       cleaning it up.
Lifetime: A scope of time during which a variable is valid.
From: A trait that allows for type conversion from one type to another by consuming the
      original variable. Once converted, the original is no longer available.
Into: A reciprocal of From, used for consuming self to convert into another type. It also
      makes the original variable unavailable after conversion.
As: Used for cheap reference-to-reference conversions or explicit primitive type casting.
Move: A keyword used to transfer ownership of a variable to another variable.
Drop: A trait that allows for custom cleanup code to be run when a variable goes out of scope.


Suggestions: Emphasize more on the importance of ownership in Rust's safety guarantees. Include examples where students can see the impact of dropping resources (e.g., releasing file handles, or database connections).
 */


/////////////////////////////////////////////////////////
// lesson 1 ownership and dropping, it's all about the scope
/////////////////////////////////////////////////////////

use std::collections::VecDeque;
use crate::{consume, pass_thru};
use crate::example_data::build_heap_data;

pub(crate) fn examples() {
    // Scope and Ownership
    {
        let my_data1 = build_heap_data();
        println!("data1: {:?}", my_data1);
    }
    // Uncommenting the next line will cause a compilation error because my_data1 is out of scope
    // println!("data1: {:?}", my_data1);

    // Passing Ownership
    let my_data2 = build_heap_data();
    println!("data2: {:?}", my_data2);
    consume(my_data2);
    // Uncommenting the next line will cause a compilation error because my_data2 has been moved
    // println!("data: {:?}", my_data2);

    // Ownership transfer in a single line
    consume(build_heap_data());

    // Reclaiming ownership after passing
    let my_data3 = build_heap_data();
    let my_data3 = pass_thru(my_data3); // Lost ownership but regained it
    println!("data3: {:?}", my_data3);

    // Using 'From' trait for type conversion
    let my_data5 = build_heap_data();
    let both_ends: VecDeque<i32> = VecDeque::from(my_data5); // Lost ownership, it 'moved'
    // Uncommenting the next line will cause a compilation error because my_data5 has been moved
    // println!("data5: {:?}", my_data5);
    println!("both_ends (From): {:?}", both_ends);

    // Using 'Into' trait for type conversion
    let my_data4 = build_heap_data(); // Note: 'Into' is auto-generated based on 'From' implementations
    let both_ends: VecDeque<i32> = my_data4.into(); // Lost ownership, it 'moved'
    // Uncommenting the next line will cause a compilation error because my_data4 has been moved
    // println!("data4: {:?}", my_data4);
    println!("both_ends (Into): {:?}", both_ends);

    // Using 'into_iter' to consume and iterate over the collection
    let my_data6 = build_heap_data();
    for item in my_data6.into_iter() { // Lost ownership, it 'moved'
        println!("Iterated item: {:?}", item);
    }
    // Uncommenting the next line will cause a compilation error because my_data6 has been moved
    // println!("data6: {:?}", my_data6);

    // Demonstrating 'as' for type conversion (i32 to i64)
    let number: i32 = 42;
    let number_as_i64: i64 = number as i64;
    println!("number (i32): {:?}", number);
    println!("number_as_i64 (i64): {:?}", number_as_i64);

    // Demonstrating 'to_owned' method
    let my_data7 = build_heap_data();
    let my_data7_owned = my_data7.to_owned(); // 'Clones' the data, creating a new owned instance
    println!("my_data7 (original): {:?}", my_data7);
    println!("my_data7_owned (to_owned): {:?}", my_data7_owned);

    // Demonstrating 'Drop' trait
    {
        let my_data1 = build_heap_data();
        println!("data1: {:?}", my_data1);
        drop(my_data1); // Explicitly dropping, although it would happen at the end of scope anyway
        // Uncommenting the next line will cause a compilation error because my_data1 has been dropped
        // println!("data1: {:?}", my_data1);
    }
}
