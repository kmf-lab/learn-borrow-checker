
mod lesson_1_scope;
mod lesson_2_drop_cc;
mod lesson_3_borrow;
mod lesson_4_bonus;

use rand::Rng;

fn main() {

    lesson_1_scope::examples();
    lesson_2_drop_cc::examples();
    lesson_3_borrow::examples();
    lesson_4_bonus::examples();

}


fn _pick_random(x: u32) -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pick_range_confirm() {
        let x = 42;

        let (min, max) = (0..1000)
            .map(|_| _pick_random(x))
            .fold((u32::MAX, u32::MIN), |(min, max), val| {
                (min.min(val), max.max(val))
            });

        println!("Min: {}, Max: {}", min, max);
        assert_eq!(1, min);
        assert_eq!(x, max);

    }


    #[test]
    fn test_pick_random() {
        //TODO: put the actual number of attendees here
        let attendee_count = 500;

        println!("The winners are:");
        let first_result = _pick_random(attendee_count);
        println!("First Selected Winner: {}", first_result);

        let second_result;
        loop {
            let temp = _pick_random(attendee_count);
            if temp != first_result {
                second_result = temp;
                break;
            }
        }
        println!("Second Selected Winner: {}", second_result);

        assert!(first_result >= 1 && first_result <= attendee_count, "Returned value is out of range");
        assert!(second_result >= 1 && second_result <= attendee_count, "Returned value is out of range");

    }
}