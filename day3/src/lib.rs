/*
Santa Claus 🎅 está revisando el inventario de su taller para preparar la entrega de regalos. Los elfos han registrado los juguetes en un array de objetos, pero la información está un poco desordenada. Necesitas ayudar a Santa a organizar el inventario.

Recibirás un array de objetos, donde cada objeto representa un juguete y tiene las propiedades:

name: el nombre del juguete (string).
quantity: la cantidad disponible de ese juguete (entero).
category: la categoría a la que pertenece el juguete (string).
Escribe una función que procese este array y devuelva un objeto que organice los juguetes de la siguiente manera:

Las claves del objeto serán las categorías de juguetes.
Los valores serán objetos que tienen como claves los nombres de los juguetes y como valores las cantidades totales de cada juguete en esa categoría.
Si hay juguetes con el mismo nombre en la misma categoría, debes sumar sus cantidades.
Si el array está vacío, la función debe devolver un objeto vacío {}.
const inventory = [
  { name: 'doll', quantity: 5, category: 'toys' },
  { name: 'car', quantity: 3, category: 'toys' },
  { name: 'ball', quantity: 2, category: 'sports' },
  { name: 'car', quantity: 2, category: 'toys' },
  { name: 'racket', quantity: 4, category: 'sports' }
]

organizeInventory(inventory)

// Resultado esperado:
// {
//   toys: {
//     doll: 5,
//     car: 5
//   },
//   sports: {
//     ball: 2,
//     racket: 4
//   }

const inventory2 = [
  { name: 'book', quantity: 10, category: 'education' },
  { name: 'book', quantity: 5, category: 'education' },
  { name: 'paint', quantity: 3, category: 'art' }
]

organizeInventory(inventory2)

// Resultado esperado:
// {
//   education: {
//     book: 15
//   },
//   art: {
//     paint: 3
//   }
// }
*/

struct InventoryItem {
    name: String,
    quantity: i32,
    category: String,
}

struct ClutteredInventory {
    items: Vec<InventoryItem>,
}

struct OrganizedInventory {
    categories: std::collections::HashMap<String, std::collections::HashMap<String, i32>>,
}

impl From<ClutteredInventory> for OrganizedInventory {
    fn from(cluttered_inventory: ClutteredInventory) -> Self {
        let mut categories = std::collections::HashMap::new();

        for item in cluttered_inventory.items {
            let category = categories
                .entry(item.category)
                .or_insert_with(std::collections::HashMap::new);
            let quantity = category.entry(item.name).or_insert(0);
            *quantity += item.quantity;
        }

        Self { categories }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_organize_inventory() {
        let inventory = ClutteredInventory {
            items: vec![
                InventoryItem {
                    name: "doll".to_string(),
                    quantity: 5,
                    category: "toys".to_string(),
                },
                InventoryItem {
                    name: "car".to_string(),
                    quantity: 3,
                    category: "toys".to_string(),
                },
                InventoryItem {
                    name: "ball".to_string(),
                    quantity: 2,
                    category: "sports".to_string(),
                },
                InventoryItem {
                    name: "car".to_string(),
                    quantity: 2,
                    category: "toys".to_string(),
                },
                InventoryItem {
                    name: "racket".to_string(),
                    quantity: 4,
                    category: "sports".to_string(),
                },
            ],
        };

        let organized_inventory = OrganizedInventory::from(inventory);

        let mut toys = std::collections::HashMap::new();
        toys.insert("doll".to_string(), 5);
        toys.insert("car".to_string(), 5);

        let mut sports = std::collections::HashMap::new();
        sports.insert("ball".to_string(), 2);
        sports.insert("racket".to_string(), 4);

        let mut expected = std::collections::HashMap::new();
        expected.insert("toys".to_string(), toys);
        expected.insert("sports".to_string(), sports);

        assert_eq!(organized_inventory.categories, expected);
    }
}
