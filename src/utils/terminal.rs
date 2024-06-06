
use rpassword::prompt_password;

pub fn esperar_enter(){
    prompt_password("Pressione ENTER para continuar..").unwrap();
}

pub fn limpar_tela(){
    print!("{esc}c", esc=27 as char);
}