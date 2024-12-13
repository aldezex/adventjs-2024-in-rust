/*
Estás en un mercado muy especial en el que se venden árboles de Navidad 🎄. Cada uno viene decorado con una serie de adornos muy peculiares, y el precio del árbol se determina en función de los adornos que tiene.

*: Copo de nieve - Valor: 1
o: Bola de Navidad - Valor: 5
^: Arbolito decorativo - Valor: 10
#: Guirnalda brillante - Valor: 50
@: Estrella polar - Valor: 100
Normalmente se sumarían todos los valores de los adornos y ya está…

Pero, ¡ojo! Si un adorno se encuentra inmediatamente a la izquierda de otro de mayor valor, en lugar de sumar, se resta su valor.

calculatePrice('***')  // 3   (1 + 1 + 1)
calculatePrice('*o')   // 4   (5 - 1)
calculatePrice('o*')   // 6   (5 + 1)
calculatePrice('*o*')  // 5  (-1 + 5 + 1)
calculatePrice('**o*') // 6  (1 - 1 + 5 + 1)
calculatePrice('o***') // 8   (5 + 3)
calculatePrice('*o@')  // 94  (-5 - 1 + 100)
calculatePrice('*#')   // 49  (-1 + 50)
calculatePrice('@@@')  // 300 (100 + 100 + 100)
calculatePrice('#@')   // 50  (-50 + 100)
calculatePrice('#@Z')  // undefined (Z es desconocido)
*/

pub fn calculate_price(input: &str) -> Option<i32> {
    input
        .chars()
        .map(|c| match c {
            '*' => Some(1),
            'o' => Some(5),
            '^' => Some(10),
            '#' => Some(50),
            '@' => Some(100),
            _ => None,
        })
        .collect::<Option<Vec<_>>>()
        .map(|values| {
            values
                .windows(2)
                .map(|w| if w[1] > w[0] { -w[0] } else { w[0] })
                .sum::<i32>()
                + values.last().unwrap_or(&0)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_price() {
        assert_eq!(calculate_price("***"), Some(3));
        assert_eq!(calculate_price("*o"), Some(4));
        assert_eq!(calculate_price("o*"), Some(6));
        assert_eq!(calculate_price("*o*"), Some(5));
        assert_eq!(calculate_price("**o*"), Some(6));
        assert_eq!(calculate_price("o***"), Some(8));
        assert_eq!(calculate_price("*o@"), Some(94));
        assert_eq!(calculate_price("*#"), Some(49));
        assert_eq!(calculate_price("@@@"), Some(300));
        assert_eq!(calculate_price("#@"), Some(50));
        assert_eq!(calculate_price("#@Z"), None);
    }
}
