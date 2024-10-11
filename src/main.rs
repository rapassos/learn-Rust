fn main() {

    // Declaração de variáveis

    let imut_x = 5; // Declaração de variável imutável
    println!("O valor de imut_x é: {imut_x}");
    let mut x = 5; // Declaração de variável mutável "mut"
    println!("O valor de x é: {x}");
    x = 6;
    println!("O valor de x é: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // Declaração de constante
    println!("O valor de 'Três horas em segundos' é: {THREE_HOURS_IN_SECONDS}");

    
    // Sombreamento de variáveis
    let sombramento_x = 5;

    let sombramento_x = sombramento_x + 1;

    {
        let sombramento_x = sombramento_x * 2;
        println!("O Valor de sombramento_x dentro deste escopo é: {sombramento_x}");
    }

    println!("O valor de sombramento_x é: {sombramento_x}");


}