/*!
One-line description.

More detailed description, with

# Example

 */

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

// macro_rules! builder_property {
//     ($v:vis $name:ident, $rtype:ty, $ptype:ty) => {
//         paste! {
//             $v fn $name(&self) -> &$rtype {
//                 &self.$name
//             }

//             $v fn [<set_ $name>](mut self, $name: &$ptype) -> Self {
//                 self.$name = $name;
//                 self
//             }
//         }
//     };
//     ($v:vis $name:ident, $type:ty => $delegate:expr) => {
//         paste! {
//             $v fn $name(&self) -> &$type {
//                 &self.$delegate.$name
//             }

//             $v fn [<set_ $name>](mut self, $name: &$type) -> Self {
//                 self.$delegate.$name = $name;
//                 self
//             }
//         }
//     };
//     ($v:vis $name:ident, $type:ty) => {
//         paste! {
//             $v fn $name(&self) -> &$type {
//                 &self.$name
//             }

//             $v fn [<set_ $name>](mut self, $name: &$type) -> Self {
//                 self.$name = $name;
//                 self
//             }
//         }
//     };
// }

// macro_rules! vec_builder_property {
//     ($v:vis $name:ident, $type:ty => $delegate:expr) => {
//         paste! {
//             $v fn [<$name s>](&self) -> impl Iterator<Item = &$type> {
//                 self.$delegate.[<$name s>].iter()
//             }

//             $v fn [<add_ $name>](mut self, $name: $type) -> Self {
//                 self.$delegate.[<$name s>].push($name);
//                 self
//             }
//         }
//     };
//     ($v:vis $name:ident, $type:ty) => {
//         paste! {
//             $v fn [<$name s>](&self) -> impl Iterator<Item = &$type> {
//                 self.[<$name s>].iter()
//             }

//             $v fn [<add_ $name>](mut self, $name: $type) -> Self {
//                 self.[<$name s>].push($name);
//                 self
//             }
//         }
//     };
// }

// macro_rules! optional_builder_property {
//     ($v:vis $name:ident, $type:ty => $delegate:expr) => {
//         paste! {
//             $v fn $name(&self) -> Option<&$type> {
//                 self.$delegate.$name.as_ref()
//             }

//             $v fn [<set_ $name>](mut self, $name: $type) -> Self {
//                 self.$delegate.$name = Some($name);
//                 self
//             }

//             $v fn [<unset_ $name>](mut self) -> Self {
//                 self.$delegate.$name = None;
//                 self
//             }
//         }
//     };
//     ($v:vis $name:ident, $type:ty) => {
//         paste! {
//             $v fn $name(&self) -> Option<&$type> {
//                 self.$name.as_ref()
//             }

//             $v fn [<set_ $name>](mut self, $name: $type) -> Self {
//                 self.$name = Some($name);
//                 self
//             }

//             $v fn [<unset_ $name>](mut self) -> Self {
//                 self.$name = None;
//                 self
//             }
//         }
//     };
//     ($v:vis string $name:ident => $delegate:expr) => {
//         paste! {
//             $v fn $name(&self) -> Option<&String> {
//                 self.$delegate.$name.as_ref()
//             }

//             $v fn [<set_ $name>](mut self, $name: &str) -> Self {
//                 self.$delegate.$name = Some($name.to_owned());
//                 self
//             }

//             $v fn [<unset_ $name>](mut self) -> Self {
//                 self.$delegate.$name = None;
//                 self
//             }
//         }
//     };
//     ($v:vis string $name:ident) => {
//         paste! {
//             $v fn $name(&self) -> Option<&String> {
//                 self.$name.as_ref()
//             }

//             $v fn [<set_ $name>](mut self, $name: &str) -> Self {
//                 self.$name = Some($name.to_owned());
//                 self
//             }

//             $v fn [<unset_ $name>](mut self) -> Self {
//                 self.$name = None;
//                 self
//             }
//         }
//     };
// }

// macro_rules! optional_property {
//     ($name:ident, $type:ty) => {
//         paste! {
//             pub fn $name(&self) -> Option<&$type> {
//                 self.$name.as_ref()
//             }

//             pub fn [<set_ $name>](&mut self, $name: $type) -> &mut Self {
//                 self.$name = Some($name);
//                 self
//             }

//             pub fn [<unset $name>](&mut self) -> &mut Self {
//                 self.$name = None;
//                 self
//             }
//         }
//     };
// }

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
