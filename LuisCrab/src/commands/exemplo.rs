pub fn run(tema: &str){
    match tema {
                "ownership" => {
                   println!("🔧 Exemplo de Ownership:\n\nfn main() {{\n    let s = String::from(\"hello\");\n    let t = s;\n    println!(\"{{}}\", t);\n}}");

                }
                "borrowing" => {
                    println!("🔧 Exemplo de Borrowing:\n\nfn main() {{\n    let s = String::from(\"hello\");\n    print_str(&s);\n}}\n\nfn print_str(s: &String) {{\n    println!(\"{{}}\", s);\n}}");
                }
                _ => println!("❌ Tema não reconhecido."),
     }
}