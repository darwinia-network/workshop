use support::{decl_storage,decl_module,decl_event,StorageValue,StorageMap,dispatch::Result,ensure,traits::Currency};

use parity_codec::{Encode,Decode};
use runtime_primitives::traits::{Zero,As,Hash};
use system::ensure_signed;
use rstd::cmp;


//添加event第二步：在module中暴露出这个新的Event类型
pub trait Trait: balances::Trait {
	type Event : From<Event<Self>>+Into<<Self as system::Trait>::Event>;
}

//自定义Kitty的数据结构
#[derive(Encode,Decode,Default,Clone,PartialEq)]
#[cfg_attr(feature="std",derive(Debug))]
pub struct Kitty<Hash,Balance>{
	id : Hash,
	dna : Hash,
	price : Balance,
	gen : u64,
}

//添加event第一步
decl_event!{
	pub enum Event<T>
	where 
		<T as system::Trait>::AccountId,
		<T as system::Trait>::Hash,
		<T as balances::Trait>::Balance
	{
		Created(AccountId,Hash),
		PriceSet(AccountId,Hash,Balance),
		Transferred(Hash,AccountId,AccountId),
		Bought(AccountId,AccountId,Hash,Balance),
		Breed(AccountId,Hash,Hash,Hash),
	}
}

decl_storage!{
	trait Store for Module<T:Trait> as KittyStorage{
	
		
		//dev
		
		AllKittiesArray get(all_kitties): map u64 => T::Hash;  //存储所有的Kitty
		AllKittiesCount get(all_kitties_count): u64;  //链上Kitty数目
		AllKittiesIndex : map T::Hash => u64;  //Kitty对应的索引


		KittiesOfOwnedByIndex get(kitties_of_owned_by_index) : map (T::AccountId,u64) => T::Hash;
		OwnedKittiesCount get(owned_kitties_count) : map T::AccountId => u64;
		OwnedKittiesIndex get(owned_kitties_index) :map T::Hash => u64;
		
		Kitties get(kitty): map T::Hash => Kitty<T::Hash,T::Balance>;
		KittyOwner get(owner_of): map T::Hash => Option<T::AccountId>;
		
		Nonce : u64;
		
	}
}


/*
	Module结构体是每个Substrate运行时模块的主干
*/
decl_module!{
	pub struct Module<T: Trait> for enum Call where origin : T::Origin{
	
		
		//dev
		
		//添加event第三步：添加一个默认实现deposit_event的函数来存储event中的函数,最后一步是去更新lib.rs
		fn deposit_event<T>() = default;
		
		fn create_kitty(origin) -> Result{
			let sender = ensure_signed(origin)?;
			
			
			
			//借助随机数种子、Nonce、以及地址来创建一个独一无二的Hash赋给id和dna
			let random_seed = <system::Module<T>>::random_seed();
			let nonce = <Nonce<T>>::get();
			
			let random_hash = (random_seed,&sender,nonce).using_encoded(<T as system::Trait>::Hashing::hash);

			//监测冲突
			//ensure!(!<Kitties<T>>::exists(random_hash),"Kiity already exists");
			
			
			let kitty = Kitty{
				id : random_hash,
				dna : random_hash,
				price : <T::Balance as As<u64>>::sa(0),
				gen : 0,
			};
			
			
			
			
			<Nonce<T>>::mutate(|n| *n += 1);//更新Nonce的值
			
			
			Self::mint(sender,random_hash,kitty)?;
		
			Ok(())
		}
		
		fn breed_kitty(origin,parent_1 : T::Hash,parent_2 : T::Hash) -> Result{
			let sender = ensure_signed(origin)?;
			ensure!(parent_1 != parent_2,"Cannot choose the same Kitty!");
			
			//判断双亲是否都属于调用者
			let owner_parent_1 = <KittyOwner<T>>::get(parent_1).ok_or("The parent_1 kitty doesnot exist")?;
			let pwner_parent_2 = <KittyOwner<T>>::get(parent_2).ok_or("The parent_2 kitty doesnot exist")?;
			
			let nonce = <Nonce<T>>::get();
			let random_hash = (<system::Module<T>>::random_seed(),&sender,nonce).using_encoded(<T as system::Trait>::Hashing::hash);
			
			let kitty_1 = Self::kitty(parent_1);
			let kitty_2 = Self::kitty(parent_2);
			
			let mut final_dna = kitty_1.dna;
			
			
			//生成final_dna的简单思想
            for (i, (dna_2_element, r)) in kitty_2.dna.as_ref().iter().zip(random_hash.as_ref().iter()).enumerate() {
                if r % 2 == 0 {
                    final_dna.as_mut()[i] = *dna_2_element;
                }
            }
			
			let final_gen = cmp::max(kitty_1.gen,kitty_2.gen)+1;
			
			let kitty = Kitty{
				id : random_hash,
				dna : final_dna,
				price : <T::Balance as As<u64>>::sa(0),
				gen : final_gen,
			};
			
			
			Self::mint(sender.clone(),random_hash,kitty)?;

			//事件的通知
			Self::deposit_event(RawEvent::Breed(sender,parent_1,parent_2,final_dna));
			Ok(())

			
		}
		
		fn set_price(origin,kitty_id:T::Hash,new_price: T::Balance) -> Result{
			let sender = ensure_signed(origin)?;
			
			ensure!(<Kitties<T>>::exists(kitty_id),"This Kitty doesnot exist");
			
			let owner = Self::owner_of(kitty_id).ok_or("Cannot find owner for this kitty")?;
			
			ensure!(owner==sender,"This kitty doesnot belong to you");
			
			let mut kitty = Self::kitty(kitty_id);
			
			//设置价格
			kitty.price = new_price;
			
			<Kitties<T>>::insert(kitty_id,kitty);
			
			Self::deposit_event(RawEvent::PriceSet(sender,kitty_id,new_price));
			
			Ok(())
		}
		
		fn transfer(origin,to : T::AccountId,kitty_id : T::Hash) -> Result{
			let sender = ensure_signed(origin)?;
			
			//进行状态修改
			Self::transfer_from(sender,to,kitty_id)?;
			
			Ok(())
			
		}
		
		fn buy_kitty(origin,kitty_id:T::Hash,max_price: T::Balance) -> Result{
			let sender = ensure_signed(origin)?;
			
			ensure!(<Kitties<T>>::exists(kitty_id),"The kitty doesnot exist");
			
			let owner = <KittyOwner<T>>::get(kitty_id).ok_or("The Kitty doesnot has a owner")?;
			ensure!(owner!=sender,"You cannot buy your own kitty");
			
			let mut kiity = <Kitties<T>>::get(kitty_id);
			let kiity_price = kiity.price;
			ensure!(!kiity_price.is_zero(),"The Kitty is not onsell");
			
			ensure!(kiity_price <= max_price,"You cannot afford this kitty with the money you give");
			
			<balances::Module<T> as Currency<_>>::transfer(&sender,&owner,kiity_price)?;
			

            Self::transfer_from(owner.clone(), sender.clone(), kitty_id)
                .expect("`owner` is shown to own the kitty; \
                `owner` must have greater than 0 kitties, so transfer cannot cause underflow; \
                `all_kitty_count` shares the same type as `owned_kitty_count` \
                and minting ensure there won't ever be more than `max()` kitties, \
                which means transfer cannot cause an overflow; \
                qed");
				
			kiity.price = <T::Balance as As<u64>>::sa(0);
			
			<Kitties<T>>::insert(kitty_id,kiity);
			
			Self::deposit_event(RawEvent::Bought(owner,sender,kitty_id,kiity_price));
			
			Ok(())
			
		}
	}
}

impl<T:Trait> Module<T>{
	//该函数仅对Kitty的所有权进行声明，不包含生成Kitty等操作
	fn mint(sender: T::AccountId,kitty_id:T::Hash,new_kitty:Kitty<T::Hash,T::Balance>) -> Result{
			//verify
			let owned_kitties_count = <OwnedKittiesCount<T>>::get(&sender);//&操作符是用来创建一个引用
			let new_owned_kitties_count = owned_kitties_count.checked_add(1).ok_or("Overflow adding a new Kitty")?;
			
			//verify
			let all_kitties_count = Self::all_kitties_count();
			let new_all_kitties_count = all_kitties_count.checked_add(1).ok_or("Overflow adding a new kitty")?;
			
			ensure!(!(<Kitties<T>>::exists(kitty_id)),"The new id already exists");//碰撞监测
			
			<Kitties<T>>::insert(kitty_id,new_kitty);
			<KittyOwner<T>>::insert(kitty_id,&sender);
			
						//更新存储
			<AllKittiesArray<T>>::insert(all_kitties_count,kitty_id);
			<AllKittiesCount<T>>::put(new_all_kitties_count);
			<AllKittiesIndex<T>>::insert(kitty_id,all_kitties_count);
			
			<KittiesOfOwnedByIndex<T>>::insert((sender.clone(),owned_kitties_count),kitty_id);
			<OwnedKittiesCount<T>>::insert(&sender,new_owned_kitties_count);
			<OwnedKittiesIndex<T>>::insert(kitty_id,owned_kitties_count);
			
			Self::deposit_event(RawEvent::Created(sender,kitty_id));
			
			Ok(())
	
	}
	
	fn transfer_from(from : T::AccountId,to : T::AccountId,kitty_id : T::Hash) -> Result{
	
		let owner = Self::owner_of(kitty_id).ok_or("Cannot find owner for this kitty")?;
		ensure!(from==owner,"This kitty doesnot belong to you ");
		
		
		
		let owned_kitties_count_from = Self::owned_kitties_count(&from);
		let owned_kitties_count_to = Self::owned_kitties_count(&to);
		
		let new_owned_kitties_count_from = owned_kitties_count_from.checked_sub(1).ok_or("Underflow when sub a kiity")?;
		let new_owned_kitties_count_to  = owned_kitties_count_to.checked_add(1).ok_or("Overflow when add a kitty")?;
		
		let kitty_index = <OwnedKittiesIndex<T>>::get(kitty_id);
		
		//swap & pop
		if kitty_index != new_owned_kitties_count_from{//不是最后一个，则要进行swap
			let last_kitty_id = <KittiesOfOwnedByIndex<T>>::get((from.clone(),new_owned_kitties_count_from));
			<KittiesOfOwnedByIndex<T>>::insert((from.clone(),kitty_index),last_kitty_id);//
			<OwnedKittiesIndex<T>>::insert(last_kitty_id,kitty_index);//
		}
		
		//pop
		<KittiesOfOwnedByIndex<T>>::remove((from.clone(),new_owned_kitties_count_from));//
		
		<KittiesOfOwnedByIndex<T>>::insert((to.clone(),owned_kitties_count_to),kitty_id);//
		<OwnedKittiesIndex<T>>::insert(kitty_id,owned_kitties_count_to);//
		
		<KittyOwner<T>>::insert(kitty_id,&to);//
		
		<OwnedKittiesCount<T>>::insert(&from,new_owned_kitties_count_from);//
		<OwnedKittiesCount<T>>::insert(&to,new_owned_kitties_count_to);//
		
		
		Self::deposit_event(RawEvent::Transferred(kitty_id,from,to));
		
		
		//可以加判断：发送和接收账户是否是同一账户
		
		Ok(())
	}
}
