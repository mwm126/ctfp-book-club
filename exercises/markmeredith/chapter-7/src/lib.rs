fn arrow<A,B>(x: A)->B{
    panic!("FAIL");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
