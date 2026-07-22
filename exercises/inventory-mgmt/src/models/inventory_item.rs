use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::category::CategoryId; // means go to parent module, models , then access its category module and then access CategoryId struct

/*
Debug → print with {:?}
Clone → item.clone()
Copy → implicit copying for small types
PartialEq / Eq → == and !=
Hash → use as a HashMap key
Default → InventoryItem::default()
 */

#[derive(Debug, Clone)] // Provides implementation
pub struct InventoryItem {
    pub id: Uuid,
    pub name: String,
    pub quantity: u32,
    pub price: f64,
    pub category_id: CategoryId,
    pub date_created: DateTime<Utc>,
}

impl InventoryItem {
    pub fn create(name: String, quantity: u32, price: f64, category_id: CategoryId) -> Self {
        // Use instead of new to avoid confusion with the Default trait
        Self {
            id: Uuid::new_v4(),
            name,
            quantity,
            price,
            category_id,
            date_created: Utc::now(),
        }
    }

    pub fn update_price(&mut self, new_price: f64) {
        self.price = new_price;
    }

    pub fn update_quantity(&mut self, new_quantity: u32) {
        self.quantity = new_quantity;
    }

    pub fn print(&self) {
        // Read-only
        println!("==============================");
        println!("Inventory Item");
        println!("==============================");
        println!("ID        : {}", self.id);
        println!("Name      : {}", self.name);
        println!("Quantity  : {}", self.quantity);
        println!("Price     : {:.2} €", self.price);
        println!("Category  : {:?}", self.category_id);
        println!("Created   : {}", self.date_created);
        println!("==============================");
    }
}
