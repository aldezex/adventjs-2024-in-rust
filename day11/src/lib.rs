/*
El Grinch ha hackeado 🏴‍☠️ los sistemas del taller de Santa Claus y ha codificado los nombres de todos los archivos importantes. Ahora los elfos no pueden encontrar los archivos originales y necesitan tu ayuda para descifrar los nombres.

Cada archivo sigue este formato:

Comienza con un número (puede contener cualquier cantidad de dígitos).
Luego tiene un guion bajo _.
Continúa con un nombre de archivo y su extensión.
Finaliza con una extensión extra al final (que no necesitamos).
Ten en cuenta que el nombre de los archivos pueden contener letras (a-z, A-Z), números (0-9), otros guiones bajos (_) y guiones (-).

Tu tarea es implementar una función que reciba un string con el nombre de un archivo codificado y devuelva solo la parte importante: el nombre del archivo y su extensión.

Ejemplos:
decodeFilename('2023122512345678_sleighDesign.png.grinchwa')
// ➞ "sleighDesign.png"

decodeFilename('42_chimney_dimensions.pdf.hack2023')
// ➞ "chimney_dimensions.pdf"

decodeFilename('987654321_elf-roster.csv.tempfile')
// ➞ "elf-roster.csv"
*/

pub fn decode_filename(filename: &str) -> Option<String> {
    let re = regex::Regex::new(r"^\d+_([A-Za-z0-9_-]+\.[A-Za-z0-9]+)\.[A-Za-z0-9]+$").unwrap();

    re.captures(filename)
        .and_then(|cap| cap.get(1))
        .map(|match_| match_.as_str().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_filename() {
        assert_eq!(
            decode_filename("2023122512345678_sleighDesign.png.grinchwa"),
            Some("sleighDesign.png".to_string())
        );

        assert_eq!(
            decode_filename("42_chimney_dimensions.pdf.hack2023"),
            Some("chimney_dimensions.pdf".to_string())
        );

        assert_eq!(
            decode_filename("987654321_elf-roster.csv.tempfile"),
            Some("elf-roster.csv".to_string())
        );
    }
}
