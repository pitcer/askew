use indexmap::{indexmap, IndexMap};
use serde::{Deserialize, Deserializer};

use crate::canvas::shape::Shape;
use crate::config::CanvasConfig;
use crate::id_assigner::IdAssigner;

#[derive(Debug, serde::Serialize)]
pub struct Objects {
    #[serde(flatten)]
    objects: IndexMap<ObjectId, Object>,
    #[serde(skip_serializing)]
    assigner: IdAssigner,
}

pub type Object = Shape;
pub type ObjectId = usize;

impl Objects {
    #[must_use]
    pub fn new(config: &CanvasConfig) -> Self {
        let mut assigner = IdAssigner::new();

        let id = assigner.assign_id();
        let object = Object::new(config.default_curve_type, config);

        let objects = indexmap![id => object];

        Self { objects, assigner }
    }

    pub fn ids(&self) -> impl Iterator<Item = ObjectId> + '_ {
        self.objects.keys().copied()
    }

    pub fn objects(&self) -> impl Iterator<Item = &Object> {
        self.objects.values()
    }

    pub fn objects_mut(&mut self) -> impl Iterator<Item = &mut Object> {
        self.objects.values_mut()
    }

    pub fn add(&mut self, object: Object) -> ObjectId {
        let id = self.assigner.assign_id();
        let already_present = self.objects.insert(id, object);
        debug_assert!(already_present.is_none(), "object with id {id} is already assigned");
        id
    }

    pub fn remove(&mut self, id: ObjectId) {
        self.objects.remove(&id);
        self.assigner.remove_id(id);
    }

    #[must_use]
    pub fn get(&self, id: ObjectId) -> Option<&Object> {
        self.objects.get(&id)
    }

    pub fn get_mut(&mut self, id: ObjectId) -> Option<&mut Object> {
        self.objects.get_mut(&id)
    }

    #[must_use]
    pub fn length(&self) -> usize {
        self.objects.len()
    }
}

impl<'de> Deserialize<'de> for Objects {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let objects = IndexMap::deserialize(deserializer)?;
        let ids = objects.keys().copied();
        let assigner = IdAssigner::from_assigned_ids(ids);
        Ok(Self { objects, assigner })
    }
}
