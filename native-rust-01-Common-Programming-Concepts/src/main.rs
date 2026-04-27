fn main() {
    let price_list = [15000, 20000, 50000];

    let special_item = ("Mechanical Keyboard", 450000, 0);

    println!("--- Welcome to the store ---");

    println!("Catalog Price List:");
    for price in price_list {
        if price > 25000
        {
            println!("expensive price");
        }
        println!("Rp {price}");
    }

    let (name, price, stock) = special_item;
    println!("\nSpecial Item: {name}");
    println!("Price: Rp {price}, Stock: {stock}");

   if is_stock_available(stock)
   {
    println!("Stock available");
   }
   else
   {
    println!("Stock out of stock");
   }

    let discounted_price = calculate_discount(price, 10); 
    println!("Price after 10% discount: Rp {discounted_price}");
}

fn calculate_discount(price: i32, percentage: i32) -> i32 {

    price - (price * percentage / 100)
}

fn is_stock_available(stock: i32) -> bool
{
   stock > 0
}