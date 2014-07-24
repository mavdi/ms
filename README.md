# ms

A crate similar to the npm [ms] module 

[ms]: https://www.npmjs.org/package/ms

##Usage

```
extern crate ms;

fn main() {
  println!("main {}", ms::to_ms("5s"));
}
```

Valid inputs: ```s```, ```m```, ```h```, ```d``` and ```y```