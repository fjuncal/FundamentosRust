mod fns;
mod lambda;

use crate::utils::terminal::{esperar_enter, exibir_menu, limpar_tela};

pub fn executar(){
    loop {
        let itens = ["Básicos", "Map", "Filter"];

        limpar_tela();

        let selecionado = exibir_menu("Funções", &itens, true);
        limpar_tela();

        match selecionado {
            1 => fns::exemplo(),
            2 => lambda::exemplo_map(),
            3 => lambda::exemplo_filter(),
            _ => break,
        }
        esperar_enter();
    }

}