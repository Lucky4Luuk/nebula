use legion::storage::IntoComponentSource;
use legion::Entity;
use legion::World;

pub struct Scene {
    world: World,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            world: World::default(),
        }
    }

    pub fn add_object<T>(&mut self, components: T) -> Entity
    where
        Option<T>: IntoComponentSource
    {
        self.world.push(components)
    }

    pub fn add_objects(&mut self, components: impl IntoComponentSource) -> &[Entity] {
        self.world.extend(components)
    }
}
