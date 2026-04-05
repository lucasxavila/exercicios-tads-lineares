use std::collections::VecDeque;
use rand::Rng;

/// 10. Simulador de fila de banco
/// Clientes chegam em intervalos aleatórios e são atendidos em ordem (FIFO).
/// Complexidade: O(n) para processar todos os clientes.
pub fn questao_10(num_clientes: usize) {
    let mut fila: VecDeque<i32> = VecDeque::new();
    let mut rng = rand::thread_rng();
    
    let mut tempo_atual = 0;
    let mut tempo_total_espera = 0;

    println!("--- Simulação de Fila de Banco ---");

    for i in 1..=num_clientes {
        // Cliente chega entre 1 e 5 minutos após o anterior
        let intervalo_chegada = rng.gen_range(1..=5);
        tempo_atual += intervalo_chegada;
        
        println!("Cliente {} chegou no minuto {}", i, tempo_atual);
        fila.push_back(tempo_atual);
    }

    let mut tempo_atendimento = 0;
    let total_clientes = fila.len() as f32;

    while let Some(chegada) = fila.pop_front() {
        // O atendimento começa no tempo_atendimento ou no tempo de chegada (o que for maior)
        if tempo_atendimento < chegada {
            tempo_atendimento = chegada;
        }
        
        let espera = tempo_atendimento - chegada;
        tempo_total_espera += espera;
        
        // Simula um tempo de atendimento fixo de 4 minutos
        tempo_atendimento += 4; 
    }

    let media = tempo_total_espera as f32 / total_clientes;
    println!("Tempo médio de espera: {:.2} minutos", media);
}

/// 11. Impressora compartilhada
/// Complexidade: O(n) para processar todos os trabalhos.
pub struct Trabalho { pub nome: String, pub paginas: u32 }

pub fn questao_11(mut fila: VecDeque<Trabalho>) {
    while let Some(t) = fila.pop_front() {
        println!("Imprimindo: {} ({} páginas)", t.nome, t.paginas);
    }
}

/// 12. Buffer de mensagens com overwrite
/// Complexidade: O(1) para adicionar uma mensagem, O(1) para remover a mais antiga.
pub struct FilaCircular { buffer: VecDeque<String>, cap: usize }

impl FilaCircular {
    pub fn new(cap: usize) -> Self { Self { buffer: VecDeque::with_capacity(cap), cap } }
    pub fn push(&mut self, msg: String) {
        if self.buffer.len() == self.cap { self.buffer.pop_front(); }
        self.buffer.push_back(msg);
    }
}

/// 13. Fila de prioridade manual (Busca linear)
/// Complexidade: O(n) para encontrar o elemento de maior prioridade.
pub fn questao_13(fila: &mut Vec<(i32, String)>) -> Option<String> {
    if fila.is_empty() { return None; }
    let mut idx_max = 0;
    for i in 1..fila.len() {
        if fila[i].0 > fila[idx_max].0 { idx_max = i; }
    }
    Some(fila.remove(idx_max).1)
}