#[derive(Debug)]
pub enum DragState {
    NotDragging,
    StartDraging,
    Dragging(f32),
}