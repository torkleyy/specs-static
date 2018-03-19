//! A library that is meant to used together with Specs in cases
//! where you need another id strategy which isn't as flexible, but much faster
//! because there's no need for complex allocations.
//!
//! One specific example is a tile map, where each tile is basically like an entity.
//! Because we won't delete any tiles, we can optimize the allocator.
//! The current revision doesn't even need an allocator at all, the user can manage it
//! freely.
//!

#[macro_use]
extern crate derivative;
extern crate hibitset;
extern crate shred;
extern crate specs;

use std::hash::Hash;
use std::marker::PhantomData;

use hibitset::BitSet;
use shred::Resources;
use specs::{Component, Index, Join, UnprotectedStorage, World};

pub trait Id: Copy + Eq + Hash + Ord + Send + Sync + Sized + 'static {
    fn from_u32(value: u32) -> Self;

    fn id(&self) -> u32;
}

#[derive(Derivative)]
#[derivative(Default(bound = "D: Default"))]
pub struct Storage<C, D: UnprotectedStorage<C>, I> {
    data: D,
    bitset: BitSet,
    phantom: PhantomData<(C, I)>,
}

impl<C, D, I> Storage<C, D, I>
where
    C: Component,
    D: UnprotectedStorage<C>,
    I: Id,
{
    pub fn get(&self, id: I) -> Option<&C> {
        match self.bitset.contains(id.id()) {
            true => unsafe { Some(self.data.get(id.id())) },
            false => None,
        }
    }

    pub fn get_mut(&mut self, id: I) -> Option<&mut C> {
        match self.bitset.contains(id.id()) {
            true => unsafe { Some(self.data.get_mut(id.id())) },
            false => None,
        }
    }

    pub fn insert(&mut self, id: I, comp: C) -> Option<C> {
        let old = match self.bitset.add(id.id()) {
            true => unsafe { Some(self.data.remove(id.id())) },
            false => None,
        };

        unsafe {
            self.data.insert(id.id(), comp);
        }

        old
    }

    pub fn remove(&mut self, id: I) -> Option<C> {
        match self.bitset.remove(id.id()) {
            true => unsafe { Some(self.data.remove(id.id())) },
            false => None,
        }
    }
}

impl<C, D, I> Drop for Storage<C, D, I>
where
    D: UnprotectedStorage<C>,
{
    fn drop(&mut self) {
        let bitset = &self.bitset;

        unsafe {
            self.data.clean(|x| bitset.contains(x));
        }
    }
}

impl<'a, C, D, I> Join for &'a Storage<C, D, I>
where
    D: UnprotectedStorage<C>,
{
    type Type = &'a C;
    type Value = &'a D;
    type Mask = &'a BitSet;

    fn open(self) -> (Self::Mask, Self::Value) {
        (&self.bitset, &self.data)
    }

    unsafe fn get(value: &mut Self::Value, id: Index) -> Self::Type {
        (*value).get(id)
    }
}

impl<'a, C, D, I> Join for &'a mut Storage<C, D, I>
where
    D: UnprotectedStorage<C>,
{
    type Type = &'a mut C;
    type Value = &'a mut D;
    type Mask = &'a BitSet;

    fn open(self) -> (Self::Mask, Self::Value) {
        (&self.bitset, &mut self.data)
    }

    unsafe fn get(value: &mut Self::Value, id: Index) -> Self::Type {
        // This is horribly unsafe. Unfortunately, Rust doesn't provide a way
        // to abstract mutable/immutable state at the moment, so we have to hack
        // our way through it.
        let value: *mut Self::Value = value as *mut Self::Value;
        (*value).get_mut(id)
    }
}

pub trait WorldExt {
    fn register_tile_comp<C: Component + Send + Sync, I: Id>(&mut self);
}

impl WorldExt for World {
    fn register_tile_comp<C: Component + Send + Sync, I: Id>(&mut self) {
        self.add_resource(Storage::<C, C::Storage, I>::default());
    }
}
