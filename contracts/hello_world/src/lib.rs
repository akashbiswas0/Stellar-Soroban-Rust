#![no_std]
extern crate soroban_sdk;
use soroban_sdk::{contractimpl, Address, Bytes, BytesN, Env, IntoVal, Vec};

pub struct Contract;

#[derive(Clone)]
pub struct Order {
    pub seller: Address,
    pub owner: Address,
    pub order_id: u64,
    pub sell_price: u64,
    pub is_buy: bool,
    pub is_sale: bool,
    pub is_option: bool,
    pub option_fee: u64,
    pub option_duration: u64,
    pub fulfilled: bool,
    pub no_of_hm_tokens: u64,
    pub created_at: u64,
}

#[contractimpl]
impl Contract {
    pub fn init(env: Env) {
        let verified_sensors = Vec::from_array(
            &env,
            &[
                BytesN::from_array(
                    &env,
                    &[
                        0xb8, 0x71, 0x5f, 0xb9, 0x8f, 0xeb, 0x70, 0xc3, 0xf3, 0xf1, 0xb0, 0x11,
                        0x74, 0x57, 0x7b, 0xbd, 0xbf, 0x7f, 0xe3, 0x28, 0x92, 0x84, 0x6a, 0xaa,
                        0xd9, 0x67, 0x76, 0xfb, 0x58, 0x27, 0x02, 0x16,
                    ],
                ),
                BytesN::from_array(
                    &env,
                    &[
                        0x21, 0x05, 0x74, 0x2f, 0x5a, 0xdb, 0x22, 0x9d, 0xd4, 0xbe, 0x38, 0x98,
                        0x31, 0x4f, 0xdd, 0x0f, 0x0d, 0xd3, 0x5e, 0xfb, 0xf0, 0xa5, 0x72, 0x4c,
                        0xc7, 0xd5, 0xa1, 0x7e, 0xee, 0x9a, 0xfd, 0x1f,
                    ],
                ),
            ],
        );
        env.storage().set("verified_sensors", verified_sensors);

        env.storage().set("latest_timestamp", 0u64);
        env.storage().set("creds_market_price", 10u64);
    }

    pub fn add_promotion_secret(env: Env, promotion_secret: Bytes) {
        let address = env.invoker();
        env.storage().set(
            "address_to_promotion_secret",
            address.clone(),
            promotion_secret,
        );
    }

    pub fn register_as_brand(env: Env) {
        let address = env.invoker();
        env.storage().set("is_brand", address.clone(), true);
    }

    pub fn add_eligible_promotions(env: Env, seller: Address, promotion_secret: Bytes) {
        let mut eligible_promotions =
            Contract::get_all_eligible_promotions(env.clone(), seller.clone());
        eligible_promotions.push_back(promotion_secret);
        env.storage().set(
            "address_to_eligible_promotions",
            seller,
            eligible_promotions,
        );
    }

    pub fn get_all_eligible_promotions(env: Env, seller: Address) -> Vec<Bytes> {
        env.storage()
            .get("address_to_eligible_promotions", seller)
            .unwrap_or(Vec::new(&env))
    }

    pub fn add_gen_station(env: Env, code: Bytes) {
        let address = env.invoker();
        env.storage().set("gen_station_to_address", code, address);
    }

    pub fn return_orders_array_length(env: Env) -> u64 {
        let order_array: Vec<Order> = env.storage().get("order_array").unwrap_or(Vec::new(&env));
        order_array.len()
    }

    pub fn update_hm_token_balance(env: Env, code: Bytes, new_value: u64) {
        Contract::update_time(env.clone());
        Contract::check_expired_options(env.clone());

        let gen_station_address: Address =
            env.storage().get("gen_station_to_address", code).unwrap();
        env.storage()
            .set("balances", gen_station_address, new_value);
    }

    pub fn return_hm_balance(env: Env) -> u64 {
        let address = env.invoker();
        env.storage().get("balances", address).unwrap_or(0)
    }

    pub fn create_buy_order(env: Env, order_id: u64) {
        Contract::update_time(env.clone());
        Contract::check_expired_options(env.clone());

        let mut order_array: Vec<Order> =
            env.storage().get("order_array").unwrap_or(Vec::new(&env));
        let order = order_array.get(order_id as usize).unwrap().clone();

        let payment = env.transferred_balance();
        assert!(payment >= order.sell_price, "Insufficient value sent");
        assert!(!order.fulfilled, "Order already fulfilled");

        let mut new_order = order.clone();
        new_order.owner = env.invoker();
        new_order.fulfilled = true;
        new_order.option_duration = 0;
        order_array.set(order_id as usize, new_order.clone());

        let mut buyer_balance = Contract::return_hm_balance(env.clone());
        buyer_balance += order.no_of_hm_tokens;
        env.storage().set("balances", env.invoker(), buyer_balance);

        let seller_address = Address::from_bytes(&order.seller.into_val(&env)).unwrap();
        env.transfer(seller_address, payment);

        let creds_market_price = order.sell_price / order.no_of_hm_tokens;
        env.storage().set("creds_market_price", creds_market_price);
    }

    pub fn consume_token(env: Env, order_id: u64) {
        Contract::update_time(env.clone());
        Contract::check_expired_options(env.clone());

        let mut order_array: Vec<Order> =
            env.storage().get("order_array").unwrap_or(Vec::new(&env));
        let order = order_array.get(order_id as usize).unwrap().clone();

        let payment = env.transferred_balance();
        assert!(payment >= order.sell_price, "Insufficient value sent");
        assert!(!order.fulfilled, "Order already fulfilled");
        assert!(
            Contract::is_brand(env.clone(), env.invoker()),
            "Not a brand"
        );

        let mut new_order = order.clone();
        new_order.owner = env.invoker();
        new_order.fulfilled = true;
        new_order.option_duration = 0;
        order_array.set(order_id as usize, new_order.clone());

        let mut receiver_balance = env
            .storage()
            .get("rec_balances", env.invoker())
            .unwrap_or(0);
        receiver_balance += order.no_of_hm_tokens;
        env.storage()
            .set("rec_balances", env.invoker(), receiver_balance);

        let promotion_secret = env
            .storage()
            .get("address_to_promotion_secret", env.invoker())
            .unwrap();
        Contract::add_eligible_promotions(env.clone(), order.seller.clone(), promotion_secret);

        let seller_address = Address::from_bytes(&order.seller.into_val(&env)).unwrap();
        env.transfer(seller_address, payment);

        let creds_market_price = order.sell_price / order.no_of_hm_tokens;
        env.storage().set("creds_market_price", creds_market_price);
    }

    pub fn list_order(
        env: Env,
        sell_price: u64,
        no_of_hm_tokens: u64,
        option_price: u64,
        duration: u64,
    ) {
        Contract::update_time(env.clone());
        Contract::check_expired_options(env.clone());

        let seller_balance = Contract::return_hm_balance(env.clone());
        assert!(seller_balance >= no_of_hm_tokens, "Insufficient HMTokens");

        let mut order_array: Vec<Order> =
            env.storage().get("order_array").unwrap_or(Vec::new(&env));
        let order_id = order_array.len();

        let order = Order {
            seller: env.invoker(),
            owner: env.invoker(),
            order_id,
            sell_price,
            is_buy: false,
            is_sale: true,
            is_option: true,
            option_fee: option_price,
            option_duration: duration,
            fulfilled: false,
            no_of_hm_tokens,
            created_at: env.ledger().timestamp(),
        };
        order_array.push_back(order);
        env.storage().set("order_array", order_array);

        let mut seller_balance = Contract::return_hm_balance(env.clone());
        seller_balance -= no_of_hm_tokens;
        env.storage().set("balances", env.invoker(), seller_balance);
    }

    pub fn take_on_option(env: Env, order_id: u64) {
        Contract::update_time(env.clone());
        Contract::check_expired_options(env.clone());

        let mut order_array: Vec<Order> =
            env.storage().get("order_array").unwrap_or(Vec::new(&env));
        let order = order_array.get(order_id as usize).unwrap().clone();

        let payment = env.transferred_balance();
        assert!(payment >= order.option_fee, "Insufficient value sent");
        assert!(!order.fulfilled, "Option already fulfilled");

        let mut new_order = order.clone();
        new_order.owner = env.invoker();
        new_order.fulfilled = true;
        new_order.created_at = env.ledger().timestamp();
        order_array.set(order_id as usize, new_order.clone());

        let mut buyer_balance = Contract::return_hm_balance(env.clone());
        buyer_balance += order.no_of_hm_tokens;
        env.storage().set("balances", env.invoker(), buyer_balance);

        let seller_address = Address::from_bytes(&order.seller.into_val(&env)).unwrap();
        env.transfer(seller_address, payment);
    }

    pub fn redeem_tokens(env: Env, value: u64, user: Address) {
        Contract::check_expired_options(env.clone());

        let mut user_balance = env.storage().get("balances", user.clone()).unwrap_or(0);
        user_balance -= value;
        env.storage().set("balances", user, user_balance);
    }

    pub fn end_option(env: Env, order_id: u64) {
        let mut order_array: Vec<Order> =
            env.storage().get("order_array").unwrap_or(Vec::new(&env));
        let order = order_array.get(order_id as usize).unwrap().clone();

        let mut owner_balance = env.storage().get("balances", order.owner).unwrap_or(0);
        owner_balance -= order.no_of_hm_tokens;
        env.storage().set("balances", order.owner, owner_balance);

        let mut new_order = order.clone();
        new_order.fulfilled = false;
        new_order.owner = order.seller;
        order_array.set(order_id as usize, new_order);
    }

    pub fn update_time(env: Env) {
        env.storage()
            .set("latest_timestamp", env.ledger().timestamp());
    }

    pub fn check_expired_options(env: Env) {
        Contract::update_time(env.clone());

        let latest_timestamp = env.storage().get("latest_timestamp").unwrap();
        let mut order_array: Vec<Order> =
            env.storage().get("order_array").unwrap_or(Vec::new(&env));

        for (index, order) in order_array.iter().enumerate() {
            if order.option_duration > 0
                && order.created_at + order.option_duration < latest_timestamp
            {
                Contract::end_option(env.clone(), index as u64);
            }
        }
    }

    pub fn is_verified(env: Env) -> bool {
        env.storage().get("is_verified").unwrap_or(false)
    }

    pub fn check_verified_sensors(env: Env, code: Bytes) -> bool {
        let verified_sensors: Vec<BytesN<32>> = env.storage().get("verified_sensors").unwrap();

        for sensor in verified_sensors.iter() {
            if sensor == &BytesN::from_array(&env, &code.into_val(&env)) {
                env.storage().set("is_verified", true);
                return true;
            }
        }

        false
    }

    pub fn is_brand(env: Env, address: Address) -> bool {
        env.storage().get("is_brand", address).unwrap_or(false)
    }
}
