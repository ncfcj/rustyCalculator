//! Núcleo da calculadora — pura lógica, sem UI.
//!
//! Separar lógica da interface facilita testar e aprender:
//! aqui só mexemos em números e operações.

/// Operações binárias suportadas.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operation {
    pub fn symbol(self) -> &'static str {
        match self {
            Self::Add => "+",
            Self::Subtract => "−",
            Self::Multiply => "×",
            Self::Divide => "÷",
        }
    }

    pub fn from_str_op(s: &str) -> Result<Self, String> {
        match s {
            "add" | "+" => Ok(Self::Add),
            "subtract" | "-" | "−" => Ok(Self::Subtract),
            "multiply" | "*" | "×" => Ok(Self::Multiply),
            "divide" | "/" | "÷" => Ok(Self::Divide),
            other => Err(format!("Operação desconhecida: {other}")),
        }
    }

    fn apply(self, left: f64, right: f64) -> Result<f64, String> {
        match self {
            Self::Add => Ok(left + right),
            Self::Subtract => Ok(left - right),
            Self::Multiply => Ok(left * right),
            Self::Divide => {
                if right == 0.0 {
                    Err("Divisão por zero".into())
                } else {
                    Ok(left / right)
                }
            }
        }
    }
}

/// Estado interno da calculadora (estilo “máquina de estados” simples).
#[derive(Debug, Default)]
pub struct Calculator {
    /// Texto mostrado no display (o que o usuário está digitando).
    display: String,
    /// Valor guardado antes de escolher uma operação.
    left: Option<f64>,
    /// Operação pendente (ex.: usuário apertou `+`).
    pending: Option<Operation>,
    /// Depois de `=`, o próximo dígito começa um número novo.
    just_evaluated: bool,
    /// Mensagem de erro (ex.: divisão por zero).
    error: Option<String>,
}

impl Calculator {
    pub fn new() -> Self {
        Self {
            display: "0".into(),
            ..Self::default()
        }
    }

    pub fn display(&self) -> &str {
        self.error.as_deref().unwrap_or(&self.display)
    }

    pub fn pending_op(&self) -> Option<Operation> {
        self.pending
    }

    pub fn input_digit(&mut self, digit: char) {
        debug_assert!(digit.is_ascii_digit());
        self.clear_error();

        if self.just_evaluated {
            self.display = digit.to_string();
            self.just_evaluated = false;
            return;
        }

        if self.display == "0" {
            self.display = digit.to_string();
        } else {
            self.display.push(digit);
        }
    }

    pub fn input_decimal(&mut self) {
        self.clear_error();

        if self.just_evaluated {
            self.display = "0.".into();
            self.just_evaluated = false;
            return;
        }

        if !self.display.contains('.') {
            self.display.push('.');
        }
    }

    pub fn set_operation(&mut self, op: Operation) {
        self.clear_error();

        let current = match self.parse_display() {
            Ok(v) => v,
            Err(e) => {
                self.error = Some(e);
                return;
            }
        };

        // Encadeia: se já havia operação pendente, calcula antes.
        if let (Some(left), Some(pending)) = (self.left, self.pending) {
            match pending.apply(left, current) {
                Ok(result) => {
                    self.display = format_number(result);
                    self.left = Some(result);
                }
                Err(e) => {
                    self.error = Some(e);
                    self.reset_partial();
                    return;
                }
            }
        } else {
            self.left = Some(current);
        }

        self.pending = Some(op);
        self.just_evaluated = true;
    }

    pub fn evaluate(&mut self) {
        self.clear_error();

        let (Some(left), Some(op)) = (self.left, self.pending) else {
            return;
        };

        let right = match self.parse_display() {
            Ok(v) => v,
            Err(e) => {
                self.error = Some(e);
                return;
            }
        };

        match op.apply(left, right) {
            Ok(result) => {
                self.display = format_number(result);
                self.left = None;
                self.pending = None;
                self.just_evaluated = true;
            }
            Err(e) => {
                self.error = Some(e);
                self.reset_partial();
            }
        }
    }

    pub fn clear_all(&mut self) {
        *self = Self::new();
    }

    pub fn backspace(&mut self) {
        self.clear_error();
        if self.just_evaluated {
            return;
        }
        self.display.pop();
        if self.display.is_empty() || self.display == "-" {
            self.display = "0".into();
        }
    }

    pub fn toggle_sign(&mut self) {
        self.clear_error();
        if let Ok(v) = self.parse_display() {
            self.display = format_number(-v);
            self.just_evaluated = false;
        }
    }

    fn parse_display(&self) -> Result<f64, String> {
        self.display
            .parse::<f64>()
            .map_err(|_| "Número inválido".into())
    }

    fn clear_error(&mut self) {
        self.error = None;
    }

    fn reset_partial(&mut self) {
        self.left = None;
        self.pending = None;
        self.just_evaluated = true;
    }
}

fn format_number(value: f64) -> String {
    if value.fract() == 0.0 && value.abs() < 1e15 {
        format!("{value:.0}")
    } else {
        let s = format!("{value:.10}");
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addition() {
        let mut c = Calculator::new();
        c.input_digit('1');
        c.input_digit('2');
        c.set_operation(Operation::Add);
        c.input_digit('3');
        c.evaluate();
        assert_eq!(c.display(), "15");
    }

    #[test]
    fn divide_by_zero() {
        let mut c = Calculator::new();
        c.input_digit('8');
        c.set_operation(Operation::Divide);
        c.input_digit('0');
        c.evaluate();
        assert_eq!(c.display(), "Divisão por zero");
    }
}
