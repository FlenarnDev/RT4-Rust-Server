use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashSet;
use std::marker::PhantomData;
use std::rc::Rc;
use crate::entity::entity::EntityExt;
use crate::entity::network_player::NetworkPlayer;

pub struct EntityList<T: EntityExt> {
    entities: Vec<Option<Rc<RefCell<T>>>>,
    ids: Vec<i32>,
    free: HashSet<usize>,
    index_padding: usize,
    last_used_index: usize,
    phantom: PhantomData<T>,
}

impl<T: EntityExt> EntityList<T> {
    pub fn new(size: usize, index_padding: usize) -> Self {
        let mut free = HashSet::with_capacity(size);
        for i in 0..size {
            free.insert(i);
        }

        // Create a Vec with None values without requiring Clone
        let mut entities = Vec::with_capacity(size);
        for _ in 0..size {
            entities.push(None);
        }

        EntityList {
            entities,
            ids: vec![-1; size],
            free,
            index_padding,
            last_used_index: 0,
            phantom: PhantomData,
        }
    }

    pub fn next(&self, _priority: bool, start: Option<usize>) -> Result<usize, &'static str> {
        let start = start.unwrap_or(self.last_used_index + 1);

        // First try searching from start to the end
        for index in start..self.ids.len() {
            if self.ids[index] == -1 {
                return Ok(index);
            }
        }

        // If not found, search from index_padding to start
        for index in self.index_padding..start {
            if self.ids[index] == -1 {
                return Ok(index);
            }
        }

        Err("No space for new entities")
    }

    pub fn count(&self) -> usize {
        self.entities.len() - self.free.len()
    }

    // Get a reference to an entity
    pub fn get(&self, id: usize) -> Option<Ref<T>> {
        if id >= self.ids.len() {
            return None;
        }

        let index = self.ids[id];
        if index == -1 {
            None
        } else {
            // Convert the Rc<RefCell<T>> to a Ref<T>
            self.entities[index as usize].as_ref().map(|rc| rc.borrow())
        }
    }

    // Get a mutable reference to an entity
    pub fn get_mut(&self, id: usize) -> Option<RefMut<T>> {
        if id >= self.ids.len() {
            return None;
        }

        let index = self.ids[id];
        if index == -1 {
            None
        } else {
            // Convert the Rc<RefCell<T>> to a RefMut<T>
            self.entities[index as usize].as_ref().map(|rc| rc.borrow_mut())
        }
    }

    pub fn set(&mut self, id: usize, entity: T) -> Result<(), &'static str> {
        if self.free.is_empty() {
            return Err("Cannot find available entities slot");
        }

        // Take the first free index
        let index = *self.free.iter().next().unwrap();
        self.free.remove(&index);

        // Set the id mapping and entity
        if id >= self.ids.len() {
            return Err("ID out of bounds");
        }

        self.ids[id] = index as i32;
        self.entities[index] = Some(Rc::new(RefCell::new(entity)));
        self.last_used_index = id;

        Ok(())
    }

    pub fn remove(&mut self, id: usize) {
        if id >= self.ids.len() {
            return;
        }

        let index = self.ids[id];
        if index != -1 {
            self.ids[id] = -1;
            self.free.insert(index as usize);
            self.entities[index as usize] = None;
        }
    }

    pub fn reset(&mut self) {
        for entity in self.entities.iter_mut() {
            *entity = None;
        }

        for id in self.ids.iter_mut() {
            *id = -1;
        }

        self.free.clear();
        for i in 0..self.entities.len() {
            self.free.insert(i);
        }

        self.last_used_index = 0;
    }

    // Iterator implementation
    pub fn iter(&self) -> EntityIterator<T> {
        EntityIterator {
            entity_list: self,
            current_index: 0,
        }
    }

    pub fn for_each<F>(&self, mut f: F)
    where
        F: FnMut(&T),
    {
        for id in 0..self.ids.len() {
            if self.ids[id] != -1 {
                if let Some(entity) = &self.entities[self.ids[id] as usize] {
                    f(&entity.borrow());
                }
            }
        }
    }

    // This method applies a mutable function to each entity
    pub fn for_each_mut<F>(&self, mut f: F)
    where
        F: FnMut(&mut T),
    {
        for id in 0..self.ids.len() {
            if self.ids[id] != -1 {
                if let Some(entity) = &self.entities[self.ids[id] as usize] {
                    f(&mut entity.borrow_mut());
                }
            }
        }
    }
}

pub struct EntityIterator<'a, T: EntityExt> {
    entity_list: &'a EntityList<T>,
    current_index: usize,
}

impl<'a, T: EntityExt> Iterator for EntityIterator<'a, T> {
    type Item = Ref<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.current_index < self.entity_list.ids.len() {
            let id = self.entity_list.ids[self.current_index];
            self.current_index += 1;

            if id != -1 {
                if let Some(rc) = &self.entity_list.entities[id as usize] {
                    return Some(rc.borrow());
                }
            }
        }
        None
    }
}

pub struct NetworkPlayerList {
    inner: EntityList<NetworkPlayer>,
}

impl NetworkPlayerList {
    pub fn new(size: usize) -> Self {
        NetworkPlayerList {
            inner: EntityList::new(size, 1),
        }
    }

    pub fn next(&self, priority: bool, start: Option<usize>) -> Result<usize, &'static str> {
        let start_index = start.unwrap_or(self.inner.last_used_index + 1);

        if priority {
            // Start searching at 1 if the calculated start is 0
            let init = if start_index == 0 { 1 } else { 0 };

            for i in init..100 {
                let index = start_index + i;
                if index >= self.inner.ids.len() {
                    break;
                }

                if self.inner.ids[index] == -1 {
                    return Ok(index);
                }
            }
        }

        // Fall back to the base implementation
        self.inner.next(false, Some(start_index))
    }

    pub fn count(&self) -> usize {
        self.inner.count()
    }

    pub fn get(&self, id: usize) -> Option<Ref<NetworkPlayer>> {
        self.inner.get(id)
    }

    pub fn get_mut(&self, id: usize) -> Option<RefMut<NetworkPlayer>> {
        self.inner.get_mut(id)
    }

    pub fn set(&mut self, id: usize, entity: NetworkPlayer) -> Result<(), &'static str> {
        self.inner.set(id, entity)
    }

    pub fn remove(&mut self, id: usize) {
        self.inner.remove(id)
    }

    pub fn reset(&mut self) {
        self.inner.reset()
    }

    pub fn iter(&self) -> EntityIterator<NetworkPlayer> {
        self.inner.iter()
    }

    pub fn for_each<F>(&self, f: F)
    where
        F: FnMut(&NetworkPlayer),
    {
        self.inner.for_each(f)
    }

    pub fn for_each_mut<F>(&self, f: F)
    where
        F: FnMut(&mut NetworkPlayer),
    {
        self.inner.for_each_mut(f)
    }
}