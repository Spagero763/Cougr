use soroban_sdk::{contracttype, Bytes, Env, Map, Symbol, Vec};

/// Simple entity ID type for Soroban-optimized ECS.
pub type EntityId = u32;

/// Simplified game world optimized for Soroban on-chain storage.
///
/// Uses `Map`-based storage for O(log n) component lookups instead of
/// linear scans. This is the recommended ECS container for Soroban contracts.
///
/// # Storage layout
/// - `components`: `Map<(EntityId, Symbol), Bytes>` — direct component lookup
/// - `entity_components`: `Map<EntityId, Vec<Symbol>>` — tracks which components an entity has
///
/// # Example
/// ```ignore
/// let mut world = SimpleWorld::new(&env);
/// let entity_id = world.spawn_entity();
/// world.add_component(entity_id, symbol_short!("position"), pos.serialize(&env));
/// ```
#[contracttype]
#[derive(Clone, Debug)]
pub struct SimpleWorld {
    pub next_entity_id: EntityId,
    /// Component data keyed by (entity_id, component_type).
    pub components: Map<(EntityId, Symbol), Bytes>,
    /// Tracks which component types each entity has.
    pub entity_components: Map<EntityId, Vec<Symbol>>,
}

impl SimpleWorld {
    pub fn new(env: &Env) -> Self {
        Self {
            next_entity_id: 1,
            components: Map::new(env),
            entity_components: Map::new(env),
        }
    }

    pub fn spawn_entity(&mut self) -> EntityId {
        let id = self.next_entity_id;
        self.next_entity_id += 1;
        id
    }

    pub fn add_component(&mut self, entity_id: EntityId, component_type: Symbol, data: Bytes) {
        // Set (or overwrite) the component data
        self.components
            .set((entity_id, component_type.clone()), data);

        // Update the entity's component type list
        let mut types = self
            .entity_components
            .get(entity_id)
            .unwrap_or_else(|| Vec::new(self.components.env()));

        // Only add the type if not already present
        let mut found = false;
        for i in 0..types.len() {
            if let Some(t) = types.get(i) {
                if t == component_type {
                    found = true;
                    break;
                }
            }
        }
        if !found {
            types.push_back(component_type);
        }
        self.entity_components.set(entity_id, types);
    }

    pub fn get_component(&self, entity_id: EntityId, component_type: &Symbol) -> Option<Bytes> {
        self.components.get((entity_id, component_type.clone()))
    }

    pub fn remove_component(&mut self, entity_id: EntityId, component_type: &Symbol) -> bool {
        let removed = self.components.remove((entity_id, component_type.clone()));

        if removed.is_some() {
            // Update entity_components list
            if let Some(types) = self.entity_components.get(entity_id) {
                let env = self.components.env();
                let mut new_types = Vec::new(env);
                for i in 0..types.len() {
                    if let Some(t) = types.get(i) {
                        if &t != component_type {
                            new_types.push_back(t);
                        }
                    }
                }
                if new_types.is_empty() {
                    self.entity_components.remove(entity_id);
                } else {
                    self.entity_components.set(entity_id, new_types);
                }
            }
            true
        } else {
            false
        }
    }

    pub fn has_component(&self, entity_id: EntityId, component_type: &Symbol) -> bool {
        self.components
            .contains_key((entity_id, component_type.clone()))
    }

    pub fn get_entities_with_component(&self, component_type: &Symbol, env: &Env) -> Vec<EntityId> {
        let mut entities = Vec::new(env);
        for key in self.entity_components.keys().iter() {
            if let Some(types) = self.entity_components.get(key) {
                for i in 0..types.len() {
                    if let Some(t) = types.get(i) {
                        if &t == component_type {
                            entities.push_back(key);
                            break;
                        }
                    }
                }
            }
        }
        entities
    }

    pub fn despawn_entity(&mut self, entity_id: EntityId) {
        // Remove all components for this entity
        if let Some(types) = self.entity_components.get(entity_id) {
            for i in 0..types.len() {
                if let Some(t) = types.get(i) {
                    self.components.remove((entity_id, t));
                }
            }
        }
        self.entity_components.remove(entity_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{symbol_short, Env};

    #[test]
    fn test_simple_world_creation() {
        let env = Env::default();
        let world = SimpleWorld::new(&env);
        assert_eq!(world.next_entity_id, 1);
        assert_eq!(world.components.len(), 0);
    }

    #[test]
    fn test_spawn_entity() {
        let env = Env::default();
        let mut world = SimpleWorld::new(&env);
        let id1 = world.spawn_entity();
        let id2 = world.spawn_entity();
        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
    }

    #[test]
    fn test_add_and_get_component() {
        let env = Env::default();
        let mut world = SimpleWorld::new(&env);
        let entity_id = world.spawn_entity();

        let data = Bytes::from_array(&env, &[1, 2, 3, 4]);
        world.add_component(entity_id, symbol_short!("test"), data.clone());

        let retrieved = world.get_component(entity_id, &symbol_short!("test"));
        assert_eq!(retrieved, Some(data));
    }

    #[test]
    fn test_has_component() {
        let env = Env::default();
        let mut world = SimpleWorld::new(&env);
        let entity_id = world.spawn_entity();

        assert!(!world.has_component(entity_id, &symbol_short!("test")));

        let data = Bytes::from_array(&env, &[1]);
        world.add_component(entity_id, symbol_short!("test"), data);

        assert!(world.has_component(entity_id, &symbol_short!("test")));
    }

    #[test]
    fn test_remove_component() {
        let env = Env::default();
        let mut world = SimpleWorld::new(&env);
        let entity_id = world.spawn_entity();

        let data = Bytes::from_array(&env, &[1]);
        world.add_component(entity_id, symbol_short!("test"), data);

        assert!(world.remove_component(entity_id, &symbol_short!("test")));
        assert!(!world.has_component(entity_id, &symbol_short!("test")));
        assert!(!world.remove_component(entity_id, &symbol_short!("test")));
    }

    #[test]
    fn test_get_entities_with_component() {
        let env = Env::default();
        let mut world = SimpleWorld::new(&env);

        let e1 = world.spawn_entity();
        let e2 = world.spawn_entity();
        let e3 = world.spawn_entity();

        let data = Bytes::from_array(&env, &[1]);
        world.add_component(e1, symbol_short!("pos"), data.clone());
        world.add_component(e2, symbol_short!("pos"), data.clone());
        world.add_component(e3, symbol_short!("vel"), data);

        let entities = world.get_entities_with_component(&symbol_short!("pos"), &env);
        assert_eq!(entities.len(), 2);
    }

    #[test]
    fn test_despawn_entity() {
        let env = Env::default();
        let mut world = SimpleWorld::new(&env);
        let entity_id = world.spawn_entity();

        let data = Bytes::from_array(&env, &[1]);
        world.add_component(entity_id, symbol_short!("a"), data.clone());
        world.add_component(entity_id, symbol_short!("b"), data);

        world.despawn_entity(entity_id);
        assert!(!world.has_component(entity_id, &symbol_short!("a")));
        assert!(!world.has_component(entity_id, &symbol_short!("b")));
    }

    #[test]
    fn test_add_component_replaces_existing() {
        let env = Env::default();
        let mut world = SimpleWorld::new(&env);
        let entity_id = world.spawn_entity();

        let data1 = Bytes::from_array(&env, &[1]);
        let data2 = Bytes::from_array(&env, &[2]);

        world.add_component(entity_id, symbol_short!("test"), data1);
        world.add_component(entity_id, symbol_short!("test"), data2.clone());

        let retrieved = world.get_component(entity_id, &symbol_short!("test"));
        assert_eq!(retrieved, Some(data2));
    }
}
