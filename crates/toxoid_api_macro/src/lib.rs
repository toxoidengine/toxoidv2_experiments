extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream, Parser}, parse_macro_input, punctuated::Punctuated, spanned::Spanned, token::Comma, FieldsNamed, Ident, ItemFn, Type
};

#[repr(u8)]
enum FieldType {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    Bool,
    String,
    U32Array,
    F32Array,
    Pointer
}

// The input to the macro will be a list of field names and types.
struct ComponentStruct {
    name: Ident,
    fields: FieldsNamed,
}
 
// Implement the parsing functionality.
impl Parse for ComponentStruct {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        let fields = input.parse()?;
        Ok(ComponentStruct { name, fields })
    }
}
 
#[proc_macro]
pub fn component(input: TokenStream) -> TokenStream {
    let components = Punctuated::<ComponentStruct, Comma>::parse_terminated
        .parse(input)
        .unwrap();
    let expanded = components
        .into_iter()
        .map(|component| {
            let ComponentStruct { name, fields } = component;
            let fields: Vec<_> = fields.named.iter().collect();

            let field_names = fields.iter().map(|f| &f.ident);
            let field_types = fields.iter().map(|f| &f.ty);

            fn align_offset(offset: u32, align: u32) -> u32 {
                (offset + align - 1) & !(align - 1)
            }
            let mut current_offset = 0;
            let fields_offsets = fields.iter().map(
                |field| {
                    let field_type = &field.ty;
                    let size = get_type_size(field_type);
                    let align = get_type_alignment(field_type);
                    current_offset = align_offset(current_offset, align);
                    let offset = current_offset;
                    current_offset += size;
                    offset
                }
            );

            let getters_and_setters =
                field_names
                    .clone()
                    .zip(field_types.clone())
                    .zip(fields_offsets)
                    .map(|((field_name, field_type), field_offset)| {
                        let getter_name = Ident::new(
                            &format!("get_{}", field_name.as_ref().unwrap()),
                            field_name.span(),
                        );
                        let setter_name = Ident::new(
                            &format!("set_{}", field_name.as_ref().unwrap()),
                            field_name.span(),
                        );
                        let field_type_str = format!("{}", quote!(#field_type));
                        match () {
                            _ if field_type_str == "u8" => {
                                quote! {
                                    pub fn #getter_name(&self) -> #field_type {
                                        unsafe {
                                            self.ptr.as_mut().unwrap().get_member_u8(#field_offset)
                                        }
                                    }
                                    pub fn #setter_name(&mut self, value: u8) {
                                        unsafe {
                                            self.ptr.as_mut().unwrap().set_member_u8(#field_offset, value);
                                        }
                                    }
                                }
                            },
                            _ if field_type_str == "u16" => {
                                quote! {
                                    pub fn #getter_name(&self) -> #field_type {
                                        unsafe {
                                            self.ptr.as_mut().unwrap().get_member_u16(#field_offset)
                                        }
                                    }
                                    pub fn #setter_name(&mut self, value: u16) {
                                        unsafe {
                                            self.ptr.as_mut().unwrap().set_member_u16(#field_offset, value);
                                        }
                                    }
                                }
                            },
                            _ if field_type_str == "u32" => {
                                quote! {
                                    pub fn #getter_name(&self) -> #field_type {
                                        unsafe {
                                            self.ptr.as_mut().unwrap().get_member_u32(#field_offset)
                                        }
                                    }
                                    pub fn #setter_name(&mut self, value: u32) {
                                        unsafe {
                                            self.ptr.as_mut().unwrap().set_member_u32(#field_offset, value);
                                        }
                                    }
                                }
                            },
                            _ if field_type_str == "u64" => {
                                quote! {
                                    pub fn #getter_name(&self) -> #field_type {
                                        unsafe {
                                            self.ptr.as_mut().unwrap().get_member_u64(#field_offset)
                                        }
                                    }
                                    pub fn #setter_name(&mut self, value: u64) {
                                        unsafe {
                                            self.ptr.as_mut().unwrap().set_member_u64(#field_offset, value);
                                        }
                                    }
                                }
                            },
                            _ if field_type_str == "i8" => {
                                quote! {
                                    pub fn #getter_name(&self) -> #field_type {
                                        unsafe {
                                            self.ptr.as_mut().unwrap().get_member_i8(#field_offset)
                                        }
                                    }
                                    pub fn #setter_name(&mut self, value: i8) {
                                        unsafe {
                                            self.ptr.as_mut().unwrap().set_member_i8(#field_offset, value);
                                        }
                                    }
                                }
                            },
                            _ if field_type_str == "i16" => {
                                quote! {
                                    pub fn #getter_name(&self) -> #field_type {
                                        unsafe {
                                            self.ptr.as_mut().unwrap().get_member_i16(#field_offset)
                                        }
                                    }
                                    pub fn #setter_name(&mut self, value: i16) {
                                        unsafe {
                                            self.ptr.as_mut().unwrap().set_member_i16(#field_offset, value);
                                        }
                                    }
                                }
                            },
                            _ if field_type_str == "i32" => {
                                quote! {
                                    pub fn #getter_name(&self) -> #field_type {
                                        unsafe {
                                            self.ptr.as_mut().unwrap().get_member_i32(#field_offset)
                                        }
                                    }
                                    pub fn #setter_name(&mut self, value: i32) {
                                        unsafe {
                                            self.ptr.as_mut().unwrap().set_member_i32(#field_offset, value);
                                        }
                                    }
                                }
                            },
                            _ if field_type_str == "i64" => {
                                quote! {
                                    pub fn #getter_name(&self) -> #field_type {
                                        unsafe {
                                            self.ptr.as_mut().unwrap().get_member_i64(#field_offset)
                                        }
                                    }
                                    pub fn #setter_name(&mut self, value: i64) {
                                        unsafe {
                                            self.ptr.as_mut().unwrap().set_member_i64(#field_offset, value);
                                        }
                                    }
                                }
                            },
                            _ if field_type_str == "f32" => {
                                quote! {
                                    pub fn #getter_name(&self) -> #field_type {
                                        unsafe {
                                            self.ptr.as_mut().unwrap().get_member_f32(#field_offset)
                                        }
                                    }
                                    pub fn #setter_name(&mut self, value: f32) {
                                        unsafe {
                                            self.ptr.as_mut().unwrap().set_member_f32(#field_offset, value);
                                        }
                                    }
                                }
                            },
                            _ if field_type_str == "f64" => {
                                quote! {
                                    pub fn #getter_name(&self) -> #field_type {
                                        unsafe {
                                            self.ptr.as_mut().unwrap().get_member_f64(#field_offset)
                                        }
                                    }
                                    pub fn #setter_name(&mut self, value: f64) {
                                        unsafe {
                                            self.ptr.as_mut().unwrap().set_member_f64(#field_offset, value);
                                        }
                                    }
                                }
                            },
                            _ if field_type_str == "bool" => {
                                quote! {
                                    pub fn #getter_name(&self) -> #field_type {
                                        unsafe {
                                            self.ptr.as_mut().unwrap().get_member_bool(#field_offset)
                                        }
                                    }
                                    pub fn #setter_name(&mut self, value: bool) {
                                        unsafe {
                                            self.ptr.as_mut().unwrap().set_member_bool(#field_offset, value);
                                        }
                                    }
                                }
                            },
                            _ if field_type_str == "String" => {
                                quote! {
                                    pub fn #getter_name(&self) -> String {
                                        unsafe {
                                            self.ptr.as_mut().unwrap().get_member_string(#field_offset)
                                        }
                                    }
                                    pub fn #setter_name(&mut self, value: String) {
                                        unsafe {
                                            self.ptr.as_mut().unwrap().set_member_string(#field_offset, value);
                                        }
                                    }
                                }
                            },
                            _ if field_type_str == "U32Array" => {
                                quote! {
                                    pub fn #getter_name(&self) -> Vec<u32> {
                                        unsafe {
                                            self.ptr.as_mut().unwrap().get_member_u32_array(#field_offset)
                                        }
                                    }
                                    pub fn #setter_name(&mut self, value: Vec<u32>) {
                                        unsafe {
                                            self.ptr.as_mut().unwrap().set_member_u32_array(#field_offset, value);
                                        }
                                    }
                                }
                            },
                            _ if field_type_str == "F32Array" => {
                                quote! {
                                    pub fn #getter_name(&self) -> Vec<f32> {
                                        unsafe {
                                            self.ptr.as_mut().unwrap().get_member_f32_array(#field_offset)
                                        }
                                    }
                                    pub fn #setter_name(&mut self, value: Vec<f32>) {
                                        unsafe {
                                            self.ptr.as_mut().unwrap().set_member_f32_array(#field_offset, value);
                                        }
                                    }
                                }
                            },
                            _ if field_type_str == "Pointer" => {
                                quote! {
                                    pub fn #getter_name(&self) -> *const core::ffi::c_void {
                                        unsafe {
                                            // self.ptr.as_mut().unwrap().get_member_f32_array(#field_offset)
                                        }
                                    }
                                    pub fn #setter_name(&mut self, value: *const core::ffi::c_void) {
                                        unsafe {
                                            // self.ptr.as_mut().unwrap().set_member_f32_array(#field_offset, value);
                                        }
                                    }
                                }
                            },
                            _ => {
                                println!("Unsupported field type: {}", quote!(#field_type));
                                panic!("Unsupported field type for getter/setter");
                            }
                        }
                    });

            let struct_fields =
                field_names
                    .clone()
                    .zip(field_types.clone())
                    .map(|(field_name, field_type)| {
                        let field_type_str = format!("{}", quote!(#field_type));
                        match field_type_str.as_str() {
                            "Pointer" | "* mut c_void" | "U32Array" | "F32Array" => {
                                quote! {
                                    #[serde(skip)]
                                    pub #field_name: i64,
                                }
                            },
                            _ => {
                                quote! {
                                    pub #field_name: #field_type,
                                }
                            }
                        }
                    });

            let default_body =
                field_names
                    .clone()
                    .zip(field_types.clone())
                    .map(|(field_name, field_type)| {
                        quote! {
                            #field_name: #field_type::default(),
                        }
                    });

            let default_impl = quote! {
                impl Default for #name {
                    fn default() -> Self {
                        Self {
                            ptr: core::ptr::null_mut(),
                            singleton: false,
                            id: 0,
                            #(#default_body)*
                        }
                    }
                }
            };

            let struct_name_str = name.to_string();
            // let type_hash = fnv1a_hash_str(&struct_name_str);
            // let type_hash_fn = quote! {
            //     fn get_hash() -> u64 {
            //         #type_hash
            //     }
            // };

            // Create the register component tokens.
            let field_names_str = field_names.clone().map(|f| f.clone().unwrap().to_string());
            let field_types_code = field_types.clone().map(|f| get_type_code(f));
            // let register_component_tokens = quote! {
            //     use {ToxoidComponentType, ComponentDesc};
            //     let id = ToxoidComponentType::new(&Toxoid        ComponentDesc {
            //         name: #struct_name_str.to_string(),
            //         member_names: vec![#(#field_names_str),*],
            //         member_types: vec![#(#field_types_code),*],
            //     }); 
            //     id
            // };
            
            // // Create the register implementation.
            // let register_fn = quote! {
            //     fn register() -> u64 {
            //         #register_component_tokens
            //     }
            // };
            
            let type_name = struct_name_str.as_str();
            let type_name_fn = quote! {
                fn get_name() -> &'static str {
                    #type_name
                }
            };
            let type_get_id_fn = quote! {
                fn get_id() -> u64 {
                    get_component_id(#struct_name_str, vec![#(#field_names_str.to_string()),*], vec![#(#field_types_code),*])
                }
            };
            quote! {
                // #[derive(Clone, PartialEq, Serialize, Deserialize)]
                #[repr(C)]
                pub struct #name {
                    // #[serde(skip)]
                    ptr: *mut ToxoidComponent,
                    singleton: bool,
                    id: ecs_entity_t,
                    #(#struct_fields)*
                }

                #default_impl

                impl #name {
                    #(#getters_and_setters)*
                }

                impl ComponentType for #name {
                    // // Static methods
                    // #register_fn
                    // #type_hash_fn
                    #type_name_fn
                    #type_get_id_fn
                }

                impl Component for #name {
                    // Object compatible trait methods
                    // fn get_id(&self) -> u64 {
                    //     combine_u32(unsafe { &self.component.lookup(toxoid_make_c_string(#type_name)) }) 
                    // }

                    fn set_ptr(&mut self, ptr: *mut ToxoidComponent) {
                        self.ptr = ptr;
                    }

                    // fn get_ptr(&self) -> *mut core::ffi::c_void {
                    //     self.ptr
                    // }

                    // fn set_singleton(&mut self, singleton: bool) {
                    //     self.singleton = singleton;
                    // }

                    // fn get_singleton(&self) -> bool {
                    //     self.singleton
                    // }
                }
            }
            
        })
        .collect::<Vec<_>>();

    TokenStream::from(quote! {
        #(#expanded)*
    })
}

fn get_type_code(ty: &Type) -> u8 {
    match ty {
        Type::Path(tp) if tp.path.is_ident("u8") => FieldType::U8 as u8,
        Type::Path(tp) if tp.path.is_ident("u16") => FieldType::U16 as u8,
        Type::Path(tp) if tp.path.is_ident("u32") => FieldType::U32 as u8,
        Type::Path(tp) if tp.path.is_ident("u64") => FieldType::U64 as u8,
        Type::Path(tp) if tp.path.is_ident("i8") => FieldType::I8 as u8,
        Type::Path(tp) if tp.path.is_ident("i16") => FieldType::I16 as u8,
        Type::Path(tp) if tp.path.is_ident("i32") => FieldType::I32 as u8,
        Type::Path(tp) if tp.path.is_ident("i64") => FieldType::I64 as u8,
        Type::Path(tp) if tp.path.is_ident("f32") => FieldType::F32 as u8,
        Type::Path(tp) if tp.path.is_ident("f64") => FieldType::F64 as u8,
        Type::Path(tp) if tp.path.is_ident("bool") => FieldType::Bool as u8,
        Type::Path(tp) if tp.path.is_ident("String") => FieldType::String as u8,
        Type::Path(tp) if tp.path.is_ident("U32Array") => FieldType::U32Array as u8,
        Type::Path(tp) if tp.path.is_ident("F32Array") => FieldType::U32Array as u8,
        Type::Path(tp) if tp.path.is_ident("Pointer") => FieldType::Pointer as u8,
        Type::Path(tp) if tp.path.is_ident("StringPtr") => FieldType::String as u8,
        Type::Ptr(ptr) => {
            match *ptr.elem {
                Type::Path(ref tp) if tp.path.is_ident("u32") => {
                    FieldType::U32Array as u8
                },
                Type::Path(ref tp) if tp.path.is_ident("f32") => {
                    FieldType::F32Array as u8
                },
                Type::Path(ref tp) if tp.path.is_ident("c_void") => {
                    FieldType::Pointer as u8
                },
                _ => {
                    println!("Unsupported pointer type code: {}", quote!(#ptr));
                    panic!("Unsupported type code")
                }
            }
        }
        _ => {
            println!("Unsupported type: {}", quote!(#ty));
            panic!("Unsupported type code")
        }
    }
}

fn get_type_size(ty: &Type) -> u32 {
    match ty {
        Type::Path(tp) if tp.path.is_ident("u8") => 1,
        Type::Path(tp) if tp.path.is_ident("u16") => 2,
        Type::Path(tp) if tp.path.is_ident("u32") => 4,
        Type::Path(tp) if tp.path.is_ident("u64") => 8,
        Type::Path(tp) if tp.path.is_ident("i8") => 1,
        Type::Path(tp) if tp.path.is_ident("i16") => 2,
        Type::Path(tp) if tp.path.is_ident("i32") => 4,
        Type::Path(tp) if tp.path.is_ident("i64") => 8,
        Type::Path(tp) if tp.path.is_ident("f32") => 4,
        Type::Path(tp) if tp.path.is_ident("f64") => 8,
        Type::Path(tp) if tp.path.is_ident("bool") => 1,
        Type::Path(tp) if tp.path.is_ident("String") => 8,
        Type::Path(tp) if tp.path.is_ident("U32Array") => 8,
        Type::Path(tp) if tp.path.is_ident("F32Array") => 8,
        Type::Path(tp) if tp.path.is_ident("Pointer") => 8,
        Type::Path(tp) if tp.path.is_ident("StringPtr") => 8,
        Type::Ptr(ptr) => {
            match *ptr.elem {
                Type::Path(ref tp) if tp.path.is_ident("u32") => {
                    8
                },
                Type::Path(ref tp) if tp.path.is_ident("f32") => {
                    8
                },
                Type::Path(ref tp) if tp.path.is_ident("c_void") => {
                    8
                },
                _ => {
                    println!("Unsupported pointer field type: {}", quote!(#ptr));
                    panic!("Unsupported field type")
                }
            }
        }
        _ => {
            println!("Unsupported field type: {}", quote!(#ty));
            panic!("Unsupported field type")
        }
    }
}  

fn get_type_alignment(ty: &Type) -> u32 {
    match ty {
        Type::Path(tp) if tp.path.is_ident("u8") => 1,
        Type::Path(tp) if tp.path.is_ident("u16") => 2,
        Type::Path(tp) if tp.path.is_ident("u32") => 4,
        Type::Path(tp) if tp.path.is_ident("u64") => 8,
        Type::Path(tp) if tp.path.is_ident("i8") => 1,
        Type::Path(tp) if tp.path.is_ident("i16") => 2,
        Type::Path(tp) if tp.path.is_ident("i32") => 4,
        Type::Path(tp) if tp.path.is_ident("i64") => 8,
        Type::Path(tp) if tp.path.is_ident("f32") => 4,
        Type::Path(tp) if tp.path.is_ident("f64") => 8,
        Type::Path(tp) if tp.path.is_ident("bool") => 1,
        Type::Path(tp) if tp.path.is_ident("String") => 8, // Assuming String is a pointer
        Type::Path(tp) if tp.path.is_ident("U32Array") => 8, // Assuming U32Array is a pointer
        Type::Path(tp) if tp.path.is_ident("F32Array") => 8, // Assuming F32Array is a pointer
        Type::Path(tp) if tp.path.is_ident("Pointer") => 8, // Assuming Pointer is a pointer
        Type::Path(tp) if tp.path.is_ident("StringPtr") => 8, // Assuming StringPtr is a pointer
        Type::Ptr(_) => 8, // Pointers are 8 bytes in a 64-bit context
        _ => {
            println!("Unsupported field type: {}", quote!(#ty));
            panic!("Unsupported field type")
        }
    }
}