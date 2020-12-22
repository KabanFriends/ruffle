//! Dispatch list object representation

use crate::avm2::activation::Activation;
use crate::avm2::class::Class;
use crate::avm2::events::DispatchList;
use crate::avm2::names::{Namespace, QName};
use crate::avm2::object::script_object::{ScriptObjectClass, ScriptObjectData};
use crate::avm2::object::{Object, ObjectPtr, TObject};
use crate::avm2::scope::Scope;
use crate::avm2::string::AvmString;
use crate::avm2::traits::Trait;
use crate::avm2::value::Value;
use crate::avm2::Error;
use crate::{impl_avm2_custom_object, impl_avm2_custom_object_properties};
use gc_arena::{Collect, GcCell, MutationContext};
use std::cell::{Ref, RefMut};

/// Internal representation of dispatch lists as generated by `EventDispatcher`.
///
/// This object is not intended to be constructed, subclassed, or otherwise
/// interacted with by user code. It exists solely to hold event handlers
/// attached to other objects. It's internal construction is subject to change.
/// Objects of this type are only accessed as private properties on
/// `EventDispatcher` instances.
///
/// `DispatchObject` exists primarily due to the generality of the class it
/// services. It has many subclasses, some of which may have different object
/// representations than `ScriptObject`. Furthermore, at least one
/// representation, `StageObject`, requires event dispatch to be able to access
/// handlers on parent objects. These requirements and a few other design goals
/// ruled out the following alternative scenarios:
///
/// 1. Adding event dispatch lists onto other associated data, such as
///    `DisplayObject`s. This would result in bare dispatchers not having a
///    place to store their data.
/// 2. Adding `DispatchList` to the `Value` enum. This would unnecessarily
///    complicate `Value` for an internal type, especially the comparison
///    logic.
/// 3. Making `DispatchObject` the default representation of all
///    `EventDispatcher` classes. This would require adding `DispatchList` to
///    other object representations that need to dispatch events, such as
///    `StageObject`.
#[derive(Clone, Collect, Debug, Copy)]
#[collect(no_drop)]
pub struct DispatchObject<'gc>(GcCell<'gc, DispatchObjectData<'gc>>);

#[derive(Clone, Collect, Debug)]
#[collect(no_drop)]
pub struct DispatchObjectData<'gc> {
    /// Base script object
    base: ScriptObjectData<'gc>,

    /// The dispatch list this object holds.
    dispatch: DispatchList<'gc>,
}

impl<'gc> DispatchObject<'gc> {
    /// Construct an empty dispatch list.
    pub fn empty_list(mc: MutationContext<'gc, '_>) -> Object<'gc> {
        let base = ScriptObjectData::base_new(None, ScriptObjectClass::NoClass);

        DispatchObject(GcCell::allocate(
            mc,
            DispatchObjectData {
                base,
                dispatch: DispatchList::new(),
            },
        ))
        .into()
    }
}

impl<'gc> TObject<'gc> for DispatchObject<'gc> {
    impl_avm2_custom_object!(base);
    impl_avm2_custom_object_properties!(base);

    fn construct(
        &self,
        _activation: &mut Activation<'_, 'gc, '_>,
        _args: &[Value<'gc>],
    ) -> Result<Object<'gc>, Error> {
        Err("Cannot construct internal event dispatcher structures.".into())
    }

    fn derive(
        &self,
        _activation: &mut Activation<'_, 'gc, '_>,
        _class: GcCell<'gc, Class<'gc>>,
        _scope: Option<GcCell<'gc, Scope<'gc>>>,
    ) -> Result<Object<'gc>, Error> {
        Err("Cannot subclass internal event dispatcher structures.".into())
    }

    fn value_of(&self, _mc: MutationContext<'gc, '_>) -> Result<Value<'gc>, Error> {
        Err("Cannot subclass internal event dispatcher structures.".into())
    }

    /// Unwrap this object as a list of event handlers.
    fn as_dispatch(&self) -> Option<Ref<DispatchList<'gc>>> {
        Some(Ref::map(self.0.read(), |r| &r.dispatch))
    }

    /// Unwrap this object as a mutable list of event handlers.
    fn as_dispatch_mut(&self, mc: MutationContext<'gc, '_>) -> Option<RefMut<DispatchList<'gc>>> {
        Some(RefMut::map(self.0.write(mc), |r| &mut r.dispatch))
    }
}
