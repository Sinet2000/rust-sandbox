mod models;
mod utils;

// or if there is a nested folder, lets say models, mod models and then use as `use models::category::{CategoryId}`

use std::io;
// to gerneate guid cargo add uuid --features v4
use uuid::Uuid;
// Datetime cargo add chrono
use chrono::{DateTime, Utc};

use std::collections::HashMap;
use std::io::Write;

use models::category::{Category, CategoryId};
use models::inventory_item::InventoryItem;
use utils::input::{pause, read_f64, read_string, read_u32};

fn list_categories(categories: &[Category]) {
    // or categories: &Vec<Category>
    println!("======================================");
    println!("       CATEGORIES");
    println!("======================================");
    for category in categories {
        println!("ID: {}, Name: {}", category.id.0, category.name);
    }
    println!("======================================");
}

fn list_products(
    products: &[InventoryItem],
    category_lookup: &HashMap<CategoryId, &Category>,
    category_id: Option<CategoryId>,
) {
    // for product in products {
    //     product.print();
    //     if let Some(category) = categories.iter().find(|c| c.id == product.category_id) {
    //         println!("Category Name: {}", category.name);
    //     } else {
    //         println!("Category Name: Unknown");
    //     }
    // }

    // for product in &products {
    //     let category = categories
    //         .iter()
    //         .find(|c| c.id == product.category_id)
    //         .expect("Category not found");

    //     println!(
    //         "{} | {} | {}",
    //         product.name,
    //         category.id.0,
    //         category.name
    //     );
    // }

    // Idiomatic
    if products.is_empty() {
        println!("No products available.");
        return;
    }

    println!();
    println!("{:-<120}", "");
    println!(
        "{:<36} {:<20} {:>8} {:>10} {:<15} {:<20}",
        "ID", "Name", "Qty", "Price", "Category", "Created"
    );
    println!("{:-<120}", "");

    for product in products
        .iter()
        .filter(|p| category_id.is_none_or(|id| p.category_id == id))
    {
        let category_name = category_lookup
            .get(&product.category_id)
            .map(|c| c.name.as_str())
            .unwrap_or("Unknown");

        println!(
            "{:<36} {:<20} {:>8} {:>10.2} {:<15} {:<20}",
            product.id(),
            product.name(),
            product.quantity(),
            product.price(),
            category_name,
            product.date_created().format("%Y-%m-%d %H:%M:%S"),
        );
    }

    println!("{:-<120}", "");
}

fn list_products_by_category(
    products: &[InventoryItem],
    categories: &[Category],
    category_lookup: &HashMap<CategoryId, &Category>,
) {
    println!("\nAvailable categories:");

    for category in categories {
        println!("{} - {}", category.id.0, category.name);
    }

    let category_id = loop {
        let id = CategoryId(read_u32("Category ID: "));

        if category_lookup.contains_key(&id) {
            break id;
        }

        println!("Category not found.");
    };

    list_products(products, category_lookup, Some(category_id));
}

fn create_product(
    products: &mut Vec<InventoryItem>,
    categories: &[Category],
    category_lookup: &HashMap<CategoryId, &Category>,
) {
    println!("\n=== Create Product ===");

    let name = read_string("Name: ");
    let quantity = read_u32("Quantity: ");
    let price = read_f64("Price (€): ");

    println!("\nAvailable categories:");

    for category in categories {
        println!("{} - {}", category.id.0, category.name);
    }

    let category_id = loop {
        let id = CategoryId(read_u32("Category ID: "));

        if category_lookup.contains_key(&id) {
            break id;
        }

        println!("Category not found.");
    };

    products.push(InventoryItem::create(name, quantity, price, category_id));

    println!("\n✅ Product created successfully.");
}

fn update_product_price(products: &mut [InventoryItem]) {
    println!("\nUpdating product price...");
}

fn update_product_quantity(products: &mut [InventoryItem]) {
    println!("\nUpdating product quantity...");
}

fn remove_product(products: &mut [InventoryItem]) {
    println!("\nRemoving product...");
}

fn print_main_menu() {
    println!();
    println!("======================================");
    println!("       INVENTORY MANAGEMENT");
    println!("======================================");
    println!("1. List categories");
    println!("2. List products");
    println!("3. List products by category");
    println!("4. Create new product");
    println!("5. Update product");
    println!("6. Remove product");
    println!("0. Exit");
    println!("======================================");
}

fn update_product_menu(products: &mut [InventoryItem]) {
    println!();
    println!("------------------------------");
    println!("       UPDATE PRODUCT");
    println!("------------------------------");
    println!("1. Update price");
    println!("2. Update quantity");
    println!("0. Back");
    println!("------------------------------");

    let action = read_u32("Select an action: ");

    match action {
        1 => update_product_price(products),
        2 => update_product_quantity(products),
        0 => {}
        _ => println!("Invalid action."),
    }
}

fn main() {
    // let mut categories = HashMap::new();
    // categories.insert(1, "Electronics");
    // categories.insert(2, "Peripherals");
    // categories.insert(3, "Furniture");
    let mut categories = vec![
        Category {
            id: CategoryId(1),
            name: "Electronics".to_string(),
        },
        Category {
            id: CategoryId(2),
            name: "Peripherals".to_string(),
        },
        Category {
            id: CategoryId(3),
            name: "Furniture".to_string(),
        },
    ];

    categories.push(Category {
        id: CategoryId(4),
        name: "Books".to_string(),
    });

    let category_lookup: HashMap<CategoryId, &Category> =
        categories.iter().map(|cat| (cat.id, cat)).collect();

    let mut products: Vec<InventoryItem> = vec![InventoryItem::create(
        "Laptop".to_string(),
        10,
        999.99,
        CategoryId(1),
    )];

    loop {
        print_main_menu();

        match read_u32("Select an action: ") {
            1 => list_categories(&categories),
            2 => list_products(&products, &category_lookup, None),
            3 => list_products_by_category(&products, &categories, &category_lookup),
            4 => create_product(&mut products, &categories, &category_lookup),
            5 => update_product_menu(&mut products),
            6 => remove_product(&mut products),
            0 => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid action."),
        }

        pause();
    }
}
