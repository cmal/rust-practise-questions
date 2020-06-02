use std::io;
use std::io::prelude::*; 
use std::vec::Vec;
use std::collections::HashMap;
// use ncurses::{addstr, stdscr, initscr, raw, getch, winstr, endwin, clear};

// Let us work on the menu of a library. Create a structure containing book information like accession number, name of author, book title and flag to know whether book is issued or not. Create a menu in which the following can be done.
// 1 - Display book information
// 2 - Add a new book
// 3 - Display all the books in the library of a particular author
// 4 - Display the number of books of a particular title
// 5 - Display the total number of books in the library
// 6 - Issue a book
// (If we issue a book, then its number gets decreased by 1 and if we add a book, its number gets increased by 1)

#[derive(Debug)]
struct Book {
    accession_number: String,
    author: String,
    title: String,
    is_issued: bool,
}

impl Book {
    fn display(&self) {
        // addstr(&format!("\nAccession Number: {}\n", self.accession_number));
        // addstr(&format!("Title: {}\n", self.title));
        // addstr(&format!("Author: {}\n", self.author));
        // addstr(&format!("Issued?: {}\n", self.is_issued));
        println!("Accession Number: {}", self.accession_number);
        println!("Title: {}", self.title);
        println!("Author: {}", self.author);
        println!("Issued?: {}", self.is_issued);
    }

    fn issue(&mut self) {
        self.is_issued = true;
    }
}

struct Menu {
    books: Vec<Book>,
}

impl Menu {
    fn new() -> Self {
        Self {
            books: Vec::new(),
        }
    }
    
    fn list(&self) {
        for book in self.books.iter() {
            book.display();
        }
    }

    fn search(&self, title: String) -> Vec<&Book> {
        self.books.iter().filter(|book| book.title == title).collect()
    }

    fn add(&mut self, book: Book) {
        self.books.push(book);
    }

    fn at(&self, index: usize) -> &Book {
        &self.books[index as usize]
    }

    fn mutable_at(&mut self, index: usize) -> &mut Book {
        &mut self.books[index as usize]
    }

    fn count_not_issued(&self) -> usize {
        self.books.iter().filter(|book| !book.is_issued).count()
    }

    fn count(&self) -> usize {
        self.books.len()
    }
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok().expect("Failed to read line");
    input.trim().to_string()
    // winstr(stdscr(), &mut input);
    // input
}

fn read_char() -> char {
    // std::io::stdin()
    //     .bytes() 
    //     .next()
    //     .and_then(|result| result.ok())
    //     .map(|byte| byte as char)
    //     .unwrap()
    read_line().chars().next().unwrap()
    // getch() as u8 as char
}

fn read_usize() -> usize {
    read_line().trim().parse::<usize>().ok().expect("Failed to read usize")
    // read_line().parse::<usize>().unwrap()
}

fn flush() {
    io::stdout().flush().unwrap();
}

fn process_command(menu: &mut Menu, commands: &HashMap<char, &str>) {
    loop {
        // addstr(&format!("Enter a command(? for help, q for quit): "));
        print!("Enter a command(? for help, q for quit): ");
        flush();
        let c = read_char();
        // clear();
        match c {
            'q' => {
                // endwin();
                break
            },
            'a' => add_book(menu),
            'l' => menu.list(),
            'd' => display_a_book(menu),
            'f' => search_book(menu),
            'i' => issue_a_book(menu),
            'c' => show_count(menu),
            '?' => show_help(commands),
            _ => invalid_command()
        }
    }
}

fn invalid_command() {
    // addstr(&format!("Invalid command"));
    println!("Invalid command");
}

fn show_help(commands: &HashMap<char, &str>) {
    // addstr(&format!("Valid commands:\n"));
    println!("Valid commands:");
    for (k, v) in commands.iter() {
        // addstr(&format!("{} -> {}\n", k, v));
        println!("{} -> {}", k, v);
    }
}

fn add_book(menu: &mut Menu) {
    // println!(">>> Add a book:");
    // addstr(&"Accession number: ");
    print!("Accession number: ");
    flush();
    let accession_number = read_line();
    // addstr(&"Author: ");
    print!("Author: ");
    flush();
    let author = read_line();
    // addstr(&"Title: ");
    print!("Title: ");
    flush();
    let title = read_line();
    let book = Book {
        accession_number: accession_number,
        author: author,
        title: title,
        is_issued: false
    };
    // addstr(&format!("\nAdd book: {:?}\n", &book));
    println!("Add book: {:?}", &book);
    menu.add(book);
}

fn display_a_book(menu: &Menu) {
    // addstr(&"Display a book at index: ");
    print!("Display a book at index: ");
    flush();
    let index = read_usize();
    if menu.count() - 1 < index {
        println!("out of index");
    } else {
        menu.at(index).display();
    }
}

fn issue_a_book(menu: &mut Menu) {
    // addstr(&format!("Issue a book at index: "));
    print!("Issue a book at index: ");
    flush();
    let index = read_usize();
    if menu.count() - 1 < index {
        println!("out of index");
    } else if menu.mutable_at(index).is_issued {
        // addstr(&format!("\nCannot issue this book. Already taken.\n"));
        println!("Cannot issue this book. Already taken.");
    } else {
        menu.mutable_at(index).issue();
        // addstr(&format!("\nDone.\n"));
        println!("Done.");
    }
}

fn show_count(menu: &Menu) {
    // addstr(&format!("Total book count: {}.\n", menu.count_not_issued()));
    println!("Total book count: {}.", menu.count_not_issued());
}

fn search_book(menu: &Menu) {
    // addstr(&format!("Title of the book you need: "));
    print!("Title of the book you need: ");
    flush();
    let title = read_line();
    let books: Vec<&Book> = menu.search(title.clone());
    if books.len() > 0 {
        // addstr(&format!("Accession Numbers of book with title \"{}\": \n", title));
        // addstr(&"\n");
        println!("Accession Numbers of book with title \"{}\": ", title);
        for book in books.iter() {
            // addstr(&format!("{}\n", book.accession_number));
            println!("{}", book.accession_number);
        }
    } else {
        // addstr(&format!("Book of title \"{}\" NOT FOUND.\n", title));
        println!("Book of title \"{}\" NOT FOUND.\n", title);
    }
}

fn main() {
    // initscr();
    // raw();
    let book1 = Book {
        accession_number: "X04SCV32".to_string(),
        author: "Jack London".to_string(),
        title: "White Fang".to_string(),
        is_issued: true
    };

    let book2 = Book {
        accession_number: "4Z52HV09".to_string(),
        author: "Charles Dickens".to_string(),
        title: "David Copperfield".to_string(),
        is_issued: false
    };

    let mut menu = Menu::new();
    menu.add(book1);
    menu.add(book2);

    let mut commands: HashMap<char, &str> = HashMap::new();

    commands.insert('q', "Quit");
    commands.insert('a', "Add a book");
    commands.insert('l', "List books");
    commands.insert('d', "Display book information");
    commands.insert('f', "Find a book accession number by title");
    commands.insert('i', "Issue a book"); // 借书
    commands.insert('c', "Show total number of books");

    process_command(&mut menu, &commands);
}
