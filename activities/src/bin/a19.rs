// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;

fn main() {
  let mut furniture_store = HashMap::new();
  furniture_store.insert("Chairs",5);
  furniture_store.insert("Beds", 3);
  furniture_store.insert("Tables", 2);
  furniture_store.insert("Couches", 0);

  let mut total_stock = 0;

  for (product, stock) in furniture_store.iter() {
    if stock == &0 {
      println!("{:?}: Out of stock", product);
    }else {
      println!("{:?}: {:?} in stock", product,stock);
      total_stock += stock;
    }
  }
  println!("\nTotal items in stock: {}", total_stock);
}
