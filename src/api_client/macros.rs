#[macro_export]
macro_rules! is_connected {
    ($self:ident) => {{
        match $self.stream {
            Some(_) => (),
            None => panic!("Client needs to be connected!"),
        };
    }};
}
