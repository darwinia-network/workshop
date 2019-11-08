use support::{decl_storage,decl_module,dispatch::Result,StorageValue};

pub trait Trait : system::Trait {
	// add code here
}

decl_module!{
	pub struct Module<T:Trait> for enum Call 
		where origin : T::Origin{
			fn setValue(origin,value:i32) -> Result{
				<Value<T>>::put(value);
				Ok(())
			}
		}
}

decl_storage!{
	trait Store for Module<T:Trait> as TestModelue{
		Value get(getValue) : i32;
	}
}