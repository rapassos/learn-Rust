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

    // Tipos de variáveis

    // Numéricos
    // Tamanho   Assinalado  Não assinalado     Observação
    // 8 bits 	 i8          u8                 -
    // 16 bits 	 i16         u16                -
    // 32 bits 	 i32         u32                -
    // 64 bits 	 i64         u64                -
    // 128 bits  i128        u128               -
    // arch      isize       usize              Conforme arquitetura do S.O. executando o código

    // Ponto flutuante (números não inteiros)
    // 32 bits   f32
    // 64 bits   f64

    //Operações matemáticas
    // adição
    let soma = 5 + 10;
    println!("O resultado da expressão (5 + 10) é: {soma}");

    // subtração
    let diferenca = 95.5 - 4.3;
    println!("O resultado da expressão (95.5 - 4.3) é: {diferenca}");

    // multiplicação
    let produto = 4 * 30;
    println!("O resultado da expressão (4 * 30) é: {produto}");

    // divisão
    let quociente = 56.7 / 32.2;
    println!("O resultado da expressão (56.7 / 32.2) é: {quociente}");
    let truncado = -5 / 3; // Results in -1
    println!("O resultado da expressão (-5 / 3) é: {truncado} (truncado devido o armazenamento em i32)");

    // resto da divisão
    let resto = 43 % 5;
    println!("O resultado da expressão (43 % 5) é: {resto}");
}
