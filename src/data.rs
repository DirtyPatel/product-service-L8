use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Dyson Supersonic Nural 1600W – Jasper Plum".into(),
            price: 629.99,
            description: "Special edition hair dryer with intelligent heat control".into(),
            image: "/Dysonjpg.jpg".into(),
            
        },
        Product {
            id: 2,
            name: "Sony WH-1000XM5 Headphones – Smoky Pink".into(),
            price: 378.00,
            description: "Industry-leading noise cancelling with ultra comfort".into(),
            image: "/Headphones.jpg".into(),
           
        },
        Product {
            id: 3,
            name: "PlayStation 5 Slim Fortnite Bundle".into(),
            price: 519.99,
            description: "PS5 Slim + Fortnite cosmetics + 1000 V-Bucks".into(),
            image: "/Playstation.jpg".into(),
            
        },
        Product {
            id: 4,
            name: "Insignia 40'' Fire TV (2025)".into(),
            price: 199.99,
            description: "1080p Full HD smart TV with Alexa voice remote".into(),
            image: "/Insignia.jpg".into(),
            
        },
        Product {
            id: 5,
            name: "Apple MacBook Pro 14'' (2024) – Space Black".into(),
            price: 1799.99,
            description: "Apple M4 chip with 16GB RAM and 512GB SSD".into(),
            image: "/Apple.jpg".into(),
           
        },
    ]
}
