fn log_any<T: Any + Debug>(value: &T) {
    let value_any = value as &dyn Any;
  
    // Try to convert our value its concrete type
    match value_any.downcast_ref::<String>() {
      Some(as_string) => {
        println!("String ({}): {}", as_string.len(), as_string);
      }
      None => {
        println!("{value:?} is not a String");
      }
    }
  
    match value_any.downcast_ref::<i32>() {
      Some(as_i32) => {
        println!("i32: {}", as_i32);
      }
      None => {
        println!("{value:?} is not an i32");
      }
    }
  }
  
  pub mod box_deref {
    use super::*;
  
    fn example() {
      let boxed: Box<dyn Any> = Box::new(3_i32);
      // You're more likely to want this:
      let actual_id = (&*boxed).type_id();
      // ... than this:
      let boxed_id = boxed.type_id();
  
      assert_eq!(actual_id, TypeId::of::<i32>());
      assert_eq!(boxed_id, TypeId::of::<Box<dyn Any>>());
    }
  }
  