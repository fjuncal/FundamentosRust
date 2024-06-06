mod utils;
use utils::terminal::{limpar_tela, esperar_enter, exibir_menu};
fn main() {
    let itens = [
        "Fundamentos",
        "Tipos",
        "Controle",
        "Funções",
        "Ownership"
    ];

    exibir_menu("Principal", &itens, true);
    esperar_enter();
}
