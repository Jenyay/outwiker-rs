pub struct Page {
    subpath: Str,
    uid: Option<Str>,
    alias: Option<Str>,
    icon: Option<Str>,
    tags: Vec<Str>,
    order: i32,
    parent: Self,
    children: Vec<Self>,
}
