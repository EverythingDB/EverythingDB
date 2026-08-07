macro_rules! has_property {
    (
        $struct:ty => {
            $(
                [$trait:ident, $function:ident, $return:ty, $($path:tt)+]
            ),* $(,)?
        }
    ) => {
        $(
            impl $trait for $struct {
                fn $function(&self) -> $return {
                    self.$($path)+
                }
            }
        )*
    };
}