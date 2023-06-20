//! 3.3 trait 定义类型共有行为
//!
//!

/**

```

    pub struct Book {
        name: String,
        price: f64,
        inventory: u32,
        author: String,
    }

    pub struct Cosmetic {
        name: String,
        price: f64,
        inventory: u32,
    }

    pub trait Record {
        fn set_price(&mut self, price: f64);
        fn set_inventory(&mut self, inventory: u32);
    }

    impl Record for Book {
        fn set_price(&mut self, price: f64) {
            self.price = price;
        }

        fn set_inventory(&mut self, inventory: u32) {
            self.inventory = inventory;
        }
    }

    impl Record for Cosmetic {
        fn set_price(&mut self, price: f64) {
            self.price = price;
        }

        fn set_inventory(&mut self, inventory: u32) {
            self.inventory = inventory;
        }
    }

    let mut book = Book {
        name: String::from("Book A"),
        price: 29.99,
        inventory: 10,
        author: String::from("Author X"),
    };

    let mut cosmetic = Cosmetic {
        name: String::from("Lipstick"),
        price: 9.99,
        inventory: 50,
    };

    book.set_price(39.99);
    book.set_inventory(5);

    cosmetic.set_price(14.99);
    cosmetic.set_inventory(20);

    println!(
        "Book: {} - Price: {} - Inventory: {} - Author: {}",
        book.name, book.price, book.inventory, book.author
    );
    println!(
        "Cosmetic: {} - Price: {} - Inventory: {}",
        cosmetic.name, cosmetic.price, cosmetic.inventory
    );

```
*/

pub fn trait_shared_behavior() {
    println!("");
}
