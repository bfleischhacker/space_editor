use bevy::prelude::*;
use bevy_scene_hook::{HookedSceneBundle, SceneHook};

use super::component::*;

pub fn spawn_scene(
    mut commands : Commands,
    prefabs : Query<(Entity, &GltfPrefab, Option<&Children>, Option<&Visibility>, Option<&Transform>), Changed<GltfPrefab>>,
    auto_childs : Query<&ScaneAutoChild>,
    asset_server : Res<AssetServer>
) {
    for (e, prefab, children, vis, tr) in prefabs.iter() {

        if let Some(children) = children {
            for e in children {
                if auto_childs.contains(*e) {
                    commands.entity(*e).despawn_recursive();
                }
            }
        }

        let id = commands.spawn(HookedSceneBundle {
              scene : SceneBundle { 
                scene: asset_server.load(format!("{}#{}", &prefab.path, &prefab.scene)), 
                ..default() },
                hook : SceneHook::new(|e, cmd| {
                    cmd.insert(ScaneAutoChild);
                })
             })
             .insert(ScaneAutoChild)
            .id();
        commands.entity(e).add_child(id);

        if vis.is_none() {
            commands.entity(e).insert(VisibilityBundle::default());
        }
        if tr.is_none() {
            commands.entity(e).insert(TransformBundle::default());
        }
    }
}

pub fn sync_mesh(
    mut commands : Commands,
    query : Query<(Entity, &MeshPrimitivePrefab), Changed<MeshPrimitivePrefab>>,
    mut meshs : ResMut<Assets<Mesh>>
) {
    for (e, pref) in query.iter() {
        let mesh = meshs.add(pref.to_mesh());
        commands.entity(e).insert(mesh);
    }
}

pub fn sync_material(
    mut commands : Commands,
    query : Query<(Entity, &MaterialPrefab), Changed<MaterialPrefab>>,
    mut materials : ResMut<Assets<StandardMaterial>>
) {
    for (e, pref) in query.iter() {
        let mat = materials.add(StandardMaterial {
             base_color: pref.color, 
            ..default()
            });
        commands.entity(e).insert(mat);
    }
}