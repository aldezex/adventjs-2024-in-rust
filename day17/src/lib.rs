/*
El Grinch ha estado haciendo de las suyas en el Polo Norte y ha sembrado bombas de carbón explosivo 💣 en la fábrica de juguetes de los duendes. Quiere que todos los juguetes queden inutilizados y por eso ha dejado una cuadrícula donde algunas celdas tienen carbón explosivo (true) y otras están vacías (false).

Los duendes necesitan tu ayuda para mapear las zonas peligrosas. Cada celda vacía debe mostrar un número que indique cuántas bombas de carbón explosivo hay en las posiciones adyacentes, incluidas las diagonales.

detectBombs([
  [true, false, false],
  [false, true, false],
  [false, false, false]
])
// [
//   [1, 2, 1],
//   [2, 1, 1],
//   [1, 1, 1]
// ]

detectBombs([
  [true, false],
  [false, false]
])
// [
//   [0, 1],
//   [1, 1]
// ]

detectBombs([
  [true, true],
  [false, false],
  [true, true]
])

// [
//   [1, 1],
//   [4, 4],
//   [1, 1]
// ]
*/

pub fn detect_bombs(grid: Vec<Vec<bool>>) -> Vec<Vec<u8>> {
    let directions = vec![
        vec![-1, -1],
        vec![-1, 0],
        vec![-1, 1],
        vec![0, -1],
        vec![0, 0],
        vec![0, 1],
        vec![1, -1],
        vec![1, 0],
        vec![1, 1],
    ];

    return grid
        .iter()
        .enumerate()
        .map(|(i, row)| {
            return row
                .iter()
                .enumerate()
                .map(|(j, cell)| {
                    if *cell {
                        return 1;
                    }
                    return directions.iter().fold(0, |acc, dir| {
                        let x = i as i32 + dir[0];
                        let y = j as i32 + dir[1];
                        if x >= 0 && x < grid.len() as i32 && y >= 0 && y < row.len() as i32 {
                            return acc + if grid[x as usize][y as usize] { 1 } else { 0 };
                        }
                        return acc;
                    });
                })
                .collect();
        })
        .collect();
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_detect_bombs() {
        assert_eq!(
            super::detect_bombs(vec![
                vec![true, false, false],
                vec![false, true, false],
                vec![false, false, false]
            ]),
            vec![vec![1, 2, 1], vec![2, 1, 1], vec![1, 1, 1]]
        );
        assert_eq!(
            super::detect_bombs(vec![vec![true, false], vec![false, false]]),
            vec![vec![1, 1], vec![1, 1]]
        );
        assert_eq!(
            super::detect_bombs(vec![vec![true, true], vec![false, false], vec![true, true]]),
            vec![vec![1, 1], vec![4, 4], vec![1, 1]]
        );
    }
}
