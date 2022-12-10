use std::fmt;

#[derive(PartialEq, Eq)]
pub enum DataTypes {
    String(String),
    Integer(i64),
    Unknown,
}

impl fmt::Display for DataTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            DataTypes::String(s) => write!(f, "{}", literal_eval(s)),
            DataTypes::Integer(i) => write!(f, "{}", i),
            _ => write!(f, "<Garbage:UnintialisedMemorySpace>"),
        }
    }
}

impl std::ops::Add for DataTypes {
    type Output = Self;
    fn add(self, rhs: DataTypes) -> Self {
        match (self, rhs) {
            (DataTypes::Integer(l), DataTypes::Integer(r)) => DataTypes::Integer(l + r),
            _ => panic!("Incompatible Datatype!"),
        }
    }
}

impl std::ops::Sub for DataTypes {
    type Output = Self;
    fn sub(self, rhs: DataTypes) -> Self {
        match (self, rhs) {
            (DataTypes::Integer(l), DataTypes::Integer(r)) => DataTypes::Integer(l - r),
            _ => panic!("Incompatible Datatype!"),
        }
    }
}

impl std::ops::Mul for DataTypes {
    type Output = Self;
    fn mul(self, rhs: DataTypes) -> Self {
        match (self, rhs) {
            (DataTypes::Integer(l), DataTypes::Integer(r)) => DataTypes::Integer(l * r),
            _ => panic!("Incompatible Datatype!"),
        }
    }
}

impl std::ops::Div for DataTypes {
    type Output = Self;
    fn div(self, rhs: DataTypes) -> Self {
        if rhs == DataTypes::Integer(0) {
            panic!("Division by zero");
        }
        match (self, rhs) {
            (DataTypes::Integer(l), DataTypes::Integer(r)) => DataTypes::Integer(l / r),
            _ => panic!("Incompatible Datatypes used!"),
        }
    }
}

impl std::ops::Rem for DataTypes {
    type Output = Self;
    fn rem(self, rhs: DataTypes) -> Self {
        if rhs == DataTypes::Integer(0) {
            panic!("Division by zero");
        }
        match (self, rhs) {
            (DataTypes::Integer(l), DataTypes::Integer(r)) => DataTypes::Integer(l % r),
            _ => panic!("Incompatible Datatypes used!"),
        }
    }
}

fn literal_eval(data: &str) -> String {
    data.replace("\\n", "\n")
}
