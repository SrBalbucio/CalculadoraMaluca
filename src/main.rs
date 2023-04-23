#[warn(non_snake_case)]
use std::io;

enum OPERADOR {
    soma,
    substracao,
    divisao,
    multiplicacao,
    none,
}

struct Conta {
    firstNumber: usize,
    operator: OPERADOR,
    secondNumber: usize,
}

impl Conta {
    fn new(firstNumber: usize, operator: OPERADOR, secondNumber: usize) -> Self {
        Conta {
            firstNumber,
            operator,
            secondNumber,
        }
    }
    fn conta(&self) -> usize {
        match self.operator {
            OPERADOR::soma => {
                return self.firstNumber + self.secondNumber;
            }
            OPERADOR::substracao => {
                return self.firstNumber - self.secondNumber;
            }
            OPERADOR::divisao => {
                return self.firstNumber / self.secondNumber;
            }
            OPERADOR::multiplicacao => {
                return self.firstNumber * self.secondNumber;
            }
            OPERADOR::none => 0,
        }
    }
}

fn main() {
    println!("Esta é calculadora maluca!");
    let mut cat: u8 = 0;
    let mut step: u8 = 0;
    while true {
        let mut command: String = String::new();
        println!("Deseja fazer uma conta em forma de questionario ou apenas enviar uma conta completa (está em beta)?");
        ler(&mut command);
        println!("{}", command.as_str());
        if (command.as_str().ne("questionario")) {
            let mut calcs: Vec<Conta> = Vec::new();
            let mut number: u32;
            let mut operator: OPERADOR = OPERADOR::none;
            let mut number2: u32;
            let mut total: usize;
            println!("OK!");
            println!("Qual é o primeiro número da conta?");
            ler(&mut command);
            number = command.trim().parse::<u32>().unwrap();
            println!("Perfeito, qual que é o operador?");
            ler(&mut command);
            if (command.eq_ignore_ascii_case("soma") || command.contains("+")) {
                operator = OPERADOR::soma;
            } else if (command.eq_ignore_ascii_case("substracao") || command.contains("-")) {
                operator = OPERADOR::substracao;
            } else if (command.eq_ignore_ascii_case("divisao") || command.contains("/")) {
                operator = OPERADOR::divisao;
            } else if (command.eq_ignore_ascii_case("multiplicacao") || command.contains("*")) {
                operator = OPERADOR::multiplicacao;
            }
            println!("Qual é o segundo número da conta?");
            ler(&mut command);
            number2 = command.trim().parse::<u32>().unwrap();
            println!("Deseja adicionar mais operações a esta conta?");
            ler(&mut command);
            if (command.eq_ignore_ascii_case("sim")) {
                println!("Esta operação ainda não existe :C");
            }
            let conta =Conta::new(number.try_into().unwrap(), operator, number2.try_into().unwrap());
            let total = conta.conta();
            println!("Resultado: {}",total);
        } else if (command.eq_ignore_ascii_case("completa")) {
        } else {
            println!("Essa opção infelizmente não existe, use: questionario ou completa.");
        }
    }
}

fn ler(cmd: &mut String) {
    cmd.clear();
    io::stdin()
        .read_line(cmd)
        .expect("Não foi possível verificar a tua conta!");
}
