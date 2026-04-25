fn main() {
   let products = ("wireless Keyboard", 450000, 12);
   let discounts = [5, 10, 15];

   println!("==== welcome ====");

   println!("Discount List");

   let (products_name, price, stok) = products;

   let mut total_price = stok * price;


   if BonusSticker(total_price){
    println!{"You got Sticker"}
   }

   for discounts_list in discounts
   {
    println!("Discount list {}", discounts_list);
    total_price = apply_discount(total_price, discounts_list);
    println!("{}", total_price)
   }
}

fn BonusSticker (total_price: i32) -> bool{
    total_price > 100000
}

fn apply_discount(total_price: i32, discounts_list: i32) -> i32
{
    total_price - (total_price * discounts_list) / 100
}
