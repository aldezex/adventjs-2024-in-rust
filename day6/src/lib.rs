/*
Ya hemos empaquetado cientos de regalos 🎁… pero a un elfo se le ha olvidado revisar si el regalo, representado por un asterisco *, está dentro de la caja.

La caja tiene un regalo (*) y cuenta como dentro de la caja si:

Está rodeada por # en los bordes de la caja.
El * no está en los bordes de la caja.
Ten en cuenta entonces que el * puede estar dentro, fuera o incluso no estar. Y debemos devolver true si el * está dentro de la caja y false en caso contrario.

Ejemplos:

inBox([
  "###",
  "#*#",
  "###"
]) // ➞ true

inBox([
  "####",
  "#* #",
  "#  #",
  "####"
]) // ➞ true

inBox([
  "#####",
  "#   #",
  "#  #*",
  "#####"
]) // ➞ false

inBox([
  "#####",
  "#   #",
  "#   #",
  "#   #",
  "#####"
]) // ➞ false
*/

pub fn in_box(s: Vec<&str>) -> bool {
    let mut result = false;

    for (i, line) in s.iter().enumerate() {
        if i == 0 || i == s.len() - 1 {
            if line.contains("*") {
                result = false;
                break;
            }
        } else {
            if line.contains("*") {
                let mut chars = line.chars();
                let first = chars.next().unwrap();
                let last = chars.last().unwrap();
                if first == '#' && last == '#' {
                    result = true;
                } else {
                    result = false;
                    break;
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_box() {
        assert_eq!(in_box(vec!["###", "#*#", "###"]), true);
        assert_eq!(in_box(vec!["####", "#* #", "#  #", "####"]), true);
        assert_eq!(in_box(vec!["#####", "#   #", "#  #*", "#####"]), false);
        assert_eq!(
            in_box(vec!["#####", "#   #", "#   #", "#   #", "#####"]),
            false
        );
    }
}
