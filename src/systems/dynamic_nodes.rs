use crate::components::node_component::NodeComponent;
use crate::components::node_template::NodeTemplate;
use gdnative::prelude::*;
use legion::systems::CommandBuffer;
use legion::{component, system, Entity, World};
use std::process::Command;

#[derive(Debug, Clone, PartialEq)]
pub enum ManageErrs {
    CouldNotMakeInstance,
    RootClassNotSpatial(String),
}

#[system(for_each)]
#[filter(!component::<NodeComponent>())]
pub fn create_node(
    cmd: &mut CommandBuffer,
    #[state] unit_node: &Ref<Node2D>,
    entity: &Entity,
    template_data: &NodeTemplate,
) {
    let units_node = match unsafe { unit_node.assume_safe_if_sane() } {
        Some(node) => node,
        None => return,
    };

    let template = load_scene(&template_data.scene_file);

    let template = if let Some(template) = &template {
        template
    } else {
        godot_print!("Could not load scene: {}", &template_data.scene_file);
        return;
    };

    match instance_scene::<Node2D>(template) {
        Ok(node2d) => {
            let node2d: Ref<Node2D> = node2d.into_shared();
            unsafe {
                let node2d = node2d.assume_safe_if_sane().unwrap();
                node2d.set_z_index(template_data.z_index);
                node2d.set_z_as_relative(false);
                node2d.set_scale(Vector2::new(template_data.scale_x, template_data.scale_y));
            }
            units_node.add_child(node2d, false);

            let entity = *entity;
            cmd.exec_mut(move |world| {
                let mut entry = world.entry(entity).unwrap();
                entry.add_component(NodeComponent { node: node2d });
            })
        }
        Err(err) => godot_print!("Could not instance Child : {:?}", err),
    }
}

pub fn load_scene(path: &str) -> Option<Ref<PackedScene, ThreadLocal>> {
    let scene = ResourceLoader::godot_singleton().load(path, "PackedScene", false)?;

    let scene = unsafe { scene.assume_thread_local() };

    scene.cast::<PackedScene>()
}

#[allow(unused_qualifications)] //It is actually used/needed here, at least according to another rustc error.
fn instance_scene<Root>(scene: &PackedScene) -> Result<Ref<Root, Unique>, ManageErrs>
where
    Root: gdnative::GodotObject<RefKind = ManuallyManaged> + SubClass<Node>,
{
    let instance = scene
        .instance(PackedScene::GEN_EDIT_STATE_DISABLED)
        .ok_or(ManageErrs::CouldNotMakeInstance)?;
    let instance = unsafe { instance.assume_unique() };

    instance
        .try_cast::<Root>()
        .map_err(|instance| ManageErrs::RootClassNotSpatial(instance.name().to_string()))
}
