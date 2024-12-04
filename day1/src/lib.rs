/*
Santa Claus 🎅 ha recibido una lista de números mágicos que representan regalos 🎁, pero algunos de ellos están duplicados y deben ser eliminados para evitar confusiones. Además, los regalos deben ser ordenados en orden ascendente antes de entregárselos a los elfos.

Tu tarea es escribir una función que reciba una lista de números enteros (que pueden incluir duplicados) y devuelva una nueva lista sin duplicados, ordenada en orden ascendente.

const gifts1 = [3, 1, 2, 3, 4, 2, 5]
const preparedGifts1 = prepareGifts(gifts1)
console.log(preparedGifts1) // [1, 2, 3, 4, 5]

const gifts2 = [6, 5, 5, 5, 5]
const preparedGifts2 = prepareGifts(gifts2)
console.log(preparedGifts2) // [5, 6]

const gifts3 = []
const preparedGifts3 = prepareGifts(gifts3)
console.log(preparedGifts3) // []
// No hay regalos, la lista queda vacía
*/

pub fn prepare_gifts(gifts: Vec<i32>) -> Vec<i32> {
    let mut gifts = gifts;
    gifts.sort();
    gifts.dedup();
    gifts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prepare_gifts() {
        assert_eq!(
            prepare_gifts(vec![3, 1, 2, 3, 4, 2, 5]),
            vec![1, 2, 3, 4, 5]
        );
        assert_eq!(prepare_gifts(vec![6, 5, 5, 5, 5]), vec![5, 6]);
        assert_eq!(prepare_gifts(vec![]), vec![]);
    }
}
