/*

Los elfos programadores están creando un pequeño ensamblador mágico para controlar las máquinas del taller de Santa Claus.

Para ayudarles, vamos a implementar un intérprete sencillo que soporte las siguientes instrucciones mágicas:

MOV x y: Copia el valor x (puede ser un número o el contenido de un registro) en el registro y
INC x: Incrementa en 1 el contenido del registro x
DEC x: Decrementa en 1 el contenido del registro x
JMP x y: Si el valor del registro x es 0 entonces salta a la instrucción en el índice y y sigue ejecutándose el programa desde ahí.
Comportamiento esperado:
Si se intenta acceder, incrementar o decrementar a un registro que no ha sido inicializado, se tomará el valor 0 por defecto.
El salto con JMP es absoluto y lleva al índice exacto indicado por y.
Al finalizar, el programa debe devolver el contenido del registro A. Si A no tenía un valor definido, retorna undefined.
const instructions = [
  'MOV -1 C', // copia -1 al registro 'C',
  'INC C', // incrementa el valor del registro 'C'
  'JMP C 1', // salta a la instrucción en el índice 1 si 'C' es 0
  'MOV C A', // copia el registro 'C' al registro 'a',
  'INC A' // incrementa el valor del registro 'a'
]

compile(instructions) // -> 2

/**
 Ejecución paso a paso:
 0: MOV -1 C -> El registro C recibe el valor -1
 1: INC C    -> El registro C pasa a ser 0
 2: JMP C 1  -> C es 0, salta a la instrucción en el índice 1
 1: INC C    -> El registro C pasa a ser 1
 2: JMP C 1  -> C es 1, ignoramos la instrucción
 3: MOV C A  -> Copiamos el registro C en A. Ahora A es 1
 4: INC A    -> El registro A pasa a ser 2
 */
Nota: Los registros que no han sido inicializados previamente se inicializan a 0.
*/

use std::collections::HashMap;

pub fn compile(instructions: Vec<&str>) -> Option<i32> {
    let mut registers: HashMap<&str, i32> = HashMap::new();
    let mut i = 0;
    while i < instructions.len() {
        let instruction = instructions[i];
        let parts: Vec<&str> = instruction.split_whitespace().collect();
        match parts[0] {
            "MOV" => {
                let value = match parts[1].parse::<i32>() {
                    Ok(v) => v,
                    Err(_) => *registers.get(parts[1]).unwrap_or(&0),
                };
                registers.insert(parts[2], value);
            }
            "INC" => {
                let value = registers.get(parts[1]).unwrap_or(&0) + 1;
                registers.insert(parts[1], value);
            }
            "DEC" => {
                let value = registers.get(parts[1]).unwrap_or(&0) - 1;
                registers.insert(parts[1], value);
            }
            "JMP" => {
                let value = *registers.get(parts[1]).unwrap_or(&0);
                if value == 0 {
                    i = parts[2].parse::<usize>().unwrap();
                    continue;
                }
            }
            _ => {}
        }
        i += 1;
    }
    registers.get("A").copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile() {
        let instructions = vec!["MOV -1 C", "INC C", "JMP C 1", "MOV C A", "INC A"];
        assert_eq!(compile(instructions), Some(2));
    }
}
