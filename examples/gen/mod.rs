pub mod bazaar;
pub mod payment;
pub mod system;
use rustorm::table::Table;
use rustorm::table::IsTable;

use gen::bazaar::Address;
use gen::bazaar::ApiKey;
use gen::system::Base;
use gen::bazaar::Cart;
use gen::bazaar::CartLine;
use gen::bazaar::Category;
use gen::bazaar::Client;
use gen::payment::Country;
use gen::payment::Currency;
use gen::payment::ExchangeRate;
use gen::bazaar::Invoice;
use gen::bazaar::OrderLine;
use gen::bazaar::Orders;
use gen::bazaar::Organization;
use gen::bazaar::Photo;
use gen::bazaar::PhotoSizes;
use gen::bazaar::Product;
use gen::bazaar::ProductAvailability;
use gen::bazaar::ProductCategory;
use gen::bazaar::ProductPhoto;
use gen::bazaar::ProductReview;
use gen::system::Record;
use gen::bazaar::Review;
use gen::bazaar::Settings;
use gen::bazaar::UserInfo;
use gen::bazaar::UserLocation;
use gen::bazaar::UserReview;
use gen::bazaar::Users;
use gen::bazaar::Wishlist;
use gen::bazaar::WishlistLine;


pub fn get_all_tables()->Vec<Table>{
    vec![
        Address::table(),
        ApiKey::table(),
        Base::table(),
        Cart::table(),
        CartLine::table(),
        Category::table(),
        Client::table(),
        Country::table(),
        Currency::table(),
        ExchangeRate::table(),
        Invoice::table(),
        OrderLine::table(),
        Orders::table(),
        Organization::table(),
        Photo::table(),
        PhotoSizes::table(),
        Product::table(),
        ProductAvailability::table(),
        ProductCategory::table(),
        ProductPhoto::table(),
        ProductReview::table(),
        Record::table(),
        Review::table(),
        Settings::table(),
        UserInfo::table(),
        UserLocation::table(),
        UserReview::table(),
        Users::table(),
        Wishlist::table(),
        WishlistLine::table(),
    ]
}