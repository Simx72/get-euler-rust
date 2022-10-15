use factorial::Factorial;
use inquire::{validator::Validation, Select, Text};
use std::vec;

fn get_e_cuotas(c: u32) -> f64 {
    let c: f64 = c.into(); // cuotas
    return (1.0 + 1.0 / c).powf(c);
}
fn get_e_series(n: u32) -> f64 {
    let mut e = 0.0;
    for x in 0..n {
        let xf = x as f64;
        let fac = (2 * x).factorial() as f64;
        let serie = (1.0 - 2.0 * xf) / fac;
        // println!("serie: (1 - 2 * {xf}) / 2 * {xf}!\t={serie}");
        e += serie;
    }
    return 1.0 / e;
}

fn calc_accuracy(s1: &str, s2: &str) -> f32 {
    let mut s1 = s1;
    let mut s2 = s2;

    if s1.len() > s2.len() {
        let s3 = s2;
        s2 = s1;
        s1 = s3;
    };

    let total = s1.len();
    let mut matches = 0;

    let s2 = s2.chars().collect::<Vec<char>>();
    for (i, ch1) in s1.chars().enumerate() {
        let ch2 = *s2.get(i).expect("error");
        if ch1 == ch2 {
            // println!("{ch1} = {ch2}");
            matches += 1;
        } else {
            // println!("{ch1} ≠ {ch2}");
            break;
        }
    }
    let accuracy: f32 = (matches as f32 / total as f32) * 100.0;
    return accuracy;
}

fn main() {
    let metodos = vec![
        "Metodo de cuotas: (1 + 1/n)^n",
        "Metodo de series: 1 - 1/1! + 1/2!...",
    ];

    let method = Select::new("Con que metodo quieres calcular e", metodos.clone())
        .prompt()
        .expect("Ocurrio un error");

    println!("{}", method);

    let res: f64;
    let validator = |input: &str| {
        if input.chars().count() > 9 {
            return Ok(Validation::Invalid(
                "You're only allowed 9 characters.".into(),
            ));
        } else {
            let is_string_numeric = |str: String| -> bool {
                for c in str.chars() {
                    if !c.is_numeric() {
                        return false;
                    }
                }
                return true;
            };
            if is_string_numeric(input.to_string()) {
                return Ok(Validation::Valid);
            } else {
                return Ok(Validation::Invalid(
                    "You have to enter an integer number".into(),
                ));
            }
        }
    };

    if metodos.starts_with(&[method]) {
        let cuotas = Text::new("Numero de cuotas:")
            .with_validator(validator)
            .prompt()
            .expect("ERROR: No se pudo leer la entrada");

        let cuotas: u32 = cuotas.trim().parse().expect("Not A Number");

        println!("Calculando {} cuotas...", cuotas);

        res = get_e_cuotas(cuotas);
    } else {
        let series = Text::new("Numero de series:")
            .with_validator(validator)
            .prompt()
            .expect("ERROR: No se pudo leer la entrada");

        let series: u32 = series.trim().parse().expect("Not A Number");

        println!("Calculando {} series...", series);

        res = get_e_series(series);
    }

    let check = res.clone().to_string();

    let check = String::from_iter(check.matches(char::is_numeric).into_iter());
    println!("Resultado: {}", res);

    let acc = calc_accuracy(&check, "27182818284590452".into());

    println!("accuracy of {}%", acc);
}
