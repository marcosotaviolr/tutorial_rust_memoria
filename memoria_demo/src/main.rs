use chrono::prelude::*;
use std::io::{self, Write};

fn main() {
    println!("=== memória_demo (Stack vs Heap) ===\n");

    // 1) Dados estáticos (literal) -> tipicamente armazenado em .rodata (segmento de dados do executável)
    let welcome: &str = "Bem-vindo ao demo de memória!";

    // 2) Entrada do usuário: String (heap) e parsing (ex: birth_year)
    println!("{}", welcome);
    let name = read_line("Nome do estudante: ");
    let birth_year: i32 = loop {
        let s = read_line("Ano de nascimento (YYYY): ");
        match s.trim().parse() {
            Ok(y) => break y,
            Err(_) => println!("Ano inválido. Tente novamente."),
        }
    };

    // 3) Exemplos de alocações: stack vs heap
    // stack_value: valor escalar (armazenado na stack como parte do frame atual)
    let stack_value: i32 = 12345;

    // heap_box: Box aloca no heap (o Box em si (pointer) fica na stack; o valor apontado fica na heap)
    let heap_box = Box::new(2025i32);

    // name_chars: Vec<char> (estrutura no stack, buffer no heap)
    let name_chars: Vec<char> = name.chars().collect();

    // 4) calcular idade (usa chrono para pegar o ano atual)
    let now = Local::now();
    let current_year = now.year();
    let age = current_year - birth_year;

    println!("\n--- Resultado ---");
    println!("Nome (String)   : {}", name.trim());
    println!("Ano nascimento  : {}", birth_year);
    println!("Ano atual       : {}", current_year);
    println!("Idade aproximada: {} anos\n", age);

    // 5) Mostrar endereços e demonstrar onde cada coisa vive (observacional)
    println!("--- Endereços / Pistas de memória ---");
    println!("&welcome (literal .rodata)      = {:p}", welcome as *const str);
    println!("name (String object on stack)   = {:p}", &name);
    println!("name buffer (heap) as_ptr()     = {:p}", name.as_ptr());
    println!("stack_value (stack)             = {:p}", &stack_value);
    println!("heap_box pointer (on stack)     = {:p}", &heap_box);
    println!("heap_box pointee (heap)         = {:p}", &*heap_box);
    println!("name_chars Vec struct (stack)   = {:p}", &name_chars);
    println!("name_chars buffer (heap)        = {:p}", name_chars.as_ptr());

    // 6) endereço de função (código -> typically in .text)
    println!("example_function (endereço código) = {:p}", example_function as *const ());

    // 7) usar uma função separada para mostrar outro frame de stack (para comparar)
    show_stack_frame(&name, stack_value);

    println!("\n(Dica) Para inspecionar o binário/assembly: veja seção 'Ver binário / assembly' no README.");
}

/// função auxiliar que existe no segmento de código (.text)
fn example_function() {
    // corpo vazio — usamos apenas o endereço
}

fn read_line(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("flush failed");
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("failed to read");
    s
}

fn show_stack_frame(name: &String, local: i32) {
    // esse frame terá seus próprios locais na stack; imprimimos endereços para comparar
    println!("\n--- Dentro de outra função (novo frame na stack) ---");
    println!("param name (referência) addr = {:p}", name);
    println!("local (i32) addr              = {:p}", &local);
}