/*
Santa Claus 🎅 quiere enmarcar los nombres de los niños buenos para decorar su taller 🖼️, pero el marco debe cumplir unas reglas específicas. Tu tarea es ayudar a los elfos a generar este marco mágico.

Reglas:

Dado un array de nombres, debes crear un marco rectangular que los contenga a todos.
Cada nombre debe estar en una línea, alineado a la izquierda.
El marco está construido con * y tiene un borde de una línea de ancho.
La anchura del marco se adapta automáticamente al nombre más largo más un margen de 1 espacio a cada lado.
Ejemplo de funcionamiento:

createFrame(['midu', 'madeval', 'educalvolpz'])

// Resultado esperado:
***************
* midu        *
* madeval     *
* educalvolpz *
***************

createFrame(['midu'])

// Resultado esperado:
********
* midu *
********

createFrame(['a', 'bb', 'ccc'])

// Resultado esperado:
*******
* a   *
* bb  *
* ccc *
*******

createFrame(['a', 'bb', 'ccc', 'dddd'])
*/

use std::fmt;

#[derive(Debug)]
struct Frame {
    names: Vec<String>,
    width: usize,
}

impl Frame {
    fn new(names: Vec<String>) -> Self {
        let width = names
            .iter()
            .map(|name| name.len())
            .max()
            .unwrap_or(0)
            .saturating_add(4);

        Self { names, width }
    }

    fn border(&self) -> String {
        "*".repeat(self.width)
    }
}

impl fmt::Display for Frame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.names.is_empty() {
            return write!(f, "****\n*  *\n****");
        }

        let content_width = self.width - 4;
        let border = self.border();

        writeln!(f, "{}", border)?;

        for name in &self.names {
            writeln!(f, "* {:<width$} *", name, width = content_width)?;
        }

        write!(f, "{}", border)
    }
}

fn create_frame<S: Into<String>>(names: Vec<S>) -> Frame {
    Frame::new(names.into_iter().map(Into::into).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiple_names() {
        let frame = create_frame(vec!["midu", "madeval", "educalvolpz"]);
        assert_eq!(
            frame.to_string(),
            "\
***************
* midu        *
* madeval     *
* educalvolpz *
***************"
        );
    }

    #[test]
    fn test_single_name() {
        let frame = create_frame(vec!["midu"]);
        assert_eq!(
            frame.to_string(),
            "\
********
* midu *
********"
        );
    }

    #[test]
    fn test_increasing_length() {
        let frame = create_frame(vec!["a", "bb", "ccc"]);
        assert_eq!(
            frame.to_string(),
            "\
*******
* a   *
* bb  *
* ccc *
*******"
        );
    }

    #[test]
    fn test_empty_array() {
        let frame = create_frame(Vec::<String>::new());
        assert_eq!(
            frame.to_string(),
            "\
****
*  *
****"
        );
    }
}
