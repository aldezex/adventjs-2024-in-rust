/*
Los elfos están trabajando arduamente para limpiar los caminos llenos de nieve mágica ❄️. Esta nieve tiene una propiedad especial: si dos montículos de nieve idénticos y adyacentes se encuentran, desaparecen automáticamente.

Tu tarea es escribir una función que ayude a los elfos a simular este proceso. El camino se representa por una cadena de texto y cada montículo de nieve un carácter.

Tienes que eliminar todos los montículos de nieve adyacentes que sean iguales hasta que no queden más movimientos posibles.

El resultado debe ser el camino final después de haber eliminado todos los montículos duplicados:

removeSnow('zxxzoz') // -> "oz"
// 1. Eliminamos "xx", quedando "zzoz"
// 2. Eliminamos "zz", quedando "oz"

removeSnow('abcdd') // -> "abc"
// 1. Eliminamos "dd", quedando "abc"

removeSnow('zzz') // -> "z"
// 1. Eliminamos "zz", quedando "z"

removeSnow('a') // -> "a"
// No hay montículos repetidos
*/

pub fn remove_snow(s: &str) -> String {
    let mut s = s.to_string();
    let mut i = 0;
    while i < s.len() - 1 {
        if s.chars().nth(i) == s.chars().nth(i + 1) {
            s.remove(i);
            s.remove(i);
            i = 0;
        } else {
            i += 1;
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_snow() {
        assert_eq!(remove_snow("zxxzoz"), "oz");
        assert_eq!(remove_snow("abcdd"), "abc");
        assert_eq!(remove_snow("zzz"), "z");
        assert_eq!(remove_snow("a"), "a");
    }
}
