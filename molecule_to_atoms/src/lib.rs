type Atom = (String, usize);
type Molecule = Vec<Atom>;

#[derive(Debug)]
pub struct ParseError {}

pub fn parse_molecule(s: &str) -> Result<Molecule, ParseError> {
    let mut stack: Vec<String> = vec![];
    let mut molecule: Molecule = vec![];
    for ch in s.chars() {
        if stack.len() == 0 {
            if ch.is_alphabetic() {
                stack.push(ch.to_string());
            } else {
                panic!("Problems");
            }
        } else {
            let last = stack.pop().unwrap();

            if ch.is_alphabetic() && ch.is_lowercase() {
                stack.push(format!("{}{}", last, ch));
            }

            if ch.is_numeric() {
                molecule.push((last, ch.to_digit(10).unwrap() as usize));
                // last will be atoms
            }
        }
    }

    println!("{:?}\n{:?}", stack, molecule);

    for atom in stack {
        molecule.push((atom, 1))
    }

    Ok(molecule)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assert_simple_atom_hidrogen() -> Result<(), ParseError> {
        assert_eq!(parse_molecule("H")?, vec![("H".to_string(), 1 as usize)]);
        Ok(())
    }

    #[test]
    fn assert_simple_atom_magnesium() -> Result<(), ParseError> {
        assert_eq!(parse_molecule("Mg")?, vec![("Mg".to_string(), 1 as usize)]);
        Ok(())
    }

    #[test]
    fn assert_simple_molecule_water() -> Result<(), ParseError> {
        assert_eq!(
            parse_molecule("H2O")?,
            vec![("H".to_string(), 2 as usize), ("O".to_string(), 1)]
        );
        Ok(())
    }
}
