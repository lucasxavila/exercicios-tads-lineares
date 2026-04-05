#![allow(dead_code)]

mod grupo1;
mod grupo2;
mod grupo3;
mod grupo4;
mod grupo5;

use std::collections::VecDeque;
use grupo2::{StackMin, Navegador, Editor};

fn main() {
    println!("====================================================");
    println!("ATIVIDADE TADs LINEARES - ESTRUTURAS DE DADOS E ALGORITMOS");
    println!("====================================================\n");

    // --- GRUPO 1: VEC E OPERAÇÕES BÁSICAS ---
    println!("--- GRUPO 1 ---");
    let v_inv = vec![1, 2, 3, 4, 5];
    println!("1. Inversão: {:?}", grupo1::questao_1(v_inv));

    let frase = vec!['a', 'b', 'a', 'c', 'a'];
    println!("\n2. Ocorrências: {:?}", grupo1::questao_2(frase));

    let mut v_pares = vec![1, 2, 3, 4, 5, 6];
    grupo1::questao_3(&mut v_pares);
    println!("\n3. Sem pares: {:?}", v_pares);

    let v1 = vec![1, 3, 5];
    let v2 = vec![2, 4, 6];
    println!("\n4. Mescla: {:?}\n", grupo1::questao_4(v1, v2));


    // --- GRUPO 2: PILHA (STACK) ---
    println!("--- GRUPO 2 ---");
    println!("5. RPN (3 4 + 2 *): {}", grupo2::questao_5("3 4 + 2 *"));

    let mut nav = Navegador::new("google.com");
    nav.visitar("rust-lang.org");
    nav.voltar();
    println!("\n6. Histórico OK.");

    let mut edit = Editor::new();
    edit.digitar("Rust");
    edit.desfazer();
    println!("\n7. Desfazer/Refazer OK.");

    println!("\n8. Balanço {{[()]}}: {}", grupo2::questao_8("{[()]}"));

    let mut s_min = StackMin::new();
    s_min.push(10); s_min.push(5); s_min.push(20);
    println!("\n9. Mínimo da pilha: {:?}\n", s_min.min());


    // --- GRUPO 3: FILA (QUEUE) ---
    println!("--- GRUPO 3 ---");
    println!("10. Simulação de Banco:");
    grupo3::questao_10(3);

    let mut fila_imp = VecDeque::new();
    fila_imp.push_back(grupo3::Trabalho { nome: "Doc1.pdf".to_string(), paginas: 5 });
    print!("\n11. "); grupo3::questao_11(fila_imp);

    let mut buffer = grupo3::FilaCircular::new(2);
    buffer.push("Msg1".to_string()); buffer.push("Msg2".to_string()); buffer.push("Msg3".to_string());
    println!("\n12. Buffer Circular OK.");

    let mut fila_prio = vec![(1, "T1".to_string()), (3, "T2".to_string())];
    println!("\n13. Fila Prioridade (maior sai): {:?}\n", grupo3::questao_13(&mut fila_prio));


    // --- GRUPO 4: DEQUE ---
    println!("--- GRUPO 4 ---");
    let pal = "A man a plan a canal Panama";
    println!("14. Palíndromo: {}", grupo4::questao_14(pal));

    let janela = vec![1, 3, -1, -3, 5, 3, 6, 7];
    println!("\n15. Janela Máxima (k=3): {:?}", grupo4::questao_15(janela, 3));

    print!("\n16. "); grupo4::questao_16();
    println!("");


    // --- GRUPO 5: REFLEXÃO E ANÁLISE ---
    println!("--- GRUPO 5 ---");
    println!("17. Benchmark:");
    grupo5::questao_17();

    println!("\n18. Justificativas Teóricas:");
    grupo5::questao_18();

    println!("\n19. Processamento em Lotes:");
    let mut fila_lotes = VecDeque::from(vec![10, 20, 30, 40, 50]);
    grupo5::questao_19(&mut fila_lotes, 2);

    println!("\n20. Round Robin Escalonamento:");
    let mut procs = VecDeque::new();
    procs.push_back(("P1".to_string(), 7));
    procs.push_back(("P2".to_string(), 4));
    grupo5::questao_20(procs, 3);

    println!("\n====================================================");
    println!("FIM DA ATIVIDADE");
    println!("====================================================");
}