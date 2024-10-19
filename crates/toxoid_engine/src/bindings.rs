#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod toxoid {
        #[allow(dead_code)]
        pub mod engine {
            #[allow(dead_code, clippy::all)]
            pub mod ecs {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                pub type EcsEntityT = u64;
                #[repr(u8)]
                #[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
                pub enum MemberType {
                    U8T,
                    U16T,
                    U32T,
                    U64T,
                    I8T,
                    I16T,
                    I32T,
                    I64T,
                    F32T,
                    F64T,
                    BoolT,
                    StringT,
                    ArrayT,
                    U32arrayT,
                    F32arrayT,
                }
                impl ::core::fmt::Debug for MemberType {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        match self {
                            MemberType::U8T => f.debug_tuple("MemberType::U8T").finish(),
                            MemberType::U16T => {
                                f.debug_tuple("MemberType::U16T").finish()
                            }
                            MemberType::U32T => {
                                f.debug_tuple("MemberType::U32T").finish()
                            }
                            MemberType::U64T => {
                                f.debug_tuple("MemberType::U64T").finish()
                            }
                            MemberType::I8T => f.debug_tuple("MemberType::I8T").finish(),
                            MemberType::I16T => {
                                f.debug_tuple("MemberType::I16T").finish()
                            }
                            MemberType::I32T => {
                                f.debug_tuple("MemberType::I32T").finish()
                            }
                            MemberType::I64T => {
                                f.debug_tuple("MemberType::I64T").finish()
                            }
                            MemberType::F32T => {
                                f.debug_tuple("MemberType::F32T").finish()
                            }
                            MemberType::F64T => {
                                f.debug_tuple("MemberType::F64T").finish()
                            }
                            MemberType::BoolT => {
                                f.debug_tuple("MemberType::BoolT").finish()
                            }
                            MemberType::StringT => {
                                f.debug_tuple("MemberType::StringT").finish()
                            }
                            MemberType::ArrayT => {
                                f.debug_tuple("MemberType::ArrayT").finish()
                            }
                            MemberType::U32arrayT => {
                                f.debug_tuple("MemberType::U32arrayT").finish()
                            }
                            MemberType::F32arrayT => {
                                f.debug_tuple("MemberType::F32arrayT").finish()
                            }
                        }
                    }
                }
                impl MemberType {
                    #[doc(hidden)]
                    pub unsafe fn _lift(val: u8) -> MemberType {
                        if !cfg!(debug_assertions) {
                            return ::core::mem::transmute(val);
                        }
                        match val {
                            0 => MemberType::U8T,
                            1 => MemberType::U16T,
                            2 => MemberType::U32T,
                            3 => MemberType::U64T,
                            4 => MemberType::I8T,
                            5 => MemberType::I16T,
                            6 => MemberType::I32T,
                            7 => MemberType::I64T,
                            8 => MemberType::F32T,
                            9 => MemberType::F64T,
                            10 => MemberType::BoolT,
                            11 => MemberType::StringT,
                            12 => MemberType::ArrayT,
                            13 => MemberType::U32arrayT,
                            14 => MemberType::F32arrayT,
                            _ => panic!("invalid enum discriminant"),
                        }
                    }
                }
                #[derive(Clone)]
                pub struct ComponentDesc {
                    pub name: _rt::String,
                    pub member_names: _rt::Vec<_rt::String>,
                    pub member_types: _rt::Vec<u8>,
                }
                impl ::core::fmt::Debug for ComponentDesc {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("ComponentDesc")
                            .field("name", &self.name)
                            .field("member-names", &self.member_names)
                            .field("member-types", &self.member_types)
                            .finish()
                    }
                }
                /// resource entity {
                /// constructor();
                /// }
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct Component {
                    handle: _rt::Resource<Component>,
                }
                type _ComponentRep<T> = Option<T>;
                impl Component {
                    /// Creates a new resource from the specified representation.
                    ///
                    /// This function will create a new resource handle by moving `val` onto
                    /// the heap and then passing that heap pointer to the component model to
                    /// create a handle. The owned handle is then returned as `Component`.
                    pub fn new<T: GuestComponent>(val: T) -> Self {
                        Self::type_guard::<T>();
                        let val: _ComponentRep<T> = Some(val);
                        let ptr: *mut _ComponentRep<T> = _rt::Box::into_raw(
                            _rt::Box::new(val),
                        );
                        unsafe { Self::from_handle(T::_resource_new(ptr.cast())) }
                    }
                    /// Gets access to the underlying `T` which represents this resource.
                    pub fn get<T: GuestComponent>(&self) -> &T {
                        let ptr = unsafe { &*self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    /// Gets mutable access to the underlying `T` which represents this
                    /// resource.
                    pub fn get_mut<T: GuestComponent>(&mut self) -> &mut T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_mut().unwrap()
                    }
                    /// Consumes this resource and returns the underlying `T`.
                    pub fn into_inner<T: GuestComponent>(self) -> T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.take().unwrap()
                    }
                    #[doc(hidden)]
                    pub unsafe fn from_handle(handle: u32) -> Self {
                        Self {
                            handle: _rt::Resource::from_handle(handle),
                        }
                    }
                    #[doc(hidden)]
                    pub fn take_handle(&self) -> u32 {
                        _rt::Resource::take_handle(&self.handle)
                    }
                    #[doc(hidden)]
                    pub fn handle(&self) -> u32 {
                        _rt::Resource::handle(&self.handle)
                    }
                    #[doc(hidden)]
                    fn type_guard<T: 'static>() {
                        use core::any::TypeId;
                        static mut LAST_TYPE: Option<TypeId> = None;
                        unsafe {
                            assert!(! cfg!(target_feature = "atomics"));
                            let id = TypeId::of::<T>();
                            match LAST_TYPE {
                                Some(ty) => {
                                    assert!(
                                        ty == id, "cannot use two types with this resource type"
                                    )
                                }
                                None => LAST_TYPE = Some(id),
                            }
                        }
                    }
                    #[doc(hidden)]
                    pub unsafe fn dtor<T: 'static>(handle: *mut u8) {
                        Self::type_guard::<T>();
                        let _ = _rt::Box::from_raw(handle as *mut _ComponentRep<T>);
                    }
                    fn as_ptr<T: GuestComponent>(&self) -> *mut _ComponentRep<T> {
                        Component::type_guard::<T>();
                        T::_resource_rep(self.handle()).cast()
                    }
                }
                /// A borrowed version of [`Component`] which represents a borrowed value
                /// with the lifetime `'a`.
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct ComponentBorrow<'a> {
                    rep: *mut u8,
                    _marker: core::marker::PhantomData<&'a Component>,
                }
                impl<'a> ComponentBorrow<'a> {
                    #[doc(hidden)]
                    pub unsafe fn lift(rep: usize) -> Self {
                        Self {
                            rep: rep as *mut u8,
                            _marker: core::marker::PhantomData,
                        }
                    }
                    /// Gets access to the underlying `T` in this resource.
                    pub fn get<T: GuestComponent>(&self) -> &T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    fn as_ptr<T: 'static>(&self) -> *mut _ComponentRep<T> {
                        Component::type_guard::<T>();
                        self.rep.cast()
                    }
                }
                unsafe impl _rt::WasmResource for Component {
                    #[inline]
                    unsafe fn drop(_handle: u32) {
                        #[cfg(not(target_arch = "wasm32"))]
                        unreachable!();
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(wasm_import_module = "[export]toxoid:engine/ecs")]
                            extern "C" {
                                #[link_name = "[resource-drop]component"]
                                fn drop(_: u32);
                            }
                            drop(_handle);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_constructor_component_cabi<T: GuestComponent>(
                    arg0: *mut u8,
                    arg1: usize,
                    arg2: *mut u8,
                    arg3: usize,
                    arg4: *mut u8,
                    arg5: usize,
                ) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    let base4 = arg2;
                    let len4 = arg3;
                    let mut result4 = _rt::Vec::with_capacity(len4);
                    for i in 0..len4 {
                        let base = base4.add(i * 8);
                        let e4 = {
                            let l1 = *base.add(0).cast::<*mut u8>();
                            let l2 = *base.add(4).cast::<usize>();
                            let len3 = l2;
                            let bytes3 = _rt::Vec::from_raw_parts(l1.cast(), len3, len3);
                            _rt::string_lift(bytes3)
                        };
                        result4.push(e4);
                    }
                    _rt::cabi_dealloc(base4, len4 * 8, 4);
                    let len5 = arg5;
                    let result6 = Component::new(
                        T::new(ComponentDesc {
                            name: _rt::string_lift(bytes0),
                            member_names: result4,
                            member_types: _rt::Vec::from_raw_parts(
                                arg4.cast(),
                                len5,
                                len5,
                            ),
                        }),
                    );
                    (result6).take_handle() as i32
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_get_id_cabi<T: GuestComponent>(
                    arg0: *mut u8,
                ) -> i64 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::get_id(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    _rt::as_i64(result0)
                }
                pub trait Guest {
                    type Component: GuestComponent;
                }
                pub trait GuestComponent: 'static {
                    #[doc(hidden)]
                    unsafe fn _resource_new(val: *mut u8) -> u32
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = val;
                            unreachable!();
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(wasm_import_module = "[export]toxoid:engine/ecs")]
                            extern "C" {
                                #[link_name = "[resource-new]component"]
                                fn new(_: *mut u8) -> u32;
                            }
                            new(val)
                        }
                    }
                    #[doc(hidden)]
                    fn _resource_rep(handle: u32) -> *mut u8
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = handle;
                            unreachable!();
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(wasm_import_module = "[export]toxoid:engine/ecs")]
                            extern "C" {
                                #[link_name = "[resource-rep]component"]
                                fn rep(_: u32) -> *mut u8;
                            }
                            unsafe { rep(handle) }
                        }
                    }
                    fn new(init: ComponentDesc) -> Self;
                    fn get_id(&self) -> EcsEntityT;
                }
                #[doc(hidden)]
                macro_rules! __export_toxoid_engine_ecs_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "toxoid:engine/ecs#[constructor]component"] unsafe extern "C" fn
                        export_constructor_component(arg0 : * mut u8, arg1 : usize, arg2
                        : * mut u8, arg3 : usize, arg4 : * mut u8, arg5 : usize,) -> i32
                        { $($path_to_types)*:: _export_constructor_component_cabi::<<$ty
                        as $($path_to_types)*:: Guest >::Component > (arg0, arg1, arg2,
                        arg3, arg4, arg5) } #[export_name =
                        "toxoid:engine/ecs#[method]component.get-id"] unsafe extern "C"
                        fn export_method_component_get_id(arg0 : * mut u8,) -> i64 {
                        $($path_to_types)*:: _export_method_component_get_id_cabi::<<$ty
                        as $($path_to_types)*:: Guest >::Component > (arg0) } const _ :
                        () = { #[doc(hidden)] #[export_name =
                        "toxoid:engine/ecs#[dtor]component"] #[allow(non_snake_case)]
                        unsafe extern "C" fn dtor(rep : * mut u8) { $($path_to_types)*::
                        Component::dtor::< <$ty as $($path_to_types)*:: Guest
                        >::Component > (rep) } }; };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_toxoid_engine_ecs_cabi;
            }
        }
    }
}
mod _rt {
    pub use alloc_crate::string::String;
    pub use alloc_crate::vec::Vec;
    use core::fmt;
    use core::marker;
    use core::sync::atomic::{AtomicU32, Ordering::Relaxed};
    /// A type which represents a component model resource, either imported or
    /// exported into this component.
    ///
    /// This is a low-level wrapper which handles the lifetime of the resource
    /// (namely this has a destructor). The `T` provided defines the component model
    /// intrinsics that this wrapper uses.
    ///
    /// One of the chief purposes of this type is to provide `Deref` implementations
    /// to access the underlying data when it is owned.
    ///
    /// This type is primarily used in generated code for exported and imported
    /// resources.
    #[repr(transparent)]
    pub struct Resource<T: WasmResource> {
        handle: AtomicU32,
        _marker: marker::PhantomData<T>,
    }
    /// A trait which all wasm resources implement, namely providing the ability to
    /// drop a resource.
    ///
    /// This generally is implemented by generated code, not user-facing code.
    #[allow(clippy::missing_safety_doc)]
    pub unsafe trait WasmResource {
        /// Invokes the `[resource-drop]...` intrinsic.
        unsafe fn drop(handle: u32);
    }
    impl<T: WasmResource> Resource<T> {
        #[doc(hidden)]
        pub unsafe fn from_handle(handle: u32) -> Self {
            debug_assert!(handle != u32::MAX);
            Self {
                handle: AtomicU32::new(handle),
                _marker: marker::PhantomData,
            }
        }
        /// Takes ownership of the handle owned by `resource`.
        ///
        /// Note that this ideally would be `into_handle` taking `Resource<T>` by
        /// ownership. The code generator does not enable that in all situations,
        /// unfortunately, so this is provided instead.
        ///
        /// Also note that `take_handle` is in theory only ever called on values
        /// owned by a generated function. For example a generated function might
        /// take `Resource<T>` as an argument but then call `take_handle` on a
        /// reference to that argument. In that sense the dynamic nature of
        /// `take_handle` should only be exposed internally to generated code, not
        /// to user code.
        #[doc(hidden)]
        pub fn take_handle(resource: &Resource<T>) -> u32 {
            resource.handle.swap(u32::MAX, Relaxed)
        }
        #[doc(hidden)]
        pub fn handle(resource: &Resource<T>) -> u32 {
            resource.handle.load(Relaxed)
        }
    }
    impl<T: WasmResource> fmt::Debug for Resource<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Resource").field("handle", &self.handle).finish()
        }
    }
    impl<T: WasmResource> Drop for Resource<T> {
        fn drop(&mut self) {
            unsafe {
                match self.handle.load(Relaxed) {
                    u32::MAX => {}
                    other => T::drop(other),
                }
            }
        }
    }
    pub use alloc_crate::boxed::Box;
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
        }
    }
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr, layout);
    }
    pub fn as_i64<T: AsI64>(t: T) -> i64 {
        t.as_i64()
    }
    pub trait AsI64 {
        fn as_i64(self) -> i64;
    }
    impl<'a, T: Copy + AsI64> AsI64 for &'a T {
        fn as_i64(self) -> i64 {
            (*self).as_i64()
        }
    }
    impl AsI64 for i64 {
        #[inline]
        fn as_i64(self) -> i64 {
            self as i64
        }
    }
    impl AsI64 for u64 {
        #[inline]
        fn as_i64(self) -> i64 {
            self as i64
        }
    }
    extern crate alloc as alloc_crate;
    pub use alloc_crate::alloc;
}
/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]
macro_rules! __export_toxoid_engine_world_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*::
        exports::toxoid::engine::ecs::__export_toxoid_engine_ecs_cabi!($ty with_types_in
        $($path_to_types_root)*:: exports::toxoid::engine::ecs);
    };
}
#[doc(inline)]
pub(crate) use __export_toxoid_engine_world_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.31.0:toxoid:engine:toxoid-engine-world:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 512] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xf6\x02\x01A\x02\x01\
A\x02\x01B\x0f\x01w\x04\0\x0cecs-entity-t\x03\0\0\x01m\x0f\x04u8-t\x05u16-t\x05u\
32-t\x05u64-t\x04i8-t\x05i16-t\x05i32-t\x05i64-t\x05f32-t\x05f64-t\x06bool-t\x08\
string-t\x07array-t\x0au32array-t\x0af32array-t\x04\0\x0bmember-type\x03\0\x02\x01\
ps\x01p}\x01r\x03\x04names\x0cmember-names\x04\x0cmember-types\x05\x04\0\x0ecomp\
onent-desc\x03\0\x06\x04\0\x09component\x03\x01\x01i\x08\x01@\x01\x04init\x07\0\x09\
\x04\0\x16[constructor]component\x01\x0a\x01h\x08\x01@\x01\x04self\x0b\0\x01\x04\
\0\x18[method]component.get-id\x01\x0c\x04\x01\x11toxoid:engine/ecs\x05\0\x04\x01\
!toxoid:engine/toxoid-engine-world\x04\0\x0b\x19\x01\0\x13toxoid-engine-world\x03\
\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-component\x070.216.0\x10wit-\
bindgen-rust\x060.31.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
