use super::Interface;

enum BuilderState {
    Boot,
    Lazy
}

pub struct InterfaceBuilder<S> {
    state: S
}


impl InterfaceBuilder {
    fn new() -> InterfaceBuilder<S> {
        InterfaceBuilder {}
    }
}
