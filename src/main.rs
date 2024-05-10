use std::env;
mod dir_explorer;
use std::error::Error;

fn main() {
    // Captura os argumentos da linha de comando
    let args: Vec<String> = env::args().collect();

    // Verifica se foram fornecidos argumentos suficientes
    if args.len() < 2 {
        println!("Uso: {} <operacao> [-d <diretorio>] [<tipo de arquivo>]", args[0]);
        return;
    }

    // Extrai a operação, o diretório e o tipo de arquivo dos argumentos
    let operation = &args[1];
    let mut dir_path = ".";
    let mut file_type = None;

    // Itera sobre os argumentos para verificar as opções
    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "-d" => {
                // Verifica se o próximo argumento é o diretório
                if i + 1 < args.len() {
                    dir_path = &args[i + 1];
                    i += 2; // Pula para o próximo argumento após o diretório
                } else {
                    println!("Erro: Diretório não especificado após a opção '-d'");
                    return;
                }
            }
            _ => {
                // O argumento é um tipo de arquivo
                file_type = Some(&args[i]);
                i += 1; // Pula para o próximo argumento
            }
        }
    }

    // Realiza a operação apropriada com base nos argumentos fornecidos
    match operation.as_str() {
        "ls" => {
            dir_explorer::list_files(dir_path, file_type.map(|x| x.as_str()), false);
        }
        "lsz" => {
            dir_explorer::list_files(dir_path, file_type.map(|x| x.as_str()), true);
        }
        _ => println!("Operacao invalida. Use 'ls' ou 'lsz'."),
    }
}
