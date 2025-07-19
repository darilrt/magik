use crate::Renderable;

pub type Children = Vec<Box<dyn Renderable>>;

#[macro_export]
macro_rules! children {
    [$($child:expr),*] => {
        vec![$(Box::new($child)),*]
    };
}
