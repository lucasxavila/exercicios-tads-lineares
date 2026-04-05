use std::collections::VecDeque;
use std::time::Instant;

/// 17. Comparação de desempenho (Vec vs VecDeque)
/// Mede o tempo para 10.000 operações de enqueue/dequeue.
/// Complexidade: O(n) para Vec (remoção no início), O(n) para VecDeque (remoção no início).
pub fn questao_17() {
    let n = 10_000;

    // Teste com Vec (Remoção no início é O(n))
    let mut v = Vec::new();
    let start = Instant::now();
    for i in 0..n { v.push(i); }
    for _ in 0..n { v.remove(0); }
    println!("Tempo Vec (ingênua): {:?}", start.elapsed());

    // Teste com VecDeque (Remoção no início é O(1))
    let mut dq = VecDeque::new();
    let start = Instant::now();
    for i in 0..n { dq.push_back(i); }
    for _ in 0..n { dq.pop_front(); }
    println!("Tempo VecDeque: {:?}", start.elapsed());
}

/// 18. Quando usar qual TAD? 
/// Justificativas para cada cenário proposto: 
pub fn questao_18() {
    println!("--- Justificativas de Escolha de TAD ---");
    println!("(a) Implementar o botão \"Ctrl+Z\" de um editor
Escolha: Pilha (Stack)
Por que: Segue o princípio LIFO (Last-In, First-Out). A última ação realizada
é a primeira que deve ser removida/desfeita.

(b) Processar pedidos de um restaurante em ordem
Escolha: Fila (Queue)
Por que: Segue o princípio FIFO (First-In, First-Out). Garante que o primeiro
pedido a chegar seja o primeiro a ser servido, mantendo a integridade da ordem.  
  
(c) Verificar se um arquivo HTML tem tags bem formadas
Escolha: Pilha (Stack)
Por que: Tags HTML são aninhadas. Quando encontramos uma tag de fechamento,
ela deve corresponder à última tag de abertura encontrada (topo da pilha).

(d) Navegar nos arquivos de um diretório em largura
Escolha: Fila (Queue)
Por que: Algoritmos de busca em largura (BFS) utilizam filas para processar 
todos os nós de um nível antes de passar para os filhos (próximo nível).

(e) Verificar se uma sequência de palavras é palíndromo 
Escolha: Deque (Double-Ended Queue)
Por que: Permite a remoção e comparação simultânea dos elementos das duas 
extremidades (frente e fundo) com custo O(1).
    ");
}

/// 19. Processar em lotes
/// Processa elementos em pedaços de tamanho fixo.
/// Complexidade: O(n) para processar todos os elementos, onde n é o número total de elementos na fila.
pub fn questao_19(fila: &mut VecDeque<i32>, tamanho_lote: usize) {
    while !fila.is_empty() {
        let mut lote = Vec::new();
        for _ in 0..tamanho_lote {
            if let Some(n) = fila.pop_front() {
                lote.push(n);
            }
        }
        println!("Processando lote: {:?}", lote);
    }
}

/// 20. Mini-projeto Round Robin
/// Simula escalonamento de processos com um quantum de tempo Q.
/// Complexidade: O(n) para processar todos os processos, onde n é o número total de processos. Cada processo pode ser reprocessado várias vezes dependendo do seu tempo de execução e do quantum.
pub fn questao_20(mut processos: VecDeque<(String, i32)>, quantum: i32) {
    let mut tempo_total = 0;
    
    while let Some((nome, mut tempo_restante)) = processos.pop_front() {
        if tempo_restante > quantum {
            tempo_total += quantum;
            tempo_restante -= quantum;
            processos.push_back((nome, tempo_restante)); // Volta para o fim da fila
        } else {
            tempo_total += tempo_restante;
            println!("Processo {} concluído em {} unidades de tempo", nome, tempo_total);
        }
    }
}