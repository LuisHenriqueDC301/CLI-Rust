use std::process::Command;
use std::path::PathBuf;
use std::env;

pub fn run(){
    run_cpp("Par.cpp");   
}


fn path(file: &str) -> PathBuf {
    env::current_dir()          // Obtém o diretório atual
        .expect("Falha ao obter o diretório")
        .join("src/scripts")    // Adiciona a pasta `src/scripts` ao caminho
        .join(file)             // Adiciona o nome do arquivo
}

pub fn run_cpp(file: &str) {
    let cpp_file = path(file);

    if !cpp_file.exists() {
        eprintln!("Arquivo {} não encontrado!", cpp_file.display());
        return;
    }

    let bin_name = cpp_file.file_stem()
        .expect("Nome de arquivo inválido")
        .to_str()
        .expect("Falha na conversão do nome");

    let bin_path = env::current_dir()
        .expect("Falha ao obter o diretório")
        .join(format!("{bin_name}"));

    // Compilando com g++
    let compile_status = Command::new("g++")
        .arg(&cpp_file)
        .arg("-o")
        .arg(&bin_path)
        .status()
        .expect("Falha ao compilar com g++");

    if !compile_status.success() {
        eprintln!("Falha na compilação de {}", file);
        return;
    }

    // Executa o binário
    let output = Command::new(&bin_path)
        .output()
        .expect("Falha ao executar o binário");

    if output.status.success() {
        println!("Saída: {}", String::from_utf8_lossy(&output.stdout));
    } else {
        eprintln!("Erro: {}", String::from_utf8_lossy(&output.stderr));
    }
}


fn run_c(file: &str) {
    let c_file = path(file);

    if !c_file.exists() {
        eprintln!("Arquivo {} não encontrado!", c_file.display());
        return;
    }

    let bin_name = c_file.file_stem()
        .expect("Nome de arquivo inválido")
        .to_str()
        .expect("Falha na conversão do nome");

    let bin_path = env::current_dir()
        .expect("Falha ao obter o diretório")
        .join(format!("{bin_name}"));

    // Compila com gcc
    let compile_status = Command::new("gcc")
        .arg(&c_file)
        .arg("-o")
        .arg(&bin_path)
        .status()
        .expect("Falha ao compilar com gcc");

    if !compile_status.success() {
        eprintln!("Falha na compilação de {}", file);
        return;
    }

    // Executa o binário compilado
    let output = Command::new(&bin_path)
        .output()
        .expect("Falha ao executar o binário");

    if output.status.success() {
        println!("Saída: {}", String::from_utf8_lossy(&output.stdout));
    } else {
        eprintln!("Erro: {}", String::from_utf8_lossy(&output.stderr));
    }
}

fn run_python(file :&str){

    let _output = Command::new("python3")
    .arg(path(file))
    .output()
    .expect("Erro Caminho Arquivo");

    if _output.status.success(){
        println!("Saída: {}", String::from_utf8_lossy(&_output.stdout));
    } else {
        eprintln!("Erro: {}", String::from_utf8_lossy(&_output.stderr));
    }
}