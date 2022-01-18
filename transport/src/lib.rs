pub mod instruction_value;
pub mod interface;
pub use instruction_value::{TransportValue, Value};

#[macro_export]
macro_rules! export_interface {
    ($register:expr) => {
        #[doc(hidden)]
        #[no_mangle]
        pub static entrypoint: $crate::interface::InstructionInterface =
            $crate::interface::InstructionInterface {
                register: $register,
            };
    };
}
