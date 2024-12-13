/*
Los elfos del Polo Norte han creado un robot 🤖 especial que ayuda a Papá Noel a distribuir regalos dentro de un gran almacén. El robot se mueve en un plano 2D y partimos desde el origen (0, 0).

Queremos saber si, tras ejecutar una serie de movimientos, el robot vuelve a estar justo donde empezó.

Las órdenes básicas del robot son:

L: Mover hacia la izquierda
R: Mover hacia la derecha
U: Mover hacia arriba
D: Mover hacia abajo
Pero también tiene ciertos modificadores para los movimientos:

*: El movimiento se realiza con el doble de intensidad (ej: *R significa RR)
!: El siguiente movimiento se invierte (ej: R!L se considera como RR)
?: El siguiente movimiento se hace sólo si no se ha hecho antes (ej: R?R significa R)
Nota: Cuando el movimiento se invierte con ! se contabiliza el movimiento invertido y no el original. Por ejemplo, !U?U invierte el movimiento de U, por lo que contabiliza que se hizo el movimiento D pero no el U. Así !U?U se traduce como D?U y, por lo tanto, se haría el movimiento U final.

Debes devolver:

true: si el robot vuelve a estar justo donde empezó
[x, y]: si el robot no vuelve a estar justo donde empezó, devolver la posición donde se detuvo
isRobotBack('R')     // [1, 0]
isRobotBack('RL')    // true
isRobotBack('RLUD')  // true
isRobotBack('*RU')   // [2, 1]
isRobotBack('R*U')   // [1, 2]
isRobotBack('LLL!R') // [-4, 0]
isRobotBack('R?R')   // [1, 0]
isRobotBack('U?D')   // true
isRobotBack('R!L')   // [2,0]
isRobotBack('U!D')   // [0,2]
isRobotBack('R?L')   // true
isRobotBack('U?U')   // [0,1]
isRobotBack('*U?U')  // [0,2]
isRobotBack('U?D?U') // true

// Ejemplos paso a paso:
isRobotBack('R!U?U') // [1,0]
// 'R'  -> se mueve a la derecha
// '!U' -> se invierte y se convierte en 'D'
// '?U' -> se mueve arriba, porque no se ha hecho el movimiento 'U'

isRobotBack('UU!U?D') // [0,1]
// 'U'  -> se mueve arriba
// 'U'  -> se mueve arriba
// '!U' -> se invierte y se convierte en 'D'
// '?D' -> no se mueve, ya que ya se hizo el movimiento 'D'
*/
#[derive(Debug, PartialEq)]
pub enum Output {
    Back(bool),
    Position([i32; 2]),
}

pub fn is_robot_back(movements: &str) -> Output {
    let mut pos = [0, 0];
    let mut used_moves = std::collections::HashSet::new();
    let chars: Vec<char> = movements.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let mut multiplier = 1;
        let mut invert = false;
        let mut check_used = false;

        // Procesar los modificadores
        while i < chars.len() {
            match chars[i] {
                '*' => {
                    multiplier = 2;
                    i += 1;
                }
                '!' => {
                    invert = true;
                    i += 1;
                }
                '?' => {
                    check_used = true;
                    i += 1;
                }
                _ => break,
            }
        }

        if i >= chars.len() {
            break;
        }

        // Obtener el movimiento base
        let mov = match chars[i] {
            'R' => {
                if !invert {
                    [1, 0]
                } else {
                    [-1, 0]
                }
            }
            'L' => {
                if !invert {
                    [-1, 0]
                } else {
                    [1, 0]
                }
            }
            'U' => {
                if !invert {
                    [0, 1]
                } else {
                    [0, -1]
                }
            }
            'D' => {
                if !invert {
                    [0, -1]
                } else {
                    [0, 1]
                }
            }
            _ => {
                i += 1;
                continue;
            }
        };

        // Si es un movimiento que ya se usó, lo saltamos
        if check_used && used_moves.contains(&mov) {
            i += 1;
            continue;
        }

        // Aplicar el movimiento
        pos[0] += mov[0] * multiplier;
        pos[1] += mov[1] * multiplier;
        used_moves.insert(mov);
        i += 1;
    }

    if pos == [0, 0] {
        Output::Back(true)
    } else {
        Output::Position(pos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_robot_back() {
        assert_eq!(is_robot_back("R"), Output::Position([1, 0]));
        assert_eq!(is_robot_back("RL"), Output::Back(true));
        assert_eq!(is_robot_back("RLUD"), Output::Back(true));
        assert_eq!(is_robot_back("*RU"), Output::Position([2, 1]));
        assert_eq!(is_robot_back("R*U"), Output::Position([1, 2]));
        assert_eq!(is_robot_back("LLL!R"), Output::Position([-4, 0]));
        assert_eq!(is_robot_back("R?R"), Output::Position([1, 0]));
        assert_eq!(is_robot_back("U?D"), Output::Back(true));
        assert_eq!(is_robot_back("R!L"), Output::Position([2, 0]));
        assert_eq!(is_robot_back("U!D"), Output::Position([0, 2]));
        assert_eq!(is_robot_back("R?L"), Output::Back(true));
        assert_eq!(is_robot_back("U?U"), Output::Position([0, 1]));
        assert_eq!(is_robot_back("*U?U"), Output::Position([0, 2]));
        assert_eq!(is_robot_back("U?D?U"), Output::Back(true));
        assert_eq!(is_robot_back("R!U?U"), Output::Position([1, 0]));
        assert_eq!(is_robot_back("UU!U?D"), Output::Position([0, 1]));
    }
}
