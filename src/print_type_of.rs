pub fn print_type_of<T>(_: &T) {
    println!("Type of {}", std::any::type_name::<T>())
}
