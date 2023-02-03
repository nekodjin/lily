pub struct AsmFile {
    pub imports: Vec<Label>,
    pub exports: Vec<Label>,
}

pub struct Label {
    pub name: Vec<u8>,
}
