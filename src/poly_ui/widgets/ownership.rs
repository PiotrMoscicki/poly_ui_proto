// std
use std::{
    cell::RefCell,
    rc::Rc,
};
// super
use super::WidgetTrait;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Debug)]
pub struct Ownerless<T: WidgetTrait + ?Sized> {
    widget: Rc<RefCell<T>>,
}

//************************************************************************************************
impl<T: WidgetTrait + ?Sized> Ownerless<T> {
    pub fn new(widget: Rc<RefCell<T>>) -> Self {
        return Self {
            widget:widget,
        };
    }

    pub fn to_owned(self) -> Owned<T> {
        return Owned{
            widget: self.widget, 
        };
    }
}

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Debug)]
pub struct Owned<T: WidgetTrait + ?Sized> {
    widget: Rc<RefCell<T>>,
}

//************************************************************************************************
impl<T: WidgetTrait> Owned<T> {
    pub fn to_ownerless(self) -> Ownerless<T> {
        return Ownerless{
            widget: self.widget, 
        };
    }

    pub fn get_widget_rc(&self) -> &Rc<RefCell<T>> {
        return &self.widget;
    }
}