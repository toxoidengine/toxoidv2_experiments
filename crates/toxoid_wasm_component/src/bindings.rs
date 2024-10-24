#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn _export_init_cabi<T: Guest>(arg0: *mut u8, arg1: usize) -> i64 {
    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
    let len0 = arg1;
    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
    let result1 = T::init(_rt::string_lift(bytes0));
    _rt::as_i64(result1)
}
pub trait Guest {
    fn init(name: _rt::String) -> u64;
}
#[doc(hidden)]
macro_rules! __export_world_toxoid_component_world_cabi {
    ($ty:ident with_types_in $($path_to_types:tt)*) => {
        const _ : () = { #[export_name = "init"] unsafe extern "C" fn export_init(arg0 :
        * mut u8, arg1 : usize,) -> i64 { $($path_to_types)*:: _export_init_cabi::<$ty >
        (arg0, arg1) } };
    };
}
#[doc(hidden)]
pub(crate) use __export_world_toxoid_component_world_cabi;
#[allow(dead_code)]
pub mod toxoid_component {
    #[allow(dead_code)]
    pub mod component {
        #[allow(dead_code, clippy::all)]
        pub mod ecs {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
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
                        MemberType::U16T => f.debug_tuple("MemberType::U16T").finish(),
                        MemberType::U32T => f.debug_tuple("MemberType::U32T").finish(),
                        MemberType::U64T => f.debug_tuple("MemberType::U64T").finish(),
                        MemberType::I8T => f.debug_tuple("MemberType::I8T").finish(),
                        MemberType::I16T => f.debug_tuple("MemberType::I16T").finish(),
                        MemberType::I32T => f.debug_tuple("MemberType::I32T").finish(),
                        MemberType::I64T => f.debug_tuple("MemberType::I64T").finish(),
                        MemberType::F32T => f.debug_tuple("MemberType::F32T").finish(),
                        MemberType::F64T => f.debug_tuple("MemberType::F64T").finish(),
                        MemberType::BoolT => f.debug_tuple("MemberType::BoolT").finish(),
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
            #[derive(Clone)]
            pub struct EntityDesc {
                pub name: Option<_rt::String>,
            }
            impl ::core::fmt::Debug for EntityDesc {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("EntityDesc").field("name", &self.name).finish()
                }
            }
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct ComponentType {
                handle: _rt::Resource<ComponentType>,
            }
            impl ComponentType {
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
            }
            unsafe impl _rt::WasmResource for ComponentType {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[resource-drop]component-type"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct Component {
                handle: _rt::Resource<Component>,
            }
            impl Component {
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
            }
            unsafe impl _rt::WasmResource for Component {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[resource-drop]component"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct Entity {
                handle: _rt::Resource<Entity>,
            }
            impl Entity {
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
            }
            unsafe impl _rt::WasmResource for Entity {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[resource-drop]entity"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            impl ComponentType {
                #[allow(unused_unsafe, clippy::all)]
                pub fn new(init: &ComponentDesc) -> Self {
                    unsafe {
                        let ComponentDesc {
                            name: name0,
                            member_names: member_names0,
                            member_types: member_types0,
                        } = init;
                        let vec1 = name0;
                        let ptr1 = vec1.as_ptr().cast::<u8>();
                        let len1 = vec1.len();
                        let vec3 = member_names0;
                        let len3 = vec3.len();
                        let layout3 = _rt::alloc::Layout::from_size_align_unchecked(
                            vec3.len() * 8,
                            4,
                        );
                        let result3 = if layout3.size() != 0 {
                            let ptr = _rt::alloc::alloc(layout3).cast::<u8>();
                            if ptr.is_null() {
                                _rt::alloc::handle_alloc_error(layout3);
                            }
                            ptr
                        } else {
                            ::core::ptr::null_mut()
                        };
                        for (i, e) in vec3.into_iter().enumerate() {
                            let base = result3.add(i * 8);
                            {
                                let vec2 = e;
                                let ptr2 = vec2.as_ptr().cast::<u8>();
                                let len2 = vec2.len();
                                *base.add(4).cast::<usize>() = len2;
                                *base.add(0).cast::<*mut u8>() = ptr2.cast_mut();
                            }
                        }
                        let vec4 = member_types0;
                        let ptr4 = vec4.as_ptr().cast::<u8>();
                        let len4 = vec4.len();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[constructor]component-type"]
                            fn wit_import(
                                _: *mut u8,
                                _: usize,
                                _: *mut u8,
                                _: usize,
                                _: *mut u8,
                                _: usize,
                            ) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(
                            _: *mut u8,
                            _: usize,
                            _: *mut u8,
                            _: usize,
                            _: *mut u8,
                            _: usize,
                        ) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import(
                            ptr1.cast_mut(),
                            len1,
                            result3,
                            len3,
                            ptr4.cast_mut(),
                            len4,
                        );
                        if layout3.size() != 0 {
                            _rt::alloc::dealloc(result3.cast(), layout3);
                        }
                        ComponentType::from_handle(ret as u32)
                    }
                }
            }
            impl ComponentType {
                #[allow(unused_unsafe, clippy::all)]
                pub fn get_id(&self) -> EcsEntityT {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[method]component-type.get-id"]
                            fn wit_import(_: i32) -> i64;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i64 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        ret as u64
                    }
                }
            }
            impl Entity {
                #[allow(unused_unsafe, clippy::all)]
                pub fn new(init: &EntityDesc) -> Self {
                    unsafe {
                        let EntityDesc { name: name0 } = init;
                        let (result2_0, result2_1, result2_2) = match name0 {
                            Some(e) => {
                                let vec1 = e;
                                let ptr1 = vec1.as_ptr().cast::<u8>();
                                let len1 = vec1.len();
                                (1i32, ptr1.cast_mut(), len1)
                            }
                            None => (0i32, ::core::ptr::null_mut(), 0usize),
                        };
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[constructor]entity"]
                            fn wit_import(_: i32, _: *mut u8, _: usize) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8, _: usize) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import(result2_0, result2_1, result2_2);
                        Entity::from_handle(ret as u32)
                    }
                }
            }
            impl Entity {
                #[allow(unused_unsafe, clippy::all)]
                pub fn get_id(&self) -> EcsEntityT {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[method]entity.get-id"]
                            fn wit_import(_: i32) -> i64;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i64 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        ret as u64
                    }
                }
            }
            impl Entity {
                #[allow(unused_unsafe, clippy::all)]
                pub fn get_component(&self, component: EcsEntityT) -> Component {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[method]entity.get-component"]
                            fn wit_import(_: i32, _: i64) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import(
                            (self).handle() as i32,
                            _rt::as_i64(component),
                        );
                        Component::from_handle(ret as u32)
                    }
                }
            }
            impl Entity {
                #[allow(unused_unsafe, clippy::all)]
                pub fn add_component(&self, component: EcsEntityT) {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[method]entity.add-component"]
                            fn wit_import(_: i32, _: i64);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i64(component));
                    }
                }
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
    pub use alloc_crate::alloc;
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
    extern crate alloc as alloc_crate;
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
macro_rules! __export_toxoid_component_world_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*:: __export_world_toxoid_component_world_cabi!($ty
        with_types_in $($path_to_types_root)*);
    };
}
#[doc(inline)]
pub(crate) use __export_toxoid_component_world_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.31.0:toxoid-component:component:toxoid-component-world:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 824] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xab\x05\x01A\x02\x01\
A\x04\x01B\x1f\x01w\x04\0\x0cecs-entity-t\x03\0\0\x01m\x0f\x04u8-t\x05u16-t\x05u\
32-t\x05u64-t\x04i8-t\x05i16-t\x05i32-t\x05i64-t\x05f32-t\x05f64-t\x06bool-t\x08\
string-t\x07array-t\x0au32array-t\x0af32array-t\x04\0\x0bmember-type\x03\0\x02\x01\
ps\x01p}\x01r\x03\x04names\x0cmember-names\x04\x0cmember-types\x05\x04\0\x0ecomp\
onent-desc\x03\0\x06\x01ks\x01r\x01\x04name\x08\x04\0\x0bentity-desc\x03\0\x09\x04\
\0\x0ecomponent-type\x03\x01\x04\0\x09component\x03\x01\x04\0\x06entity\x03\x01\x01\
i\x0b\x01@\x01\x04init\x07\0\x0e\x04\0\x1b[constructor]component-type\x01\x0f\x01\
h\x0b\x01@\x01\x04self\x10\0\x01\x04\0\x1d[method]component-type.get-id\x01\x11\x01\
i\x0d\x01@\x01\x04init\x0a\0\x12\x04\0\x13[constructor]entity\x01\x13\x01h\x0d\x01\
@\x01\x04self\x14\0\x01\x04\0\x15[method]entity.get-id\x01\x15\x01i\x0c\x01@\x02\
\x04self\x14\x09component\x01\0\x16\x04\0\x1c[method]entity.get-component\x01\x17\
\x01@\x02\x04self\x14\x09component\x01\x01\0\x04\0\x1c[method]entity.add-compone\
nt\x01\x18\x03\x01\x1etoxoid-component:component/ecs\x05\0\x01@\x01\x04names\0w\x04\
\0\x04init\x01\x01\x04\x011toxoid-component:component/toxoid-component-world\x04\
\0\x0b\x1c\x01\0\x16toxoid-component-world\x03\0\0\0G\x09producers\x01\x0cproces\
sed-by\x02\x0dwit-component\x070.216.0\x10wit-bindgen-rust\x060.31.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
