use std::collections::HashMap;

/// 1. Inversão com Vec usando push/pop
/// Complexidade: O(n)
pub fn questao_1(mut v: Vec<i32>) -> Vec<i32> {
    let mut invertido = Vec::new();
    while let Some(x) = v.pop() {
        invertido.push(x);
    }
    invertido
}

/// 2. Contador de ocorrências com iteração for
/// Complexidade: O(n)
pub fn questao_2(vec: Vec<char>) -> HashMap<char, usize> {
    let mut mapa = HashMap::new();
    for x in &vec {
        *mapa.entry(*x).or_insert(0) += 1;
    }
    mapa
}

/// 3. Remoção condicional de pares sem .retain()
/// Analise: O custo é O(n²) pois cada remove() desloca os elementos à direita, resultando em O(n) para cada elemento par encontrado.
pub fn questao_3(v: &mut Vec<i32>) {
    let mut i = 0;
    while i < v.len() {
        if v[i] % 2 == 0 {
            v.remove(i);
        } else {
            i += 1;
        }
    }
}

/// 4. Mescla ordenada manual
/// Complexidade: O(n + m)
pub fn questao_4(v1: Vec<i32>, v2: Vec<i32>) -> Vec<i32> {
    let mut res = Vec::with_capacity(v1.len() + v2.len());
    let (mut i, mut j) = (0, 0);
    while i < v1.len() && j < v2.len() {
        if v1[i] <= v2[j] { res.push(v1[i]); i += 1; }
        else { res.push(v2[j]); j += 1; }
    }
    res.extend(&v1[i..]);
    res.extend(&v2[j..]);
    res
}