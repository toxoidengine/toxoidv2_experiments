# WIT File Differences: WASM Guest vs Native Host

## Key Architectural Differences

### toxoid_wasm_component/wit/world.wit (Guest Interface)
- **Purpose**: Provides a safe interface for WASM components to interact with host resources
- **Memory Model**: Works with handles/resources that abstract away direct memory access
- **Entity Handling**: Returns complete `entity` resource objects that encapsulate the functionality

```wit:toxoid_wasm_component/wit/world.wit
// Entity returns a full resource object that can be safely used in WASM
resource entity {
    constructor(init: entity-desc);
    from-id: static func(id: u64) -> entity;  // Returns a resource handle
    get-id: func() -> ecs-entity-t;
}

// Query returns list of entity resources
resource query {
    entities: func() -> list<entity>;  // Returns safe entity handles
}
```

### toxoid_engine/wit/world.wit (Host Interface)
- **Purpose**: Provides direct interface for native code to interact with the ECS
- **Memory Model**: Works with raw pointers and memory addresses
- **Entity Handling**: Returns raw entity IDs and memory pointers

```wit:toxoid_engine/wit/world.wit
resource entity {
    from-id: static func(id: u64) -> s64;  // Returns raw pointer
    get: func(component: ecs-entity-t) -> s64;  // Returns raw pointer
}

// Query returns raw entity IDs
resource query {
    entities: func() -> list<ecs-entity-t>;  // Returns raw IDs
}
```

## Notable Implementation Differences

1. **System Description**
   - **WASM Guest**: Uses a `callback` resource type for safety
   - **Host**: Uses raw `s64` pointers for callbacks and query handles

2. **Memory Access**
   - **WASM Guest**: All memory access is mediated through resource methods
   - **Host**: Allows direct pointer manipulation with `s64` return types

3. **Component Access**
   - **WASM Guest**: Components are accessed through safe resource handles
   - **Host**: Components are accessed through raw pointers and ECS entity IDs

## Safety Considerations

The WASM guest interface (`toxoid_wasm_component`) provides several safety guarantees:
1. No direct memory access
2. Resource-based abstraction for all operations
3. Safe entity and component handles

The host interface (`toxoid_engine`) prioritizes performance and direct access:
1. Raw pointer access for native code
2. Direct ECS entity ID manipulation
3. Minimal abstraction overhead

This separation ensures that WASM components can't accidentally corrupt memory while allowing native code to maintain full performance when needed.

## Example Implementation:
```rust:crates/toxoid_api/src/lib.rs
impl Iter {
    pub fn entities(&self) -> Vec<Entity> {
        self.iter
            .entities()
            .iter()
            .map(|entity| {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    // In native mode, we get a u64 ID directly
                    Entity { 
                        entity: ToxoidEntity::from_id(entity.get_id())
                    }
                }
                #[cfg(target_arch = "wasm32")]
                {
                    // In WASM mode, we're working with the guest component object
                    Entity {
                        entity: entity.clone()
                    }
                }
            })
            .collect()
    }
}
```

This change:
1. Uses conditional compilation to handle the different entity representations
2. For native, keeps the ID-based entity creation
3. For WASM, clones the guest component object directly

The key difference is that in WASM mode, we're working with actual component objects that need to be cloned, while in native mode we're working with numeric IDs that can be converted directly into an entity ID u64 which effectively acts like a pointer in the ECS[^1].

Note [^1]: If we were not able to access the memory through ECS, we would have directly used a pointer to the entity instead, which we'd also pass back and forth as a u64. The ECS acts as a cache efficient way to access the memory.




