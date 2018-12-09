use super::*;
use std::collections::HashMap;
use std::any::TypeId;
use std::any::Any;

pub struct Entry {
    pub is_live: bool,
    pub generation: u64
}

pub struct EntityStorage {
    pub storage: HashMap<TypeId, Vec<Option<Box<Any>>>>,
    pub entity_list: Vec<Entry>,
    pub free_list: Vec<usize>,
}

impl EntityStorage{

    pub fn register_new_entity(&mut self) -> EntityIndex {
        if let Some(x) = self.free_list.pop() {
            self.entity_list[x] = Entry{is_live: true, generation: 0};
            (x, 0)
        }else{
            self.entity_list.push(Entry { is_live: true, generation: 0 });
            let entity = (self.entity_list.len(), 0);
            for (_, component) in self.storage.iter_mut(){
                component.push(None);
            }
            entity
        }
    }

    pub fn deallocate_entity(&mut self, id: EntityIndex) -> Result<(), &str> {
        if id.1 == self.entity_list[id.0].generation {
            self.entity_list[id.0].is_live = false;
            self.free_list.push(id.0);
            for (_, component) in self.storage.iter_mut(){
                component[id.0] = None;
            }
            Ok(())
        }else{
            Err("incorrect generation")
        }
    }

    pub fn add_component<T: 'static>(&mut self, index: EntityIndex, component: T) -> Result<EntityIndex, &str>{
        if index.1 == self.entity_list[index.0].generation {
            if let Some(comp) = self.storage.get_mut(&TypeId::of::<T>()) {
                if let None = comp[index.0] {
                    comp[index.0] = Some(Box::new(component));
                    self.entity_list[index.0].generation += 1;
                    Ok((index.0, self.entity_list[index.0].generation))
                } else {
                    Err("unable to add component")
                }
            } else {
                Err("no component of this type")
            }
        }else{
            Err("incorrect generation")
        }
    }

    pub fn remove_component<T: 'static>(&mut self, index: EntityIndex) -> Result<EntityIndex, &str>{
        if index.1 != self.entity_list[index.0].generation {
            Err("incorrect generation")
        }else{
            if let Some(x) = self.storage.get_mut(&TypeId::of::<T>()){
                x[index.0] = None;
                self.entity_list[index.0].generation += 1;
            }
            Ok((index.0, self.entity_list[index.0].generation))
        }
    }

    pub fn fetch<T: 'static>(&mut self, id: EntityIndex) -> Result<Option<&mut T>, &str> {
        if id.1 != self.entity_list[id.0].generation{
            Err("incorrect generation")
        }else{
            let component = self.storage.get_mut(&TypeId::of::<T>()).unwrap();
            let unwrapped_component = component[id.0].as_mut().unwrap();
            let downcast: Option<&mut T> = unwrapped_component.downcast_mut::<T>();
            Ok(downcast)
        }
    }

    pub fn new() -> EntityStorage {
        EntityStorage{storage: HashMap::new(), free_list: Vec::new(), entity_list: Vec::new()}
    }
}