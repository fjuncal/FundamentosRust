mod utils;
mod fundamentos;

use std::process::exit;
use utils::terminal::{limpar_tela, esperar_enter, exibir_menu};
fn main() {
    loop {
        let itens = ["Fundamentos", "Tipos", "Controle", "Funções", "Ownership"];
        let selecionado = exibir_menu("Principal", &itens, true);
        limpar_tela();
        match selecionado {
            1 => fundamentos::executar(),
            2 => println!("2"),
            3 => println!("3"),
            4 => println!("4"),
            5 => println!("5"),
            _ => exit(0)
        }

        esperar_enter();
    }


}
