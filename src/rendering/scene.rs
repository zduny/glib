use glium::*;
use std::rc::Rc;

use super::super::geometry::GpuGeometry;
use super::super::math::*;
use super::*;

pub struct Mesh<M: Material> {
    geometry: Rc<GpuGeometry>,
    material: Rc<M>,
}

#[allow(dead_code)]
impl<M: Material> Mesh<M> {
    pub fn new(geometry: Rc<GpuGeometry>, material: Rc<M>) -> Mesh<M> {
        Mesh { geometry, material }
    }

    pub fn get_geometry(&self) -> &GpuGeometry {
        &self.geometry
    }

    pub fn get_geometry_rc(&self) -> Rc<GpuGeometry> {
        Rc::clone(&self.geometry)
    }

    pub fn get_material(&self) -> &M {
        &self.material
    }

    pub fn get_material_rc(&self) -> Rc<M> {
        Rc::clone(&self.material)
    }
}

impl<M: Material> Drawable for Mesh<M> {
    fn draw(
        &self,
        drawer: &mut dyn Drawer,
        transform: &Transform,
        camera: &Camera,
    ) -> Result<(), DrawError> {
        let geometry = self.get_geometry();
        let material = self.get_material();

        drawer.draw_single(
            &geometry.vertex_buffer,
            &geometry.index_buffer,
            &transform,
            &camera,
            material,
        )
    }
}

pub trait Transformable {
    fn set_transform(&mut self, transform: Transform);

    fn get_transform(&self) -> Transform;

    fn prepend_transform(&mut self, transform: Transform) {
        self.set_transform(self.get_transform() * transform);
    }

    fn append_transform(&mut self, transform: Transform) {
        self.set_transform(transform * self.get_transform());
    }

    fn to_local_space(&self, position: Position3) -> Position3 {
        self.get_transform().inverse() * position
    }
}

pub struct Object {
    pub transform: Transform,
    children: Vec<Rc<dyn Drawable>>,
    _private: (),
}

impl<'a> Object {
    pub fn new() -> Object {
        Object {
            transform: IDENTITY,
            children: vec![],
            _private: (),
        }
    }

    pub fn add(&mut self, child: Rc<dyn Drawable>) -> &Self {
        self.children.push(child);
        self
    }

    pub fn remove(&mut self, child: Rc<dyn Drawable>) -> &Self {
        self.children.retain(|ch| !Rc::ptr_eq(ch, &child));
        self
    }
}

impl Transformable for Object {
    fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }

    fn get_transform(&self) -> Transform {
        self.transform
    }
}

impl Drawable for Object {
    fn draw(
        &self,
        drawer: &mut dyn Drawer,
        transform: &Transform,
        camera: &Camera,
    ) -> Result<(), DrawError> {
        let multiplied_transform = *transform * self.transform;
        self.children
            .iter()
            .map(|child| child.draw(drawer, &multiplied_transform, camera))
            .collect()
    }
}

pub struct Scene {
    background_color: Color,
    root: Object,
}

#[allow(dead_code)]
impl Scene {
    pub fn new() -> Scene {
        let background_color = rgba(0.0, 0.0, 0.0, 0.0);
        let root = Object::new();

        Scene {
            background_color,
            root,
        }
    }

    pub fn add(&mut self, child: Rc<dyn Drawable>) -> &Self {
        self.root.add(child);
        self
    }

    pub fn remove(&mut self, child: Rc<dyn Drawable>) -> &Self {
        self.root.remove(child);
        self
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn get_background_color(&self) -> Color {
        self.background_color
    }
}

impl Transformable for Scene {
    fn set_transform(&mut self, transform: Transform) {
        self.root.set_transform(transform);
    }

    fn get_transform(&self) -> Transform {
        self.root.get_transform()
    }
}

impl Drawable for Scene {
    fn draw(
        &self,
        drawer: &mut dyn Drawer,
        transform: &Transform,
        camera: &Camera,
    ) -> Result<(), DrawError> {
        drawer.clear_scene(self.background_color, 1.0);
        self.root.draw(drawer, transform, camera)
    }
}
