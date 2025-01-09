// This is a quiz for the following sections:
// - Variables
// - Functions
// - If
//
// Mary is buying apples. The price of an apple is calculated as follows:
// - An apple costs 2 rustbucks.
// - However, if Mary buys more than 40 apples, the price of each apple in the
// entire order is reduced to only 1 rustbuck!

// TODO: Write a function that calculates the price of an order of apples given
// the quantity bought.
fn calculate_price_of_apples(x: u16) -> u16 {
    let apple_price = 2;
    if x > 40 {
        return x * (apple_price - 1);
    } else {
        return x * 2;
    }
}

pub fn if_var_func_quiz() {
    let quantity = 35;
    let price = calculate_price_of_apples(quantity);
    println!("The price of {} apples is {} rustbucks", quantity, price);
}

// Don't change the tests!
#[cfg(test)]
mod pub tests {
    use super::*;

    #[test]
pub fn verify_test() {
        assert_eq!(calculate_price_of_apples(35), 70);
        assert_eq!(calculate_price_of_apples(40), 80);
        assert_eq!(calculate_price_of_apples(41), 41);
        assert_eq!(calculate_price_of_apples(65), 65);
    }
}
