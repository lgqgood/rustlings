// Define the struct with lifetime parameters
struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    // Create an instance of the Book struct with references
    let book = Book {
        author: "George Orwell",
        title: "1984",
    };

    // Print the book's title and author
    println!("{} by {}", book.title, book.author);
}