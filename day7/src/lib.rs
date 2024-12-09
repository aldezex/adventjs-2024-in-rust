/*
¡El grinch 👹 ha pasado por el taller de Santa Claus! Y menudo desastre ha montado. Ha cambiado el orden de algunos paquetes, por lo que los envíos no se pueden realizar.

Por suerte, el elfo Pheralb ha detectado el patrón que ha seguido el grinch para desordenarlos. Nos ha escrito las reglas que debemos seguir para reordenar los paquetes. Las instrucciones que siguen son:

Recibirás un string que contiene letras y paréntesis.
Cada vez que encuentres un par de paréntesis, debes voltear el contenido dentro de ellos.
Si hay paréntesis anidados, resuelve primero los más internos.
Devuelve el string resultante con los paréntesis eliminados, pero con el contenido volteado correctamente.
Nos ha dejado algunos ejemplos:

fixPackages('a(cb)de')
// ➞ "abcde"
// Volteamos "cb" dentro de los paréntesis

fixPackages('a(bc(def)g)h')
// ➞ "agdefcbh"
// 1º volteamos "def" → "fed", luego volteamos "bcfedg" → "gdefcb"

fixPackages('abc(def(gh)i)jk')
// ➞ "abcighfedjk"
// 1º volteamos "gh" → "hg", luego "defhgi" → "ighfed"

fixPackages('a(b(c))e')
// ➞ "acbe"
// 1º volteamos "c" → "c", luego "bc" → "cb"
*/
pub fn fix_packages(packages: &'static str) -> String {
    let re = regex::Regex::new(r"\(([^()]*)\)").unwrap();
    let mut result = String::from(packages);

    while let Some(caps) = re.captures(&result) {
        result = result.replacen(&caps[0], &caps[1].chars().rev().collect::<String>(), 1);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fix_packages() {
        assert_eq!(fix_packages("a(cb)de"), "abcde");
        assert_eq!(fix_packages("a(bc(def)g)h"), "agdefcbh");
        assert_eq!(fix_packages("abc(def(gh)i)jk"), "abcighfedjk");
        assert_eq!(fix_packages("a(b(c))e"), "acbe");
    }
}
