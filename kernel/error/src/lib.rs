/**--------------------------------------------------------------------------------------------------
 *   Copyright (c) OpusCommunity. All rights reserved.
 *   Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------------*/

#[derive(Debug, Clone)]
pub struct WrappedError {
    message: String,
    original: String,
}

impl std::fmt::Display for WrappedError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", self.message, self.original)
    }
}

impl std::error::Error for WrappedError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

pub fn wrapdbg<T, S>(original: T, message: S) -> WrappedError
where
    T: std::fmt::Debug,
    S: Into<String>,
{
    WrappedError {
        message: message.into(),
        original: format!("{:?}", original),
    }
}

pub fn wrap<T, S>(original: T, message: S) -> WrappedError
where
    T: std::fmt::Display,
    S: Into<String>,
{
    WrappedError {
        message: message.into(),
        original: format!("{}", original),
    }
}

macro_rules! makeAnyError {
    ($($e:ident),*) => {

        #[derive(Debug)]
        #[allow(clippy::enum_variant_names)]
        pub enum AnyError {
            $($e($e),)*
        }

        impl std::fmt::Display for AnyError {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match *self {
                    $(AnyError::$e(ref e) => e.fmt(f),)*
                }
            }
        }

        impl std::error::Error for AnyError {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                None
            }
        }

        $(impl From<$e> for AnyError {
            fn from(e: $e) -> AnyError {
                AnyError::$e(e)
            }
        })*
    };
}

makeAnyError!(WrappedError);
