struct Book{
    title: String
}

fn main() {

    let mut my_book_title = create_book();
    show_book (&my_book_title);

    edit_edition(&mut my_book_title);

    println!("Setelah revisi:");
    show_book(&my_book_title);
}

fn create_book() -> String {
    let my_book = Book { title: String::from("HDP JKW") };
    my_book.title
}

fn show_book(title: &String){
    println!("Book Title: {}", title);
}

fn edit_edition(title: &mut String) {
    title.push_str(" - Edisi 2026");
}