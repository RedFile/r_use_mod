pub fn is_even_fn(x:i32)->Result<bool,&'static str>{
    match x%2==0 {
        true=>Ok(true),
        false=>Err("not even")
    }
}