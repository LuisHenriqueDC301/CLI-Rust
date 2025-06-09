pub fn run(tema: &str) {
    match tema {
        "ownership" => {
            println!(
                "\
🔧 EXEMPLO DE OWNERSHIP

fn main() {{
    let s = String::from(\"hello\");
    let t = s;  // s transfere a posse da string para t
    println!(\"{{}}\", t);
    // println!(\"{{}}\", s); // ❌ Erro: s não é mais válido!
}}

🧠 A variável `s` não pode mais ser usada depois de mover a posse para `t`.
Isso evita uso duplo de memória.
"
            );
        }
        "borrowing" => {
            println!(
                "\
🔧 EXEMPLO DE BORROWING

fn main() {{
    let s = String::from(\"hello\");
    print_str(&s); // referência imutável
    println!(\"{{}}\", s); // ✅ s ainda é válido
}}

fn print_str(s: &String) {{
    println!(\"{{}}\", s);
}}

🧠 Em vez de transferir a posse, emprestamos `s` com `&s`, permitindo reutilização segura.
"
            );
        }
        _ => println!("❌ Tema não reconhecido. Use: 'ownership' ou 'borrowing'."),
    }
}
