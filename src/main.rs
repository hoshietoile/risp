mod modules;

// TODO: need to implement eval with real struct.
fn eval() -> () {}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test() {}

    #[test]
    fn eval_basic_atom() {}

    #[test]
    fn eval_calc() {}

    #[test]
    fn eval_symbol() {}

    #[test]
    fn eval_func() {}
}
