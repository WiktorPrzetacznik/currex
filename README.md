# Currex
Simple application that converts amounts between different currencies using  
real-time exchange rate data fetched from ExchangeRate-API. 

### Features
- Currency conversion using real-time data
- Listing all available currencies with their exchange rate for chosen currency
- Caching system 

## Build
### Prerequisites
- Rust 1.76 along with recent toolchain (March 13th 2024)
- ExchangeRate-API api key

### Procedure
In order to build the application, follow steps below:
 1. Clone/download repository
 2. Go into the repository directory
 3. (if you already have api key, go to step 5) Register on https://www.exchangerate-api.com/
 4. Activate your api-key
 5. Replace the placeholder api-key in currex\src\converter\external_api.rs with your own key
 6. Using command line (or IDE) trigger project building from repo's directory
  `cargo build --release` or   `cargo build` if you want to skip optimizations

## Run
In order to run the application, execute .exe file you received in building process passing following arguments:

`-b, --base-code <BASE_CODE>` | currency code you want to convert from

`-t, --target-code <TARGET_CODE>` | currency code you want to convert to

`-a, --amount <AMOUNT>` | amount you want to convert

`-l, --list` | optional, listing all available currencies with rates for given base code

### Notes
While running, currex creates cache file in its directory. 
