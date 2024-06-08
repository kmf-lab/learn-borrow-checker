pub fn build_heap_data() -> Vec<i32> {
    let mut my_data = Vec::new();
    my_data.push(1701);
    my_data.push(401);
    my_data.push(8675309);
    my_data
}

struct MyHeapData {
    text: String
}

impl Drop for MyHeapData {
    fn drop(&mut self) {
        println!("drop of MyHeapData");
    }
}


struct MyCopyData {
    count: u32
}

impl Drop for MyCopyData {
    fn drop(&mut self) {
        println!("drop of MyCopyData");
    }
}


pub fn build_copy_data() -> MyCopyData {
    MyCopyData { count: 42}
}