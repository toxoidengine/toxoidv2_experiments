#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod toxoid {
        #[allow(dead_code)]
        pub mod api {
            #[allow(dead_code, clippy::all)]
            pub mod ecs {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                pub type EcsEntityT = u64;
                #[repr(u8)]
                #[derive(Clone, Copy, Eq, PartialEq)]
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
                            #[link(wasm_import_module = "[export]toxoid:api/ecs")]
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
                pub unsafe fn _export_method_component_set_member_u8_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32, arg2: i32) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::set_member_u8(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                        arg2 as u8,
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_set_member_u16_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32, arg2: i32) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::set_member_u16(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                        arg2 as u16,
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_set_member_u32_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32, arg2: i32) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::set_member_u32(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                        arg2 as u32,
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_set_member_u64_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32, arg2: i64) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::set_member_u64(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                        arg2 as u64,
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_set_member_i8_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32, arg2: i32) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::set_member_i8(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                        arg2 as i8,
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_set_member_i16_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32, arg2: i32) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::set_member_i16(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                        arg2 as i16,
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_set_member_i32_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32, arg2: i32) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::set_member_i32(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                        arg2,
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_set_member_i64_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32, arg2: i64) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::set_member_i64(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                        arg2,
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_set_member_f32_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32, arg2: f32) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::set_member_f32(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                        arg2,
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_set_member_f64_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32, arg2: f64) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::set_member_f64(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                        arg2,
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_set_member_bool_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32, arg2: i32) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::set_member_bool(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                        _rt::bool_lift(arg2 as u8),
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_set_member_string_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32, arg2: *mut u8, arg3: usize) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg3;
                    let bytes0 = _rt::Vec::from_raw_parts(arg2.cast(), len0, len0);
                    T::set_member_string(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                        _rt::string_lift(bytes0),
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_set_member_u32array_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32, arg2: *mut u8, arg3: usize) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg3;
                    T::set_member_u32array(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                        _rt::Vec::from_raw_parts(arg2.cast(), len0, len0),
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_set_member_f32array_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32, arg2: *mut u8, arg3: usize) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg3;
                    T::set_member_f32array(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                        _rt::Vec::from_raw_parts(arg2.cast(), len0, len0),
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_get_member_u8_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::get_member_u8(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                    );
                    _rt::as_i32(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_get_member_u16_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::get_member_u16(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                    );
                    _rt::as_i32(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_get_member_u32_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::get_member_u32(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                    );
                    _rt::as_i32(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_get_member_u64_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32) -> i64 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::get_member_u64(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                    );
                    _rt::as_i64(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_get_member_i8_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::get_member_i8(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                    );
                    _rt::as_i32(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_get_member_i16_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::get_member_i16(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                    );
                    _rt::as_i32(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_get_member_i32_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::get_member_i32(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                    );
                    _rt::as_i32(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_get_member_i64_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32) -> i64 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::get_member_i64(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                    );
                    _rt::as_i64(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_get_member_f32_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32) -> f32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::get_member_f32(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                    );
                    _rt::as_f32(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_get_member_f64_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32) -> f64 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::get_member_f64(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                    );
                    _rt::as_f64(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_get_member_bool_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::get_member_bool(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                    );
                    match result0 {
                        true => 1,
                        false => 0,
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_get_member_string_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::get_member_string(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let vec2 = (result0.into_bytes()).into_boxed_slice();
                    let ptr2 = vec2.as_ptr().cast::<u8>();
                    let len2 = vec2.len();
                    ::core::mem::forget(vec2);
                    *ptr1.add(4).cast::<usize>() = len2;
                    *ptr1.add(0).cast::<*mut u8>() = ptr2.cast_mut();
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_component_get_member_string<
                    T: GuestComponent,
                >(arg0: *mut u8) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    _rt::cabi_dealloc(l0, l1, 1);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_get_member_u32array_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::get_member_u32array(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let vec2 = (result0).into_boxed_slice();
                    let ptr2 = vec2.as_ptr().cast::<u8>();
                    let len2 = vec2.len();
                    ::core::mem::forget(vec2);
                    *ptr1.add(4).cast::<usize>() = len2;
                    *ptr1.add(0).cast::<*mut u8>() = ptr2.cast_mut();
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_component_get_member_u32array<
                    T: GuestComponent,
                >(arg0: *mut u8) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    let base2 = l0;
                    let len2 = l1;
                    _rt::cabi_dealloc(base2, len2 * 4, 4);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_get_member_f32array_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::get_member_f32array(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let vec2 = (result0).into_boxed_slice();
                    let ptr2 = vec2.as_ptr().cast::<u8>();
                    let len2 = vec2.len();
                    ::core::mem::forget(vec2);
                    *ptr1.add(4).cast::<usize>() = len2;
                    *ptr1.add(0).cast::<*mut u8>() = ptr2.cast_mut();
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_component_get_member_f32array<
                    T: GuestComponent,
                >(arg0: *mut u8) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    let base2 = l0;
                    let len2 = l1;
                    _rt::cabi_dealloc(base2, len2 * 4, 4);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_component_get_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                ) -> i64 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    let result1 = T::component_get(_rt::string_lift(bytes0));
                    _rt::as_i64(result1)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_tag_create_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                ) -> i64 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    let result1 = T::tag_create(_rt::string_lift(bytes0));
                    _rt::as_i64(result1)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_prefab_create_cabi<T: Guest>() -> i64 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::prefab_create();
                    _rt::as_i64(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_prefab_instance_cabi<T: Guest>(arg0: i64) -> i64 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::prefab_instance(arg0 as u64);
                    _rt::as_i64(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_entity_create_cabi<T: Guest>() -> i64 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::entity_create();
                    _rt::as_i64(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_singleton_add_cabi<T: Guest>(arg0: i64) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::singleton_add(arg0 as u64);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_singleton_get_cabi<T: Guest>(arg0: i64) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::singleton_get(arg0 as u64);
                    (result0).take_handle() as i32
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_singleton_remove_cabi<T: Guest>(arg0: i64) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::singleton_remove(arg0 as u64);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_entity_get_component_cabi<T: Guest>(
                    arg0: i64,
                    arg1: i64,
                ) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::entity_get_component(arg0 as u64, arg1 as u64);
                    (result0).take_handle() as i32
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_entity_set_component_cabi<T: Guest>(
                    arg0: i64,
                    arg1: i64,
                    arg2: i32,
                ) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::entity_set_component(
                        arg0 as u64,
                        arg1 as u64,
                        Component::from_handle(arg2 as u32),
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_entity_remove_component_cabi<T: Guest>(
                    arg0: i64,
                    arg1: i64,
                ) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::entity_remove_component(arg0 as u64, arg1 as u64);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_entity_has_component_cabi<T: Guest>(
                    arg0: i64,
                    arg1: i64,
                ) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::entity_has_component(arg0 as u64, arg1 as u64);
                    match result0 {
                        true => 1,
                        false => 0,
                    }
                }
                pub trait Guest {
                    type Component: GuestComponent;
                    /// component-create: func(desc: component-desc) -> ecs-entity-t;
                    fn component_get(name: _rt::String) -> EcsEntityT;
                    fn tag_create(tag_name: _rt::String) -> EcsEntityT;
                    fn prefab_create() -> EcsEntityT;
                    fn prefab_instance(prefab: EcsEntityT) -> EcsEntityT;
                    fn entity_create() -> EcsEntityT;
                    fn singleton_add(component: EcsEntityT);
                    fn singleton_get(component: EcsEntityT) -> Component;
                    fn singleton_remove(component: EcsEntityT);
                    fn entity_get_component(
                        entity: EcsEntityT,
                        component: EcsEntityT,
                    ) -> Component;
                    fn entity_set_component(
                        entity: EcsEntityT,
                        component: EcsEntityT,
                        value: Component,
                    );
                    fn entity_remove_component(
                        entity: EcsEntityT,
                        component: EcsEntityT,
                    );
                    fn entity_has_component(
                        entity: EcsEntityT,
                        component: EcsEntityT,
                    ) -> bool;
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
                            #[link(wasm_import_module = "[export]toxoid:api/ecs")]
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
                            #[link(wasm_import_module = "[export]toxoid:api/ecs")]
                            extern "C" {
                                #[link_name = "[resource-rep]component"]
                                fn rep(_: u32) -> *mut u8;
                            }
                            unsafe { rep(handle) }
                        }
                    }
                    fn new(init: ComponentDesc) -> Self;
                    fn set_member_u8(&self, offset: u32, value: u8);
                    fn set_member_u16(&self, offset: u32, value: u16);
                    fn set_member_u32(&self, offset: u32, value: u32);
                    fn set_member_u64(&self, offset: u32, value: u64);
                    fn set_member_i8(&self, offset: u32, value: i8);
                    fn set_member_i16(&self, offset: u32, value: i16);
                    fn set_member_i32(&self, offset: u32, value: i32);
                    fn set_member_i64(&self, offset: u32, value: i64);
                    fn set_member_f32(&self, offset: u32, value: f32);
                    fn set_member_f64(&self, offset: u32, value: f64);
                    fn set_member_bool(&self, offset: u32, value: bool);
                    fn set_member_string(&self, offset: u32, value: _rt::String);
                    fn set_member_u32array(&self, offset: u32, value: _rt::Vec<u32>);
                    fn set_member_f32array(&self, offset: u32, value: _rt::Vec<f32>);
                    fn get_member_u8(&self, offset: u32) -> u8;
                    fn get_member_u16(&self, offset: u32) -> u16;
                    fn get_member_u32(&self, offset: u32) -> u32;
                    fn get_member_u64(&self, offset: u32) -> u64;
                    fn get_member_i8(&self, offset: u32) -> i8;
                    fn get_member_i16(&self, offset: u32) -> i16;
                    fn get_member_i32(&self, offset: u32) -> i32;
                    fn get_member_i64(&self, offset: u32) -> i64;
                    fn get_member_f32(&self, offset: u32) -> f32;
                    fn get_member_f64(&self, offset: u32) -> f64;
                    fn get_member_bool(&self, offset: u32) -> bool;
                    fn get_member_string(&self, offset: u32) -> _rt::String;
                    fn get_member_u32array(&self, offset: u32) -> _rt::Vec<u32>;
                    fn get_member_f32array(&self, offset: u32) -> _rt::Vec<f32>;
                }
                #[doc(hidden)]
                macro_rules! __export_toxoid_api_ecs_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "toxoid:api/ecs#[constructor]component"] unsafe extern "C" fn
                        export_constructor_component(arg0 : * mut u8, arg1 : usize, arg2
                        : * mut u8, arg3 : usize, arg4 : * mut u8, arg5 : usize,) -> i32
                        { $($path_to_types)*:: _export_constructor_component_cabi::<<$ty
                        as $($path_to_types)*:: Guest >::Component > (arg0, arg1, arg2,
                        arg3, arg4, arg5) } #[export_name =
                        "toxoid:api/ecs#[method]component.set-member-u8"] unsafe extern
                        "C" fn export_method_component_set_member_u8(arg0 : * mut u8,
                        arg1 : i32, arg2 : i32,) { $($path_to_types)*::
                        _export_method_component_set_member_u8_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1, arg2) }
                        #[export_name =
                        "toxoid:api/ecs#[method]component.set-member-u16"] unsafe extern
                        "C" fn export_method_component_set_member_u16(arg0 : * mut u8,
                        arg1 : i32, arg2 : i32,) { $($path_to_types)*::
                        _export_method_component_set_member_u16_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1, arg2) }
                        #[export_name =
                        "toxoid:api/ecs#[method]component.set-member-u32"] unsafe extern
                        "C" fn export_method_component_set_member_u32(arg0 : * mut u8,
                        arg1 : i32, arg2 : i32,) { $($path_to_types)*::
                        _export_method_component_set_member_u32_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1, arg2) }
                        #[export_name =
                        "toxoid:api/ecs#[method]component.set-member-u64"] unsafe extern
                        "C" fn export_method_component_set_member_u64(arg0 : * mut u8,
                        arg1 : i32, arg2 : i64,) { $($path_to_types)*::
                        _export_method_component_set_member_u64_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1, arg2) }
                        #[export_name = "toxoid:api/ecs#[method]component.set-member-i8"]
                        unsafe extern "C" fn export_method_component_set_member_i8(arg0 :
                        * mut u8, arg1 : i32, arg2 : i32,) { $($path_to_types)*::
                        _export_method_component_set_member_i8_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1, arg2) }
                        #[export_name =
                        "toxoid:api/ecs#[method]component.set-member-i16"] unsafe extern
                        "C" fn export_method_component_set_member_i16(arg0 : * mut u8,
                        arg1 : i32, arg2 : i32,) { $($path_to_types)*::
                        _export_method_component_set_member_i16_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1, arg2) }
                        #[export_name =
                        "toxoid:api/ecs#[method]component.set-member-i32"] unsafe extern
                        "C" fn export_method_component_set_member_i32(arg0 : * mut u8,
                        arg1 : i32, arg2 : i32,) { $($path_to_types)*::
                        _export_method_component_set_member_i32_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1, arg2) }
                        #[export_name =
                        "toxoid:api/ecs#[method]component.set-member-i64"] unsafe extern
                        "C" fn export_method_component_set_member_i64(arg0 : * mut u8,
                        arg1 : i32, arg2 : i64,) { $($path_to_types)*::
                        _export_method_component_set_member_i64_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1, arg2) }
                        #[export_name =
                        "toxoid:api/ecs#[method]component.set-member-f32"] unsafe extern
                        "C" fn export_method_component_set_member_f32(arg0 : * mut u8,
                        arg1 : i32, arg2 : f32,) { $($path_to_types)*::
                        _export_method_component_set_member_f32_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1, arg2) }
                        #[export_name =
                        "toxoid:api/ecs#[method]component.set-member-f64"] unsafe extern
                        "C" fn export_method_component_set_member_f64(arg0 : * mut u8,
                        arg1 : i32, arg2 : f64,) { $($path_to_types)*::
                        _export_method_component_set_member_f64_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1, arg2) }
                        #[export_name =
                        "toxoid:api/ecs#[method]component.set-member-bool"] unsafe extern
                        "C" fn export_method_component_set_member_bool(arg0 : * mut u8,
                        arg1 : i32, arg2 : i32,) { $($path_to_types)*::
                        _export_method_component_set_member_bool_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1, arg2) }
                        #[export_name =
                        "toxoid:api/ecs#[method]component.set-member-string"] unsafe
                        extern "C" fn export_method_component_set_member_string(arg0 : *
                        mut u8, arg1 : i32, arg2 : * mut u8, arg3 : usize,) {
                        $($path_to_types)*::
                        _export_method_component_set_member_string_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1, arg2,
                        arg3) } #[export_name =
                        "toxoid:api/ecs#[method]component.set-member-u32array"] unsafe
                        extern "C" fn export_method_component_set_member_u32array(arg0 :
                        * mut u8, arg1 : i32, arg2 : * mut u8, arg3 : usize,) {
                        $($path_to_types)*::
                        _export_method_component_set_member_u32array_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1, arg2,
                        arg3) } #[export_name =
                        "toxoid:api/ecs#[method]component.set-member-f32array"] unsafe
                        extern "C" fn export_method_component_set_member_f32array(arg0 :
                        * mut u8, arg1 : i32, arg2 : * mut u8, arg3 : usize,) {
                        $($path_to_types)*::
                        _export_method_component_set_member_f32array_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1, arg2,
                        arg3) } #[export_name =
                        "toxoid:api/ecs#[method]component.get-member-u8"] unsafe extern
                        "C" fn export_method_component_get_member_u8(arg0 : * mut u8,
                        arg1 : i32,) -> i32 { $($path_to_types)*::
                        _export_method_component_get_member_u8_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1) }
                        #[export_name =
                        "toxoid:api/ecs#[method]component.get-member-u16"] unsafe extern
                        "C" fn export_method_component_get_member_u16(arg0 : * mut u8,
                        arg1 : i32,) -> i32 { $($path_to_types)*::
                        _export_method_component_get_member_u16_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1) }
                        #[export_name =
                        "toxoid:api/ecs#[method]component.get-member-u32"] unsafe extern
                        "C" fn export_method_component_get_member_u32(arg0 : * mut u8,
                        arg1 : i32,) -> i32 { $($path_to_types)*::
                        _export_method_component_get_member_u32_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1) }
                        #[export_name =
                        "toxoid:api/ecs#[method]component.get-member-u64"] unsafe extern
                        "C" fn export_method_component_get_member_u64(arg0 : * mut u8,
                        arg1 : i32,) -> i64 { $($path_to_types)*::
                        _export_method_component_get_member_u64_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1) }
                        #[export_name = "toxoid:api/ecs#[method]component.get-member-i8"]
                        unsafe extern "C" fn export_method_component_get_member_i8(arg0 :
                        * mut u8, arg1 : i32,) -> i32 { $($path_to_types)*::
                        _export_method_component_get_member_i8_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1) }
                        #[export_name =
                        "toxoid:api/ecs#[method]component.get-member-i16"] unsafe extern
                        "C" fn export_method_component_get_member_i16(arg0 : * mut u8,
                        arg1 : i32,) -> i32 { $($path_to_types)*::
                        _export_method_component_get_member_i16_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1) }
                        #[export_name =
                        "toxoid:api/ecs#[method]component.get-member-i32"] unsafe extern
                        "C" fn export_method_component_get_member_i32(arg0 : * mut u8,
                        arg1 : i32,) -> i32 { $($path_to_types)*::
                        _export_method_component_get_member_i32_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1) }
                        #[export_name =
                        "toxoid:api/ecs#[method]component.get-member-i64"] unsafe extern
                        "C" fn export_method_component_get_member_i64(arg0 : * mut u8,
                        arg1 : i32,) -> i64 { $($path_to_types)*::
                        _export_method_component_get_member_i64_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1) }
                        #[export_name =
                        "toxoid:api/ecs#[method]component.get-member-f32"] unsafe extern
                        "C" fn export_method_component_get_member_f32(arg0 : * mut u8,
                        arg1 : i32,) -> f32 { $($path_to_types)*::
                        _export_method_component_get_member_f32_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1) }
                        #[export_name =
                        "toxoid:api/ecs#[method]component.get-member-f64"] unsafe extern
                        "C" fn export_method_component_get_member_f64(arg0 : * mut u8,
                        arg1 : i32,) -> f64 { $($path_to_types)*::
                        _export_method_component_get_member_f64_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1) }
                        #[export_name =
                        "toxoid:api/ecs#[method]component.get-member-bool"] unsafe extern
                        "C" fn export_method_component_get_member_bool(arg0 : * mut u8,
                        arg1 : i32,) -> i32 { $($path_to_types)*::
                        _export_method_component_get_member_bool_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1) }
                        #[export_name =
                        "toxoid:api/ecs#[method]component.get-member-string"] unsafe
                        extern "C" fn export_method_component_get_member_string(arg0 : *
                        mut u8, arg1 : i32,) -> * mut u8 { $($path_to_types)*::
                        _export_method_component_get_member_string_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1) }
                        #[export_name =
                        "cabi_post_toxoid:api/ecs#[method]component.get-member-string"]
                        unsafe extern "C" fn
                        _post_return_method_component_get_member_string(arg0 : * mut u8,)
                        { $($path_to_types)*::
                        __post_return_method_component_get_member_string::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0) } #[export_name
                        = "toxoid:api/ecs#[method]component.get-member-u32array"] unsafe
                        extern "C" fn export_method_component_get_member_u32array(arg0 :
                        * mut u8, arg1 : i32,) -> * mut u8 { $($path_to_types)*::
                        _export_method_component_get_member_u32array_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1) }
                        #[export_name =
                        "cabi_post_toxoid:api/ecs#[method]component.get-member-u32array"]
                        unsafe extern "C" fn
                        _post_return_method_component_get_member_u32array(arg0 : * mut
                        u8,) { $($path_to_types)*::
                        __post_return_method_component_get_member_u32array::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0) } #[export_name
                        = "toxoid:api/ecs#[method]component.get-member-f32array"] unsafe
                        extern "C" fn export_method_component_get_member_f32array(arg0 :
                        * mut u8, arg1 : i32,) -> * mut u8 { $($path_to_types)*::
                        _export_method_component_get_member_f32array_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1) }
                        #[export_name =
                        "cabi_post_toxoid:api/ecs#[method]component.get-member-f32array"]
                        unsafe extern "C" fn
                        _post_return_method_component_get_member_f32array(arg0 : * mut
                        u8,) { $($path_to_types)*::
                        __post_return_method_component_get_member_f32array::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0) } #[export_name
                        = "toxoid:api/ecs#component-get"] unsafe extern "C" fn
                        export_component_get(arg0 : * mut u8, arg1 : usize,) -> i64 {
                        $($path_to_types)*:: _export_component_get_cabi::<$ty > (arg0,
                        arg1) } #[export_name = "toxoid:api/ecs#tag-create"] unsafe
                        extern "C" fn export_tag_create(arg0 : * mut u8, arg1 : usize,)
                        -> i64 { $($path_to_types)*:: _export_tag_create_cabi::<$ty >
                        (arg0, arg1) } #[export_name = "toxoid:api/ecs#prefab-create"]
                        unsafe extern "C" fn export_prefab_create() -> i64 {
                        $($path_to_types)*:: _export_prefab_create_cabi::<$ty > () }
                        #[export_name = "toxoid:api/ecs#prefab-instance"] unsafe extern
                        "C" fn export_prefab_instance(arg0 : i64,) -> i64 {
                        $($path_to_types)*:: _export_prefab_instance_cabi::<$ty > (arg0)
                        } #[export_name = "toxoid:api/ecs#entity-create"] unsafe extern
                        "C" fn export_entity_create() -> i64 { $($path_to_types)*::
                        _export_entity_create_cabi::<$ty > () } #[export_name =
                        "toxoid:api/ecs#singleton-add"] unsafe extern "C" fn
                        export_singleton_add(arg0 : i64,) { $($path_to_types)*::
                        _export_singleton_add_cabi::<$ty > (arg0) } #[export_name =
                        "toxoid:api/ecs#singleton-get"] unsafe extern "C" fn
                        export_singleton_get(arg0 : i64,) -> i32 { $($path_to_types)*::
                        _export_singleton_get_cabi::<$ty > (arg0) } #[export_name =
                        "toxoid:api/ecs#singleton-remove"] unsafe extern "C" fn
                        export_singleton_remove(arg0 : i64,) { $($path_to_types)*::
                        _export_singleton_remove_cabi::<$ty > (arg0) } #[export_name =
                        "toxoid:api/ecs#entity-get-component"] unsafe extern "C" fn
                        export_entity_get_component(arg0 : i64, arg1 : i64,) -> i32 {
                        $($path_to_types)*:: _export_entity_get_component_cabi::<$ty >
                        (arg0, arg1) } #[export_name =
                        "toxoid:api/ecs#entity-set-component"] unsafe extern "C" fn
                        export_entity_set_component(arg0 : i64, arg1 : i64, arg2 : i32,)
                        { $($path_to_types)*:: _export_entity_set_component_cabi::<$ty >
                        (arg0, arg1, arg2) } #[export_name =
                        "toxoid:api/ecs#entity-remove-component"] unsafe extern "C" fn
                        export_entity_remove_component(arg0 : i64, arg1 : i64,) {
                        $($path_to_types)*:: _export_entity_remove_component_cabi::<$ty >
                        (arg0, arg1) } #[export_name =
                        "toxoid:api/ecs#entity-has-component"] unsafe extern "C" fn
                        export_entity_has_component(arg0 : i64, arg1 : i64,) -> i32 {
                        $($path_to_types)*:: _export_entity_has_component_cabi::<$ty >
                        (arg0, arg1) } const _ : () = { #[doc(hidden)] #[export_name =
                        "toxoid:api/ecs#[dtor]component"] #[allow(non_snake_case)] unsafe
                        extern "C" fn dtor(rep : * mut u8) { $($path_to_types)*::
                        Component::dtor::< <$ty as $($path_to_types)*:: Guest
                        >::Component > (rep) } }; };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_toxoid_api_ecs_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 8]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 8],
                );
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
    pub unsafe fn bool_lift(val: u8) -> bool {
        if cfg!(debug_assertions) {
            match val {
                0 => false,
                1 => true,
                _ => panic!("invalid bool discriminant"),
            }
        } else {
            val != 0
        }
    }
    pub fn as_i32<T: AsI32>(t: T) -> i32 {
        t.as_i32()
    }
    pub trait AsI32 {
        fn as_i32(self) -> i32;
    }
    impl<'a, T: Copy + AsI32> AsI32 for &'a T {
        fn as_i32(self) -> i32 {
            (*self).as_i32()
        }
    }
    impl AsI32 for i32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for char {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for usize {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
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
    pub fn as_f32<T: AsF32>(t: T) -> f32 {
        t.as_f32()
    }
    pub trait AsF32 {
        fn as_f32(self) -> f32;
    }
    impl<'a, T: Copy + AsF32> AsF32 for &'a T {
        fn as_f32(self) -> f32 {
            (*self).as_f32()
        }
    }
    impl AsF32 for f32 {
        #[inline]
        fn as_f32(self) -> f32 {
            self as f32
        }
    }
    pub fn as_f64<T: AsF64>(t: T) -> f64 {
        t.as_f64()
    }
    pub trait AsF64 {
        fn as_f64(self) -> f64;
    }
    impl<'a, T: Copy + AsF64> AsF64 for &'a T {
        fn as_f64(self) -> f64 {
            (*self).as_f64()
        }
    }
    impl AsF64 for f64 {
        #[inline]
        fn as_f64(self) -> f64 {
            self as f64
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
macro_rules! __export_toxoid_world_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*::
        exports::toxoid::api::ecs::__export_toxoid_api_ecs_cabi!($ty with_types_in
        $($path_to_types_root)*:: exports::toxoid::api::ecs);
    };
}
#[doc(inline)]
pub(crate) use __export_toxoid_world_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.30.0:toxoid-world:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 2576] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\x8d\x13\x01A\x02\x01\
A\x02\x01B]\x01w\x04\0\x0cecs-entity-t\x03\0\0\x01m\x0f\x04u8-t\x05u16-t\x05u32-\
t\x05u64-t\x04i8-t\x05i16-t\x05i32-t\x05i64-t\x05f32-t\x05f64-t\x06bool-t\x08str\
ing-t\x07array-t\x0au32array-t\x0af32array-t\x04\0\x0bmember-type\x03\0\x02\x01p\
s\x01p}\x01r\x03\x04names\x0cmember-names\x04\x0cmember-types\x05\x04\0\x0ecompo\
nent-desc\x03\0\x06\x04\0\x09component\x03\x01\x01i\x08\x01@\x01\x04init\x07\0\x09\
\x04\0\x16[constructor]component\x01\x0a\x01h\x08\x01@\x03\x04self\x0b\x06offset\
y\x05value}\x01\0\x04\0\x1f[method]component.set-member-u8\x01\x0c\x01@\x03\x04s\
elf\x0b\x06offsety\x05value{\x01\0\x04\0\x20[method]component.set-member-u16\x01\
\x0d\x01@\x03\x04self\x0b\x06offsety\x05valuey\x01\0\x04\0\x20[method]component.\
set-member-u32\x01\x0e\x01@\x03\x04self\x0b\x06offsety\x05valuew\x01\0\x04\0\x20\
[method]component.set-member-u64\x01\x0f\x01@\x03\x04self\x0b\x06offsety\x05valu\
e~\x01\0\x04\0\x1f[method]component.set-member-i8\x01\x10\x01@\x03\x04self\x0b\x06\
offsety\x05value|\x01\0\x04\0\x20[method]component.set-member-i16\x01\x11\x01@\x03\
\x04self\x0b\x06offsety\x05valuez\x01\0\x04\0\x20[method]component.set-member-i3\
2\x01\x12\x01@\x03\x04self\x0b\x06offsety\x05valuex\x01\0\x04\0\x20[method]compo\
nent.set-member-i64\x01\x13\x01@\x03\x04self\x0b\x06offsety\x05valuev\x01\0\x04\0\
\x20[method]component.set-member-f32\x01\x14\x01@\x03\x04self\x0b\x06offsety\x05\
valueu\x01\0\x04\0\x20[method]component.set-member-f64\x01\x15\x01@\x03\x04self\x0b\
\x06offsety\x05value\x7f\x01\0\x04\0![method]component.set-member-bool\x01\x16\x01\
@\x03\x04self\x0b\x06offsety\x05values\x01\0\x04\0#[method]component.set-member-\
string\x01\x17\x01py\x01@\x03\x04self\x0b\x06offsety\x05value\x18\x01\0\x04\0%[m\
ethod]component.set-member-u32array\x01\x19\x01pv\x01@\x03\x04self\x0b\x06offset\
y\x05value\x1a\x01\0\x04\0%[method]component.set-member-f32array\x01\x1b\x01@\x02\
\x04self\x0b\x06offsety\0}\x04\0\x1f[method]component.get-member-u8\x01\x1c\x01@\
\x02\x04self\x0b\x06offsety\0{\x04\0\x20[method]component.get-member-u16\x01\x1d\
\x01@\x02\x04self\x0b\x06offsety\0y\x04\0\x20[method]component.get-member-u32\x01\
\x1e\x01@\x02\x04self\x0b\x06offsety\0w\x04\0\x20[method]component.get-member-u6\
4\x01\x1f\x01@\x02\x04self\x0b\x06offsety\0~\x04\0\x1f[method]component.get-memb\
er-i8\x01\x20\x01@\x02\x04self\x0b\x06offsety\0|\x04\0\x20[method]component.get-\
member-i16\x01!\x01@\x02\x04self\x0b\x06offsety\0z\x04\0\x20[method]component.ge\
t-member-i32\x01\"\x01@\x02\x04self\x0b\x06offsety\0x\x04\0\x20[method]component\
.get-member-i64\x01#\x01@\x02\x04self\x0b\x06offsety\0v\x04\0\x20[method]compone\
nt.get-member-f32\x01$\x01@\x02\x04self\x0b\x06offsety\0u\x04\0\x20[method]compo\
nent.get-member-f64\x01%\x01@\x02\x04self\x0b\x06offsety\0\x7f\x04\0![method]com\
ponent.get-member-bool\x01&\x01@\x02\x04self\x0b\x06offsety\0s\x04\0#[method]com\
ponent.get-member-string\x01'\x01@\x02\x04self\x0b\x06offsety\0\x18\x04\0%[metho\
d]component.get-member-u32array\x01(\x01@\x02\x04self\x0b\x06offsety\0\x1a\x04\0\
%[method]component.get-member-f32array\x01)\x01@\x01\x04names\0\x01\x04\0\x0dcom\
ponent-get\x01*\x01@\x01\x08tag-names\0\x01\x04\0\x0atag-create\x01+\x01@\0\0\x01\
\x04\0\x0dprefab-create\x01,\x01@\x01\x06prefab\x01\0\x01\x04\0\x0fprefab-instan\
ce\x01-\x04\0\x0dentity-create\x01,\x01@\x01\x09component\x01\x01\0\x04\0\x0dsin\
gleton-add\x01.\x01@\x01\x09component\x01\0\x09\x04\0\x0dsingleton-get\x01/\x04\0\
\x10singleton-remove\x01.\x01@\x02\x06entity\x01\x09component\x01\0\x09\x04\0\x14\
entity-get-component\x010\x01@\x03\x06entity\x01\x09component\x01\x05value\x09\x01\
\0\x04\0\x14entity-set-component\x011\x01@\x02\x06entity\x01\x09component\x01\x01\
\0\x04\0\x17entity-remove-component\x012\x01@\x02\x06entity\x01\x09component\x01\
\0\x7f\x04\0\x14entity-has-component\x013\x04\x01\x0etoxoid:api/ecs\x05\0\x04\x01\
\x17toxoid:api/toxoid-world\x04\0\x0b\x12\x01\0\x0ctoxoid-world\x03\0\0\0G\x09pr\
oducers\x01\x0cprocessed-by\x02\x0dwit-component\x070.215.0\x10wit-bindgen-rust\x06\
0.30.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
