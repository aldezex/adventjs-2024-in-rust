/*
Los elfos 🧝🧝‍♂️ de Santa Claus han encontrado un montón de botas mágicas desordenadas en el taller. Cada bota se describe por dos valores:

type indica si es una bota izquierda (I) o derecha (R).
size indica el tamaño de la bota.
Tu tarea es ayudar a los elfos a emparejar todas las botas del mismo tamaño que tengan izquierda y derecha. Para ello, debes devolver una lista con los tamaños disponibles después de emparejar las botas.

const shoes = [
  { type: 'I', size: 38 },
  { type: 'R', size: 38 },
  { type: 'R', size: 42 },
  { type: 'I', size: 41 },
  { type: 'I', size: 42 }
]

organizeShoes(shoes)
// [38, 42]

const shoes2 = [
  { type: 'I', size: 38 },
  { type: 'R', size: 38 },
  { type: 'I', size: 38 },
  { type: 'I', size: 38 },
  { type: 'R', size: 38 }
]
// [38, 38]

const shoes3 = [
  { type: 'I', size: 38 },
  { type: 'R', size: 36 },
  { type: 'R', size: 42 },
  { type: 'I', size: 41 },
  { type: 'I', size: 43 }
]

organizeShoes(shoes3)
// []
*/
use itertools::Itertools;
use std::collections::HashMap;
use std::iter;

#[derive(Debug, PartialEq)]
struct Shoe {
    _type: char,
    size: i32,
}

fn organize_shoes(shoes: Vec<Shoe>) -> Vec<i32> {
    shoes
        .iter()
        .fold(HashMap::new(), |mut acc, shoe| {
            let entry = acc.entry(shoe.size).or_insert_with(|| (0i32, 0i32));

            match shoe._type {
                'I' => entry.0 += 1,
                'R' => entry.1 += 1,
                _ => {}
            }
            acc
        })
        .into_iter()
        .filter(|(_, (left, right))| *left > 0 && *right > 0)
        .flat_map(|(size, (left, right))| iter::repeat(size).take(left.min(right) as usize))
        .sorted()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_organize_shoes() {
        let shoes = vec![
            Shoe {
                _type: 'I',
                size: 38,
            },
            Shoe {
                _type: 'R',
                size: 38,
            },
            Shoe {
                _type: 'R',
                size: 42,
            },
            Shoe {
                _type: 'I',
                size: 41,
            },
            Shoe {
                _type: 'I',
                size: 42,
            },
        ];
        assert_eq!(organize_shoes(shoes), vec![38, 42]);

        let shoes2 = vec![
            Shoe {
                _type: 'I',
                size: 38,
            },
            Shoe {
                _type: 'R',
                size: 38,
            },
            Shoe {
                _type: 'I',
                size: 38,
            },
            Shoe {
                _type: 'I',
                size: 38,
            },
            Shoe {
                _type: 'R',
                size: 38,
            },
        ];
        assert_eq!(organize_shoes(shoes2), vec![38, 38]);

        let shoes3 = vec![
            Shoe {
                _type: 'I',
                size: 38,
            },
            Shoe {
                _type: 'R',
                size: 36,
            },
            Shoe {
                _type: 'R',
                size: 42,
            },
            Shoe {
                _type: 'I',
                size: 41,
            },
            Shoe {
                _type: 'I',
                size: 43,
            },
        ];
        assert_eq!(organize_shoes(shoes3), vec![]);
    }
}
