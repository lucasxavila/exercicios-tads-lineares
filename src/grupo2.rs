/// 5. Calculadora RPN
/// Complexidade: O(n)
pub fn questao_5(expressao: &str) -> f64 {
    let mut pilha: Vec<f64> = Vec::new();
    for token in expressao.split_whitespace() {
        if let Ok(val) = token.parse::<f64>() {
            pilha.push(val);
        } else {
            let b = pilha.pop().unwrap();
            let a = pilha.pop().unwrap();
            match token {
                "+" => pilha.push(a + b),
                "*" => pilha.push(a * b),
                _ => panic!("Operador inválido"),
            }
        }
    }
    pilha.pop().unwrap()
}

/// 6. Histórico de navegação (Voltar/Avançar)
/// Usa duas pilhas para gerenciar a navegação.
/// Complexidade: O(1) para cada movimento.
pub struct Navegador {
    atual: String,
    historico_back: Vec<String>,
    historico_forward: Vec<String>,
}

impl Navegador {
    pub fn new(home: &str) -> Self {
        Self {
            atual: home.to_string(),
            historico_back: Vec::new(),
            historico_forward: Vec::new(),
        }
    }

    pub fn visitar(&mut self, url: &str) {
        self.historico_back.push(self.atual.clone());
        self.atual = url.to_string();
        self.historico_forward.clear(); // Nova visita limpa o "avançar"
        println!("Visitando: {}", self.atual);
    }

    pub fn voltar(&mut self) {
        if let Some(url) = self.historico_back.pop() {
            self.historico_forward.push(self.atual.clone());
            self.atual = url;
            println!("Voltou para: {}", self.atual);
        }
    }

    pub fn avancar(&mut self) {
        if let Some(url) = self.historico_forward.pop() {
            self.historico_back.push(self.atual.clone());
            self.atual = url;
            println!("Avançou para: {}", self.atual);
        }
    }
}

/// 7. Desfazer/Refazer (Editor Minimalista)
/// Implementa pilhas para armazenar estados do texto.
/// Complexidade: O(n) para salvar o estado (clone), O(1) para trocar pilhas.
pub struct Editor {
    texto: String,
    pilha_desfazer: Vec<String>,
    pilha_refazer: Vec<String>,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            texto: String::new(),
            pilha_desfazer: Vec::new(),
            pilha_refazer: Vec::new(),
        }
    }

    pub fn digitar(&mut self, novo_texto: &str) {
        self.pilha_desfazer.push(self.texto.clone());
        self.texto.push_str(novo_texto);
        self.pilha_refazer.clear();
    }

    pub fn desfazer(&mut self) {
        if let Some(anterior) = self.pilha_desfazer.pop() {
            self.pilha_refazer.push(self.texto.clone());
            self.texto = anterior;
        }
    }

    pub fn refazer(&mut self) {
        if let Some(proximo) = self.pilha_refazer.pop() {
            self.pilha_desfazer.push(self.texto.clone());
            self.texto = proximo;
        }
    }

    pub fn exibir(&self) {
        println!("Texto atual: \"{}\"", self.texto);
    }
}

/// 8. Sequências de símbolos balanceados
/// Complexidade: O(n)
pub fn questao_8(s: &str) -> bool {
    let mut pilha = Vec::new();
    for c in s.chars() {
        match c {
            '(' | '[' | '{' => pilha.push(c),
            ')' => if pilha.pop() != Some('(') { return false },
            ']' => if pilha.pop() != Some('[') { return false },
            '}' => if pilha.pop() != Some('{') { return false },
            _ => (),
        }
    }
    pilha.is_empty()
}

/// 9. Pilha com mínimo em O(1)
/// Complexidade: O(1) para push, pop e min.
pub struct StackMin {
    pilha: Vec<i32>,
    min_pilha: Vec<i32>,
}

impl StackMin {
    pub fn new() -> Self { Self { pilha: vec![], min_pilha: vec![] } }
    pub fn push(&mut self, val: i32) {
        self.pilha.push(val);
        if self.min_pilha.is_empty() || val <= *self.min_pilha.last().unwrap() {
            self.min_pilha.push(val);
        }
    }
    pub fn pop(&mut self) -> Option<i32> {
        let val = self.pilha.pop();
        if val == self.min_pilha.last().copied() { self.min_pilha.pop(); }
        val
    }
    pub fn min(&self) -> Option<i32> { self.min_pilha.last().copied() }
}