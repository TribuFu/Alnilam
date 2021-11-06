use crate::Display;
use anyhow::Result;
use std::time::Duration;
use winit::event::*;

pub trait State: Sized + 'static {
    fn Init(display: &Display) -> Result<Self>;
    fn Input(&mut self, display: &Display, event: &WindowEvent) -> bool;
    fn Update(&mut self, display: &Display, delta: Duration);
    fn Resize(&mut self, display: &Display);
    fn Draw(&mut self, display: &mut Display) -> Result<(), wgpu::SurfaceError>;
}
