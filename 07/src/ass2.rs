#![allow(unused_variables)]
#![allow(dead_code)]

struct Fruit {
    apples: i32,
    bananas: i32,
}
struct FruitAndPrice {
    fruit: Fruit,
    price: i32
}
fn count_fruit(fruit: Fruit) -> Fruit {
    println!(
        "I've got {} apples and {} bananas",
        fruit.apples, fruit.bananas
    );
    fruit
}
fn price_fruit(fruit: Fruit) -> FruitAndPrice {
    let price = fruit.apples * 8 + fruit.bananas * 12;
    FruitAndPrice {fruit: fruit, price: price}
}
fn increase_fruit(mut fruit: Fruit) -> Fruit {
    fruit.apples *= 2;
    fruit.bananas *= 3;
    fruit
}
pub fn main() {
    let fruit = Fruit {
        apples: 10,
        bananas: 5,
    };
    let fruit = count_fruit(fruit);
    let fruit_and_price = price_fruit(fruit);
    println!("Original price: {}", fruit_and_price.price);
    let fruit = increase_fruit(fruit_and_price.fruit);
    let fruit_and_price = price_fruit(fruit);
    println!("I can make {} cents for more fruit", fruit_and_price.price);
    let fruit = increase_fruit(fruit_and_price.fruit);
    let fruit_and_price = price_fruit(fruit);
    println!("I can make {} cents for more fruit", fruit_and_price.price);
}
