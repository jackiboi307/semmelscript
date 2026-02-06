use crate::*;

mod funcs;
use funcs::*;

#[macro_export]
macro_rules! get {
    ($runtime:ident, $scope:ident, $name:ident, $type:ident) => {
        expect_type!(
            $scope.get($runtime, stringify!($name))
                .unwrap_or_else(|_| {
                    panic!(concat!("invalid arg: ", stringify!($name)));
                }),
            $type
        )
    }
}

use crate::get;

macro_rules! add {
    ($runtime:ident, $scope:ident,
        $($name:ident($($arg:ident$(,)?)*);)*) => {

        $(
            $scope.define($runtime, stringify!($name),
                Object::Function {
                    func: &Function::Pointer($name) as *const Function,
                    args: vec![$( stringify!($arg).into(), )*],
                }
            );
        )*
    }
}

pub fn init(runtime: &mut Runtime, scope: &mut Scope) {
    // this is such a sexy macro
    add!(runtime, scope,
        println(text);
        print(text);
        call(cmd);
        source(path);
        tostring(value);
    );
}
