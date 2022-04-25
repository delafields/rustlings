// structs1.rs
// Address all the TODOs to make the tests pass!

// SOLVED added struct below
struct ColorClassicStruct {
    name: String,
    hex: String
}

// SOLVED added line below
struct ColorTupleStruct(String, String);

#[derive(Debug)]
struct UnitStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // TODO: Instantiate a classic c struct!
        // SOLVED added struct below
        let green = ColorClassicStruct {
            name: String::from("green"),
            hex: String::from("#00FF00")
        };

        assert_eq!(green.name, "green");
        assert_eq!(green.hex, "#00FF00");
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct!
        // SOLVED added line below
        let green = ColorTupleStruct (String::from("green"), String::from("#00FF00"));

        assert_eq!(green.0, "green");
        assert_eq!(green.1, "#00FF00");
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit struct!
        // SOLVED added line below
        let unit_struct = UnitStruct;
        let message = format!("{:?}s are fun!", unit_struct);

        assert_eq!(message, "UnitStructs are fun!");
    }
}
