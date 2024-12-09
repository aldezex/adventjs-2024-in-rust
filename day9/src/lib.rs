/*
Los elfos están jugando con un tren 🚂 mágico que transporta regalos. Este tren se mueve en un tablero representado por un array de strings.

El tren está compuesto por una locomotora (@), seguida de sus vagones (o), y debe recoger frutas mágicas (*) que le sirve de combustible. El movimiento del tren sigue las siguientes reglas:

Recibirás dos parámetros board y mov.

board es un array de strings que representa el tablero:

@ es la locomotora del tren.
o son los vagones del tren.
* es una fruta mágica.
· son espacios vacíos.
mov es un string que indica el próximo movimiento del tren desde la cabeza del tren @:

'L': izquierda
'R': derecha
'U': arriba
'D': abajo.
Con esta información, debes devolver una cadena de texto:

'crash': Si el tren choca contra los bordes del tablero o contra sí mismo.
'eat': Si el tren recoge una fruta mágica (*).
'none': Si avanza sin chocar ni recoger ninguna fruta mágica.
Ejemplo:

const board = ['·····', '*····', '@····', 'o····', 'o····']

console.log(moveTrain(board, 'U'))
// ➞ 'eat'
// Porque el tren se mueve hacia arriba y encuentra una fruta mágica

console.log(moveTrain(board, 'D'))
// ➞ 'crash'
// El tren se mueve hacia abajo y la cabeza se choca consigo mismo

console.log(moveTrain(board, 'L'))
// ➞ 'crash'
// El tren se mueve a la izquierda y se choca contra la pared

console.log(moveTrain(board, 'R'))
// ➞ 'none'
// El tren se mueve hacia derecha y hay un espacio vacío en la derecha
*/

pub fn move_train(board: &[&str], mov: &str) -> &'static str {
    let board = board
        .iter()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut train = (0, 0);

    for (i, row) in board.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '@' {
                train = (i, j);
            }
        }
    }

    let (mut i, mut j) = train;

    match mov {
        "U" => {
            if i == 0 {
                return "crash";
            }
            i -= 1;
        }
        "D" => {
            if i == board.len() - 1 {
                return "crash";
            }
            i += 1;
        }
        "L" => {
            if j == 0 {
                return "crash";
            }
            j -= 1;
        }
        "R" => {
            if j == board[0].len() - 1 {
                return "none";
            }
            j += 1;
        }
        _ => unreachable!(),
    }

    if board[i][j] == 'o' {
        return "crash";
    }

    if board[i][j] == '*' {
        return "eat";
    }

    "none"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_train() {
        assert_eq!(
            move_train(&["·····", "*····", "@····", "o····", "o····"], "U"),
            "eat"
        );
        assert_eq!(
            move_train(&["·····", "*····", "@····", "o····", "o····"], "D"),
            "crash"
        );
        assert_eq!(
            move_train(&["·····", "*····", "@····", "o····", "o····"], "L"),
            "crash"
        );
        assert_eq!(
            move_train(&["·····", "*····", "@····", "o····", "o····"], "R"),
            "none"
        );
    }
}
