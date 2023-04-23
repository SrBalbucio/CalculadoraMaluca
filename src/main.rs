#[warn(non_snake_case)]
use std::io;

enum OPERADOR {
    soma,
    substracao,
    divisao,
    multiplicacao,
    none,
}

impl OPERADOR {
    pub fn getSignal(self) -> String {
        match self {
            OPERADOR::soma => String::from("+"),
            OPERADOR::substracao => String::from("-"),
            OPERADOR::divisao => String::from("/"),
            OPERADOR::multiplicacao => String::from("*"),
            OPERADOR::none => todo!(),
        }
    }
    pub fn clone(self)-> OPERADOR{
        match self {
            OPERADOR::soma => OPERADOR::soma,
            OPERADOR::substracao => OPERADOR::substracao,
            OPERADOR::divisao => OPERADOR::divisao,
            OPERADOR::multiplicacao => OPERADOR::multiplicacao,
            OPERADOR::none => todo!(),
        }
    }
  }

struct ContaSecundaria{
    firstNumber: usize,
    operator: OPERADOR,
}

struct Conta {
    firstNumber: usize,
    operator: OPERADOR,
    secondNumber: usize,
    total: usize,
}

impl ContaSecundaria{
    fn new(firstNumber: usize, operator: OPERADOR) -> Self {
        ContaSecundaria {
            firstNumber,
            operator,
        }
    }

    fn clone(self) -> ContaSecundaria {
        ContaSecundaria { firstNumber: self.firstNumber.clone()
            , operator: self.operator.clone() }
    }
}

impl Conta {
    fn new(firstNumber: usize, operator: OPERADOR, secondNumber: usize) -> Self {
        Conta {
            firstNumber,
            operator,
            secondNumber,
            total: 0,
        }
    }

    fn clone(self) -> Conta{
        Conta { firstNumber: self.firstNumber.clone(), operator: self.operator.clone(), secondNumber: self.secondNumber.clone(), total: self.total.clone() }
    }
    fn conta(&mut self) -> &mut usize {

        match self.operator {
            OPERADOR::soma => {
                self.total = self.firstNumber + self.secondNumber;
                println!("{}{}{}={}", self.firstNumber, "+", self.secondNumber, self.total);
            }
            OPERADOR::substracao => {
                self.total = self.firstNumber - self.secondNumber;
                println!("{}{}{}={}", self.firstNumber, "-", self.secondNumber, self.total);
            }
            OPERADOR::divisao => {
                self.total = self.firstNumber / self.secondNumber;
                println!("{}{}{}={}", self.firstNumber, "/", self.secondNumber, self.total);
            }
            OPERADOR::multiplicacao => {
                self.total = self.firstNumber * self.secondNumber;
                println!("{}{}{}={}", self.firstNumber, "*", self.secondNumber, self.total);
            }
            OPERADOR::none => (),
        }
        return &mut self.total;
    }

    fn addToConta(&mut self, calc: ContaSecundaria)-> usize{
        let oldTotal = self.total.clone();
        match calc.operator {
            OPERADOR::soma => {
                self.total = self.total + calc.firstNumber;
            }
            OPERADOR::substracao => {
                self.total = self.total - calc.firstNumber;
            }
            OPERADOR::divisao => {
                self.total = self.total / calc.firstNumber;
            }
            OPERADOR::multiplicacao => {
                self.total = self.total * calc.firstNumber;
            }
            OPERADOR::none => (),
        }
        println!("{}{}{}={}", oldTotal, calc.operator.getSignal(), calc.firstNumber, self.total);
        return self.total;
    }
}

fn main() {
    println!("Esta é calculadora maluca!");
    let mut cat: u8 = 0;
    let mut step: u8 = 0;
    while true {
        let mut calcs: Vec<ContaSecundaria> = Vec::new();
        let mut command: String = String::new();
        println!("Deseja fazer uma conta em forma de questionario ou apenas enviar uma conta completa (está em beta)?");
        ler(&mut command);
        println!("{}", command.as_str());
        if (command.contains("questionario")) {
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
            if (command.contains("sim")) {
                let mut addendo: bool = true;
                while addendo {
                    let mut nnumber: u32;
                    let mut ooperator: OPERADOR = OPERADOR::none;
                    println!("Qual é o novo número?");
                    ler(&mut command);
                    nnumber = command.trim().parse::<u32>().unwrap();
                    println!("Perfeito, qual que é o operador a ser usado nesta nova conta?");
                    ler(&mut command);
                    if (command.eq_ignore_ascii_case("soma") || command.contains("+")) {
                        ooperator = OPERADOR::soma;
                    } else if (command.eq_ignore_ascii_case("substracao") || command.contains("-"))
                    {
                        operator = OPERADOR::substracao;
                    } else if (command.eq_ignore_ascii_case("divisao") || command.contains("/")) {
                        ooperator = OPERADOR::divisao;
                    } else if (command.eq_ignore_ascii_case("multiplicacao")
                        || command.contains("*"))
                    {
                        ooperator = OPERADOR::multiplicacao;
                    }

                    calcs.push(ContaSecundaria { firstNumber: nnumber.try_into().unwrap(), operator: ooperator });
                    println!("Deseja adicionar mais operações a esta conta?");
                    ler(&mut command);
                    if(command.contains("não")){
                        addendo = false;
                    }
                }
            }
            let mut conta = Conta::new(
                number.try_into().unwrap(),
                operator,
                number2.try_into().unwrap(),
            );
            conta.conta();

            for calc in calcs {
                conta.addToConta(calc);
            }
            println!("Resultado: {}", conta.total);
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
