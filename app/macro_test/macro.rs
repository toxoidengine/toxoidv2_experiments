warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   C:\Users\troye\dev\toxoid\toxoidv2_experiments\crates\toxoid_engine\Cargo.toml
workspace: C:\Users\troye\dev\toxoid\toxoidv2_experiments\Cargo.toml
warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   C:\Users\troye\dev\toxoid\toxoidv2_experiments\crates\toxoid_wasm_component\Cargo.toml
workspace: C:\Users\troye\dev\toxoid\toxoidv2_experiments\Cargo.toml
warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   C:\Users\troye\dev\toxoid\toxoidv2_experiments\app\guest\Cargo.toml
workspace: C:\Users\troye\dev\toxoid\toxoidv2_experiments\Cargo.toml
warning: virtual workspace defaulting to `resolver = "1"` despite one or more workspace members being on edition 2021 which implies `resolver = "2"`
note: to keep the current resolver, specify `workspace.resolver = "1"` in the workspace root's manifest
note: to use the edition 2021 resolver, specify `workspace.resolver = "2"` in the workspace root's manifest
note: for more details see https://doc.rust-lang.org/cargo/reference/resolver.html#resolver-versions
warning: C:\Users\troye\dev\toxoid\toxoidv2_experiments\Cargo.toml: unused manifest key: workspace.name
   Compiling syn v1.0.109
   Compiling toxoid_api_macro v0.1.0 (C:\Users\troye\dev\toxoid\toxoidv2_experiments\crates\toxoid_api_macro)
error: unexpected closing delimiter: `}`
   --> crates\toxoid_api_macro\src\lib.rs:393:1
    |
352 | fn get_type_size(ty: &Type) -> u32 {
    |                                    - this opening brace...
...
392 | }  
    | - ...matches this closing brace
393 | }
    | ^ unexpected closing delimiter
error: could not compile `toxoid_api_macro` (lib) due to 1 previous error
