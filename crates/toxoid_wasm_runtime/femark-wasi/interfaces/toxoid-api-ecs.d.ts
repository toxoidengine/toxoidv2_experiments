export namespace ToxoidApiEcs {
  export { Component };
}
export type EcsEntityT = bigint;
/**
 * # Variants
 * 
 * ## `"u8-t"`
 * 
 * ## `"u16-t"`
 * 
 * ## `"u32-t"`
 * 
 * ## `"u64-t"`
 * 
 * ## `"i8-t"`
 * 
 * ## `"i16-t"`
 * 
 * ## `"i32-t"`
 * 
 * ## `"i64-t"`
 * 
 * ## `"f32-t"`
 * 
 * ## `"f64-t"`
 * 
 * ## `"bool-t"`
 * 
 * ## `"string-t"`
 * 
 * ## `"array-t"`
 * 
 * ## `"u32array-t"`
 * 
 * ## `"f32array-t"`
 */
export type MemberType = 'u8-t' | 'u16-t' | 'u32-t' | 'u64-t' | 'i8-t' | 'i16-t' | 'i32-t' | 'i64-t' | 'f32-t' | 'f64-t' | 'bool-t' | 'string-t' | 'array-t' | 'u32array-t' | 'f32array-t';
export interface ComponentDesc {
  name: string,
  memberNames: Array<string>,
  memberTypes: Uint8Array,
}

export class Component {
  constructor(init: ComponentDesc)
  getId(): EcsEntityT;
}
