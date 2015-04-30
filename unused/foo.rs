pub trait Foo {
    fn foo(&self);
}

pub trait FooBar : Foo {
    fn foobar(&self);
    fn foo(&self, str:String);
}

pub struct Baz;

impl Foo for Baz {
    fn foo(&self) { println!("foo"); }
}

impl FooBar for Baz {
    fn foobar(&self) { println!("foobar"); }
    fn foo(&self, str:String) { println!("foo in foobar"); }
}