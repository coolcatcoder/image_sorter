#[macro_export]
macro_rules! setup {
    () => {
        #[derive(SystemSet, Hash, Debug, PartialEq, Eq, Clone)]
        pub struct Setup;
    };
}

#[macro_export]
macro_rules! plugin {
    // Outside of the function.
    (
        [outside]
        mod $target:path;
    ) => {
        mod $target;
    };

    (
        [outside]
        plugin $target:path;
    ) => {};

    (
        [outside]
        system $target:path => $schedule:expr;
    ) => {};

    // Inside of the function.
    (
        [inside]
        mod $target:path;
    ) => {};

    (
        [inside]
        plugin $target:path;
    ) => {};

    (
        [inside]
        system $target:path => $schedule:expr;
    ) => {};

    // UI
    (
        $(
            $keyword:ident $target:path $(=> $schedule:expr)?;
        )*
    ) => {
        $(
            $crate::plugin! {
                [outside]
                $keyword $target $(=> $schedule)?;
            }
        )*

        pub fn plugin(app: &mut ::bevy::prelude::App) {
            $(
                $crate::plugin! {
                    [inside]
                    $keyword $target $(=> $schedule)?;
                }
            )*
        }
    };
}
