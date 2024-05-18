# Twilight (DePin Network with Brand Integrations)

Twilight introduces an innovative ecosystem to accelerate the adoption and generation of renewable energy. With the GenSensor technology, individuals can attach smart sensors to their green energy sources, like solar panels, to track and convert energy production into digital tokens. This mechanism not only encourages renewable energy generation but also allows for the creation and trade of green energy certificates in our marketplace, promoting transparency and economic activity within the green energy sector. Addressing real-world problems, our project directly contributes to reducing the global carbon footprint by providing economic incentives for clean energy usage, thereby making green energy more financially attractive and accessible. Ultimately, our initiative empowers individuals to actively participate in a sustainable energy future, bridging the gap between renewable energy producers and consumers, and fostering a community committed to environmental stewardship.

## Personal story and inspiration 

The inception of Twilight stems from a deeply rooted passion for sustainability and a desire to catalyze positive change in the world. As avid advocates for environmental conservation, the founding team members of Twilight were inspired by the pressing need to address the escalating climate crisis. Drawing from their diverse backgrounds in renewable energy, blockchain technology, and environmental activism, they embarked on a journey to create an innovative solution that not only incentivizes the adoption of clean energy but also fosters a sense of community-driven environmental stewardship. Motivated by a shared vision of a greener and more sustainable future, the team poured their hearts and minds into developing Twilight, driven by the belief that every individual has the power to make a meaningful difference in combating climate change. Through Twilight, they aspire to empower individuals and businesses alike to embrace renewable energy solutions, paving the way towards a more sustainable and equitable world for generations to come.

# Key Features

- *Innovative Sensor Technology:* Tracks and converts renewable energy production into digital tokens, incentivizing green energy generation.
  
- *Green Energy Token Marketplace:* Enables the buying and selling of Tokens that represent renewable energy generation, fostering a transparent and active market.
  
- *Tackles Carbon Footprint:* Directly addresses climate change by motivating individuals and businesses to reduce carbon emissions through renewable energy adoption.
  
- *Economic Incentives for Green Energy:* Offers financial rewards for clean energy production and consumption, making renewable energy investments more attractive.
  
- *Empowers Individuals:* Democratizes energy generation and consumption, creating a community of producers and consumers dedicated to promoting environmental sustainability.

# Implementation of Stellar Blockchain

Twilight leverages the Stellar blockchain to ensure secure, fast, and low-cost transactions for its digital tokens and green energy certificates. Stellar's robust infrastructure provides the foundation for our platform's financial operations, enhancing the efficiency and transparency of the marketplace.

## Why Stellar?

- *Low Transaction Fees:* Stellar offers minimal transaction costs, making it financially feasible to conduct numerous microtransactions for energy tokens.
  
- *Speed:* Transactions on the Stellar network are confirmed in a matter of seconds, ensuring swift trading and transfer of tokens.
  
- *Security:* Stellar's blockchain technology ensures the highest level of security for all transactions, protecting user data and assets.
  
- *Decentralization:* Stellar's decentralized nature aligns with Twilight's goal of democratizing energy generation and consumption, allowing anyone to participate in the green energy ecosystem.

## How It Works

1. *Tokenization of Energy Production:* GenSensor technology tracks renewable energy production and converts it into digital tokens on the Stellar blockchain.
   
2. *Trading on the Marketplace:* Users can buy and sell these tokens on Twilight's Green Energy Token Marketplace, powered by Stellar's efficient transaction system.
   
3. *Transparent and Immutable Records:* Every transaction and certificate is recorded on the Stellar blockchain, ensuring transparency and preventing fraud.
   
4. *Economic Rewards:* Users earn tokens for their energy production, which can be traded or redeemed, providing financial incentives for green energy adoption.

![Twilight workflow](./public/twilight.jpg)

## Smart Contract Functions

### init(env: Env)
- Initializes the smart contract by setting up initial state variables and mappings.

### add_promotion_secret(env: Env, promotion_secret: Bytes)
- Allows users to add promotion secrets for promotional activities.

### register_as_brand(env: Env)
- Enables users to register as a brand to participate in promotional activities.

### add_eligible_promotions(env: Env, seller: Address, promotion_secret: Bytes)
- Allows brands to add eligible promotions for promotional activities.

### add_gen_station(env: Env, code: Bytes)
- Implements functionality to add generation stations and link them to addresses.

### return_orders_array_length(env: Env) -> u64
- Retrieves the length of the order array.

### update_hm_token_balance(env: Env, code: Bytes, new_value: u64)
- Updates the balance of HM tokens for a given generation station.

### return_hm_balance(env: Env) -> u64
- Retrieves the balance of HM tokens for the caller.

### create_buy_order(env: Env, order_id: u64)
- Creates a buy order for green energy tokens.

### consume_token(env: Env, order_id: u64)
- Consumes tokens for a given order.

### list_order(env: Env, sell_price: u64, no_of_hm_tokens: u64, option_price: u64, duration: u64)
- Lists an order for selling green energy tokens.

### take_on_option(env: Env, order_id: u64)
- Allows users to take on an option for a given order.

### redeem_tokens(env: Env, value: u64, user: Address)
- Redeems tokens for a given user.

### end_option(env: Env, order_id: u64)
- Ends an option for a given order.

### update_time(env: Env)
- Updates the current timestamp in the contract storage.

### check_expired_options(env: Env)
- Checks for and handles expired options in the contract.

# Deployed Contract (Stellar Testnet)

# Arduino Setup
1. Download Arduino Modules.
2. Add INA219 Sensor for collecting data.
3. Serialize the connection to get data.
4. Tokens will be generated after collecting data.

## Getting Started

To get started with Twilight, follow these steps:

1. Clone the repository: git clone https://github.com/your-username/Twilight.git
2. Install dependencies: npm install
3. Configure your GenSensor devices and connect them to the Twilight platform.
4. Start tracking and converting renewable energy production into digital tokens.
5. Explore the Green Energy Token Marketplace and participate in trading activities.