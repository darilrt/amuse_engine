use amuse_macro::*;

#[derive(Event)]
pub struct LoopInit;

#[derive(Event)]
pub struct LoopUpdate;

#[derive(Event)]
pub struct LoopPostUpdate;

#[derive(Event)]
pub struct LoopRender;

#[derive(Event)]
pub struct LoopExit;
