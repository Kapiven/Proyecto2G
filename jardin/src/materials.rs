use bevy::prelude::*;

#[derive(Resource)]
pub struct Materials {
    pub grass: Handle<StandardMaterial>,
    pub wood: Handle<StandardMaterial>,
    pub stone: Handle<StandardMaterial>,
    pub glass: Handle<StandardMaterial>,
    pub water: Handle<StandardMaterial>,
    
    pub metal: Handle<StandardMaterial>,
    
    pub lantern_glass: Handle<StandardMaterial>,
    
    pub sky: Handle<StandardMaterial>,
}
