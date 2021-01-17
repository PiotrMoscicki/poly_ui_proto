// deps
use nalgebra::Point2;
use nalgebra::Vector2;
use uuid::Uuid;
// crate
use crate::poly_ui::app::Color;
use crate::poly_ui::app::PainterTrait;
use crate::poly_ui::app::Rect;
use crate::poly_ui::components::Hierarchy;
use crate::poly_ui::components::Transform;
// super
use super::NewWidget;
use super::Ownerless;
use super::WidgetTrait;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Debug)]
pub struct Widget {
    id: Uuid,
    hierarchy: Hierarchy,
}

//************************************************************************************************
impl Widget {
    pub fn new_raw() -> Self {
        Self {
            id: Uuid::new_v4(),
            hierarchy: Hierarchy::default(),
        }
    }

    pub fn new() -> NewWidget<Self> {
        NewWidget::new(Self::new_raw())
    }
}

//************************************************************************************************
impl WidgetTrait for Widget {
    fn id(&self) -> &Uuid {
        &self.id
    }

    fn add(&mut self, child: Ownerless) {
        self.hierarchy.add(child);
    }

    fn remove(&mut self, child: &Uuid) -> Ownerless {
        self.hierarchy.remove(child)
    }

    fn update(&mut self, dt: f32) {
        println!("update widget");
        update_children(&self.hierarchy, dt);
    }

    fn paint(&self, painter: &mut dyn PainterTrait) {
        painter.set_draw_color(&Color {
            r: 255,
            g: 0,
            b: 255,
            a: 128,
        });
        painter.draw_rect(Rect {
            pos: Point2::<i32>::new(50, 50),
            size: Vector2::<u32>::new(50, 50),
        });

        println!("paint widget");
        paint_children(&self.hierarchy, painter);
    }
}
