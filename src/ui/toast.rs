// J1: App-wide toast notification system

#[derive(Debug, Clone, PartialEq)]
pub enum ToastKind {
    Info,
    Success,
    Error,
}

#[derive(Debug, Clone)]
pub struct Toast {
    pub id: u64,
    pub body: String,
    pub kind: ToastKind,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toast_fields() {
        let t = Toast { id: 1, body: "hello".into(), kind: ToastKind::Info };
        assert_eq!(t.id, 1);
        assert_eq!(t.body, "hello");
        assert_eq!(t.kind, ToastKind::Info);
    }
}
