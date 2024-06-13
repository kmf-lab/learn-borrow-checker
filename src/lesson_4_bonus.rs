
/*************************************************************/
/* Lesson 4: Advanced Borrowing and Reference Traits in Rust */
/*************************************************************/

/// In this lesson, we will delve into advanced concepts related to borrowing and references in Rust.
/// We'll explore traits such as AsRef, AsMut, Deref, and DerefMut, which provide powerful ways to
/// work with references in a flexible and idiomatic manner. We'll also introduce smart pointers like
/// Ref and RefMut from the RefCell type, enabling interior mutability and dynamic borrow checking.
/// Additionally, we'll cover the Self keyword and lifetime specifiers ('a), which are essential for
/// writing generic and reusable Rust code. Understanding these advanced borrowing and reference traits
/// will equip you with the skills to write more efficient, safe, and idiomatic Rust programs.

/********************/
/*   Vocabulary     */
/********************/

/// Self:     A keyword that refers to the current type or instance of a type.
/// 'a:       A lifetime specifier that indicates the lifetime of a reference.
/// AsRef:    A trait that allows for cheap reference-to-reference conversions.
/// AsMut:    A trait that allows for cheap mutable reference-to-mutable reference conversions.
/// Deref:    A trait that allows for implicit dereferencing of a value.
/// DerefMut: A trait that allows for implicit mutable dereferencing of a value.
/// Ref:      A smart pointer to a value that is borrowed.
/// RefMut:   A smart pointer to a value that is mutably borrowed.
///
///

/*
Suggestions: This lesson might be challenging for beginners. It’s important to ensure
that students are comfortable with the earlier concepts before diving into this lesson.
Consider making this an optional advanced topic for those who are ready.
 */

/////////////////////////////////////////////////////////
// lesson 4 advanced borrowing and reference traits in Rust
/////////////////////////////////////////////////////////

use std::cell::{RefCell, Ref, RefMut};
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
struct MyStruct<'a> {
    data: &'a str,
}

impl<'a> MyStruct<'a> {
    fn new(data: &'a str) -> Self {
        Self { data }
    }

    fn display(&self) {
        println!("Data: {}", self.data);
    }
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

pub(crate) fn examples() {
    // Self keyword and Lifetime specifier
    {
        let my_struct = MyStruct::new("Hello, Rust!");
        my_struct.display();
    }

    // AsRef trait
    {
        let s = String::from("Hello, Rust!");
        let s_ref: &str = s.as_ref();
        println!("s_ref: {}", s_ref);
    }

    // AsMut trait
    {
        let mut s = String::from("Hello, Rust!");
        let s_mut: &mut str = s.as_mut();
        s_mut.make_ascii_uppercase();
        println!("s_mut: {}", s_mut);
    }

    // Deref and DerefMut traits
    {
        let x = MyBox::new(String::from("Hello, Rust!"));
        println!("Dereferenced: {}", *x);

        let mut y = MyBox::new(String::from("Hello, Rust!"));
        y.push_str(" How are you?");
        println!("Mutably Dereferenced: {}", *y);
    }

    // Ref and RefMut smart pointers
    {
        let data = RefCell::new(String::from("Hello, Rust!"));

        // Borrow as immutable
        { //comment this bracket and see we have no compiler error
            let borrowed: Ref<String> = data.borrow();
            println!("Borrowed: {}", borrowed);
        } //comment this bracket and see we have no compiler error
        // Borrow as mutable ONLY works because our borrow above was dropped.
        // NOTE this one gets past the compiler because we used RefCell which is checked at runtime.
        let mut borrowed_mut: RefMut<String> = data.borrow_mut();
        borrowed_mut.push_str(" How are you?");
        println!("Mutably Borrowed: {}", borrowed_mut);
    }
}