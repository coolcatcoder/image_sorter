#[macro_export]
macro_rules! system_set {
    ($identifier:ident) => {
        #[derive(SystemSet, Hash, Debug, PartialEq, Eq, Clone)]
        pub struct $identifier;
    };
}

#[macro_export]
macro_rules! bundle_effect {
    ($($any:tt)*) => {
        const _:() = {
            use ::bevy::ecs::bundle::Bundle as Effect;
            use ::bevy::ecs::component::ComponentsRegistrator;
            use ::bevy::ecs::component::ComponentId;
            use ::bevy::ecs::component::Components;

            // SAFETY: Defers to ()'s Bundle implementation.
            unsafe $($any)* {
                #[allow(refining_impl_trait)]
                fn component_ids(
                    _components: &mut ComponentsRegistrator,
                ) -> core::iter::Empty<ComponentId> {
                    core::iter::empty()
                }
                fn get_component_ids(
                    components: &Components,
                ) -> impl Iterator<Item = Option<ComponentId>> {
                    <() as Bundle>::get_component_ids(components)
                }
            }
        };

        const _:() = {
            use ::core::mem::MaybeUninit;
            use ::core::mem::forget;

            use ::bevy::{
                ecs::{
                    bundle::DynamicBundle as Effect,
                    ptr::{
                        MovingPtr,
                        OwningPtr,
                    },
                    component::StorageType,
                },
                prelude::{
                    EntityWorldMut,
                    World,
                },
            };

            $($any)* {
                type Effect = Self;

                // SAFETY: There are no components and thus we call func zero times.
                unsafe fn get_components(
                    ptr: MovingPtr<'_, Self>,
                    _func: &mut impl FnMut(StorageType, OwningPtr<'_>),
                ) {
                    forget(ptr);
                }

                // SAFETY: ptr is not dropped due to being forgotten in the previous function.
                unsafe fn apply_effect(
                    ptr: MovingPtr<'_, MaybeUninit<Self>>,
                    entity: &mut EntityWorldMut<'_>,
                ) {
                    let value = ptr.read();
                    let value = unsafe {
                        value.assume_init()
                    };

                    if entity.is_despawned() {
                        return;
                    }

                    let entity_id = entity.id();
                    entity.world_scope(|world: &mut World| {
                        if world.run_system_cached_with(Self::effect, (entity_id, value)).is_err() {
                            todo!("A bundle effect system failed to run and we don't yet have a handler.");
                        }
                    });
                }
            }
        };
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
