use crate::*;
use crate::runtime::*;

mod funcs;
use funcs::*;

#[macro_export]
macro_rules! get {
    ($runtime:ident, $scope:ident, $name:ident, $type:ident) => {
        unsafe {
            (*expect_type!($scope.get_from($runtime, stringify!($name))?, $type)).clone()
        }
    }
}

use crate::get;

macro_rules! add {
    ($runtime:ident, $scope:ident, $name:ident, $func:expr, [$($arg:ident$(,)?)*]) => {
        $scope.add_in($runtime, stringify!($name),
            Object::Function {
                func: &Function::Pointer($func) as *const Function,
                args: vec![$( stringify!($arg).into(), )*],
            }
        );
    }
}

pub fn init(runtime: &mut Runtime, scope: &mut Scope) {
    add!(runtime, scope, println, println, [text]);
}
