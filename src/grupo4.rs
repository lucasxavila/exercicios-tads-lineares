use std::collections::VecDeque;

/// 14. Palíndromo com Deque
/// Complexidade: O(n) para verificar se a string é um palíndromo, onde n é o comprimento da string.
pub fn questao_14(s: &str) -> bool {
    let mut dq: VecDeque<char> = s.to_lowercase().chars().filter(|c| c.is_alphanumeric()).collect();
    while dq.len() > 1 {
        if dq.pop_front() != dq.pop_back() { return false; }
    }
    true
}

/// 15. Janela deslizante máxima O(n)
/// Complexidade: O(n) para encontrar a máxima em cada janela, onde n é o comprimento do vetor.
pub fn questao_15(nums: Vec<i32>, k: usize) -> Vec<i32> {
    let mut dq = VecDeque::new();
    let mut res = Vec::new();
    for i in 0..nums.len() {
        if !dq.is_empty() && *dq.front().unwrap() <= i.saturating_sub(k) { dq.pop_front(); }
        while !dq.is_empty() && nums[*dq.back().unwrap()] <= nums[i] { dq.pop_back(); }
        dq.push_back(i);
        if i >= k - 1 { res.push(nums[*dq.front().unwrap()]); }
    }
    res
}

/// 16. Fila de tarefas com prioridade de frente
/// Tarefas urgentes entram na frente, normais atrás, todas saem pela frente.
/// Complexidade: O(1) para todas as operações.
pub fn questao_16() {
    let mut deque = VecDeque::new();
    
    // Inserindo tarefa normal
    deque.push_back("Tarefa Normal 1");
    // Inserindo tarefa urgente (vai para a frente da fila)
    deque.push_front("Tarefa URGENTE 1");
    
    while let Some(tarefa) = deque.pop_front() {
        println!("Processando: {}", tarefa);
    }
}