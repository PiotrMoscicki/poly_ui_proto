// std
use std::{cell::RefCell, rc::Rc};
// super
use super::WidgetTrait;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
pub struct Ownerless {
    widget: Rc<RefCell<dyn WidgetTrait>>,
}

//************************************************************************************************
impl Ownerless {
    pub fn to_owned(self) -> Owned {
        return Owned {
            widget: self.widget,
        };
    }

    pub fn get(&self) -> &Rc<RefCell<dyn WidgetTrait>> {
        return &self.widget;
    }
}

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Debug)]
pub struct Owned {
    widget: Rc<RefCell<dyn WidgetTrait>>,
}

//************************************************************************************************
impl Owned {
    pub fn to_ownerless(self) -> Ownerless {
        return Ownerless {
            widget: self.widget,
        };
    }

    pub fn get(&self) -> &Rc<RefCell<dyn WidgetTrait>> {
        return &self.widget;
    }
}

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Debug)]
pub struct NewWidget<T: WidgetTrait + 'static> {
    widget: Rc<RefCell<T>>,
}

//************************************************************************************************
impl<T: WidgetTrait + 'static> NewWidget<T> {
    pub fn new(widget: T) -> Self {
        return Self {
            widget: Rc::new(RefCell::new(widget)),
        };
    }

    pub fn to_ownerless(self) -> Ownerless {
        return Ownerless {
            widget: self.widget,
        };
    }

    pub fn get(&self) -> &Rc<RefCell<T>> {
        return &self.widget;
    }
}