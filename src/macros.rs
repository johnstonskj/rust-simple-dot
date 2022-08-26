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


macro_rules! styled_common_impl {
    ($type:ty) => {
        impl Default for $type {
            fn default() -> Self {
                Self(Default::default())
            }
        }

        impl Display for $type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                if self.0.is_empty() {
                    write!(f, "[]")
                } else {
                    write!(
                        f,
                        "[ {} ]",
                        self.0
                            .iter()
                            .map(Attribute::to_string)
                            .collect::<Vec<String>>()
                            .join("; ")
                    )
                }
            }
        }

        impl AsRef<Vec<Attribute>> for $type {
            fn as_ref(&self) -> &Vec<Attribute> {
                &self.0
            }
        }

        impl Attributes for $type {}

        impl $type {
            fn push(self, attribute: Attribute) -> Self
            where
                Self: Sized,
            {
                let mut self_mut = self;
                self_mut.0.push(attribute);
                self_mut
            }
        }
    };
}

macro_rules! attribute_setter {
    ($name:ident, $variant_and_type:ident) => {
        pub fn $name(self, $name: $variant_and_type) -> Self {
            self.push(Attribute::$variant_and_type($name))
        }
    };
    ($name:ident, $variant:ident, $type:ty) => {
        pub fn $name(self, $name: $type) -> Self {
            self.push(Attribute::$variant($name))
        }
    };
}

macro_rules! style_attribute_setter {
    ($style_variant:ident, $style_type:ty) => {
        pub fn style(self, styles: Vec<$style_type>) -> Self {
            self.push(Attribute::Style(
                styles
                    .into_iter()
                    .map(|s| Style::$style_variant(s))
                    .collect(),
            ))
        }
    };
}

macro_rules! boolean_attribute_setter {
    ($name_true:ident, $variant:ident) => {
        pub fn $name_true(self) -> Self {
            self.push(Attribute::$variant(true))
        }
    };
}
