use std::any::{Any, TypeId};
use std::collections::HashMap;

/// Generational Entity Identifier preventing stale references (ABA problem).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityId {
    pub index: u32,
    pub generation: u32,
}

#[derive(Debug, Clone)]
enum Slot {
    Free { next_free: Option<u32>, generation: u32 },
    Occupied { generation: u32 },
}

/// Generational Arena for high-performance, $O(1)$ entity allocation.
#[derive(Debug, Default)]
pub struct GenerationalArena {
    slots: Vec<Slot>,
    free_head: Option<u32>,
    active_count: usize,
}

impl GenerationalArena {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn spawn(&mut self) -> EntityId {
        self.active_count += 1;
        if let Some(index) = self.free_head {
            match self.slots[index as usize] {
                Slot::Free { next_free, generation } => {
                    self.free_head = next_free;
                    self.slots[index as usize] = Slot::Occupied { generation };
                    EntityId { index, generation }
                }
                Slot::Occupied { .. } => unreachable!(),
            }
        } else {
            let index = self.slots.len() as u32;
            let generation = 0;
            self.slots.push(Slot::Occupied { generation });
            EntityId { index, generation }
        }
    }

    pub fn despawn(&mut self, entity: EntityId) -> bool {
        if !self.is_alive(entity) {
            return false;
        }

        self.active_count -= 1;
        let old_gen = match self.slots[entity.index as usize] {
            Slot::Occupied { generation } => generation,
            Slot::Free { .. } => return false,
        };

        let next_gen = old_gen.wrapping_add(1);
        self.slots[entity.index as usize] = Slot::Free {
            next_free: self.free_head,
            generation: next_gen,
        };
        self.free_head = Some(entity.index);

        true
    }

    pub fn is_alive(&self, entity: EntityId) -> bool {
        if let Some(slot) = self.slots.get(entity.index as usize) {
            match slot {
                Slot::Occupied { generation } => *generation == entity.generation,
                Slot::Free { .. } => false,
            }
        } else {
            false
        }
    }

    pub fn len(&self) -> usize {
        self.active_count
    }

    pub fn is_empty(&self) -> bool {
        self.active_count == 0
    }
}

/// Generic type-erased component storage container.
pub trait AnyComponentPool: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn remove_entity(&mut self, entity: EntityId);
}

#[derive(Default)]
pub struct ComponentPool<T> {
    components: HashMap<EntityId, T>,
}

impl<T: 'static> ComponentPool<T> {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    #[inline]
    pub fn insert(&mut self, entity: EntityId, component: T) {
        self.components.insert(entity, component);
    }

    #[inline]
    pub fn get(&self, entity: EntityId) -> Option<&T> {
        self.components.get(&entity)
    }

    #[inline]
    pub fn get_mut(&mut self, entity: EntityId) -> Option<&mut T> {
        self.components.get_mut(&entity)
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (EntityId, &T)> {
        self.components.iter().map(|(&e, c)| (e, c))
    }

    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (EntityId, &mut T)> {
        self.components.iter_mut().map(|(&e, c)| (e, c))
    }
}

impl<T: 'static> AnyComponentPool for ComponentPool<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn remove_entity(&mut self, entity: EntityId) {
        self.components.remove(&entity);
    }
}

/// The ECS World managing entities and component storages.
#[derive(Default)]
pub struct World {
    arena: GenerationalArena,
    pools: HashMap<TypeId, Box<dyn AnyComponentPool>>,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn spawn(&mut self) -> EntityId {
        self.arena.spawn()
    }

    pub fn despawn(&mut self, entity: EntityId) -> bool {
        if self.arena.despawn(entity) {
            for pool in self.pools.values_mut() {
                pool.remove_entity(entity);
            }
            true
        } else {
            false
        }
    }

    pub fn is_alive(&self, entity: EntityId) -> bool {
        self.arena.is_alive(entity)
    }

    pub fn add_component<T: 'static>(&mut self, entity: EntityId, component: T) {
        if !self.is_alive(entity) {
            return;
        }
        let type_id = TypeId::of::<T>();
        let pool = self
            .pools
            .entry(type_id)
            .or_insert_with(|| Box::new(ComponentPool::<T>::new()));
        
        pool.as_any_mut()
            .downcast_mut::<ComponentPool<T>>()
            .unwrap()
            .insert(entity, component);
    }

    pub fn get_component<T: 'static>(&self, entity: EntityId) -> Option<&T> {
        if !self.is_alive(entity) {
            return None;
        }
        self.pools
            .get(&TypeId::of::<T>())?
            .as_any()
            .downcast_ref::<ComponentPool<T>>()?
            .get(entity)
    }

    pub fn get_component_mut<T: 'static>(&mut self, entity: EntityId) -> Option<&mut T> {
        if !self.is_alive(entity) {
            return None;
        }
        self.pools
            .get_mut(&TypeId::of::<T>())?
            .as_any_mut()
            .downcast_mut::<ComponentPool<T>>()?
            .get_mut(entity)
    }

    pub fn pool<T: 'static>(&self) -> Option<&ComponentPool<T>> {
        self.pools
            .get(&TypeId::of::<T>())?
            .as_any()
            .downcast_ref::<ComponentPool<T>>()
    }

    pub fn pool_mut<T: 'static>(&mut self) -> Option<&mut ComponentPool<T>> {
        self.pools
            .get_mut(&TypeId::of::<T>())?
            .as_any_mut()
            .downcast_mut::<ComponentPool<T>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Position {
        x: f32,
        _y: f32,
    }

    struct Velocity {
        _vx: f32,
        _vy: f32,
    }

    #[test]
    fn test_ecs_basic() {
        let mut world = World::new();
        let e1 = world.spawn();

        world.add_component(e1, Position { x: 10.0, _y: 20.0 });
        world.add_component(e1, Velocity { _vx: 1.0, _vy: -1.0 });

        assert!(world.is_alive(e1));
        assert_eq!(world.get_component::<Position>(e1).unwrap().x, 10.0);

        world.despawn(e1);
        assert!(!world.is_alive(e1));
        assert!(world.get_component::<Position>(e1).is_none());
    }
}
