// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

macro_rules! double_newtype {
    ($type:ident, $type_name:expr, $is_valid:ident) => {
        impl std::fmt::Display for $type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl AsRef<Double> for $type {
            fn as_ref(&self) -> &Double {
                &self.0
            }
        }

        impl TryFrom<Double> for $type {
            type Error = crate::error::Error;

            fn try_from(value: Double) -> Result<Self, Self::Error> {
                if $is_valid(value) {
                    Ok(Self(value))
                } else {
                    Err(invalid_value($type_name, &value))
                }
            }
        }
    };
}

macro_rules! string_newtype {
    ($type:ident, $type_name:expr, $is_valid:ident) => {
        impl std::fmt::Display for $type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl AsRef<str> for $type {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }

        impl std::str::FromStr for $type {
            type Err = crate::error::Error;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                if $is_valid(value) {
                    Ok(Self(value.to_string()))
                } else {
                    Err(crate::error::invalid_value($type_name, &value))
                }
            }
        }
    };
}
