use gen::structs::Base;
use gen::structs::Record;
use table::Column;
use table::Foreign;
use table::IsTable;
use table::Table;


impl IsTable for Base{

    fn table()->Table{
    
        Table{
            schema:"system".to_string(),
            name:"base".to_string(),
            parent_table:None,
            sub_table:Some(vec!["record".to_string(),"product_availability".to_string(),"product_category".to_string(),"product_photo".to_string(),"product_review".to_string(),]),
            comment:Some("Base table contains the creation and modification status of a record".to_string()),
            columns:
            vec![
                Column{
                    name:"organization_id".to_string(),
                    data_type:"Uuid".to_string(),
                    db_data_type:"uuid".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"client_id".to_string(),
                    data_type:"Uuid".to_string(),
                    db_data_type:"uuid".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"created".to_string(),
                    data_type:"DateTime<UTC>".to_string(),
                    db_data_type:"timestamp with time zone".to_string(),
                    is_primary:false, is_unique:false, not_null:true, is_inherited:false, 
                    default:Some("now()".to_string()),
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"createdby".to_string(),
                    data_type:"Uuid".to_string(),
                    db_data_type:"uuid".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"updated".to_string(),
                    data_type:"DateTime<UTC>".to_string(),
                    db_data_type:"timestamp with time zone".to_string(),
                    is_primary:false, is_unique:false, not_null:true, is_inherited:false, 
                    default:Some("now()".to_string()),
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"updatedby".to_string(),
                    data_type:"Uuid".to_string(),
                    db_data_type:"uuid".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"priority".to_string(),
                    data_type:"f64".to_string(),
                    db_data_type:"numeric".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false, 
                    default:None,
                    comment:Some("priority of saving data and eviction".to_string()),
                    foreign:None,
                },
            ],
        }
    }
}

impl IsTable for Record{

    fn table()->Table{
    
        Table{
            schema:"system".to_string(),
            name:"record".to_string(),
            parent_table:Some("base".to_string()),
            sub_table:Some(vec!["address".to_string(),"api_key".to_string(),"cart".to_string(),"cart_line".to_string(),"category".to_string(),"client".to_string(),"invoice".to_string(),"order_line".to_string(),"orders".to_string(),"organization".to_string(),"photo".to_string(),"photo_sizes".to_string(),"product".to_string(),"review".to_string(),"settings".to_string(),"user_info".to_string(),"user_location".to_string(),"user_review".to_string(),"users".to_string(),"wishlist".to_string(),"wishlist_line".to_string(),"country".to_string(),"currency".to_string(),"exchange_rate".to_string(),]),
            comment:Some("All User table should inherit from this one".to_string()),
            columns:
            vec![
                Column{
                    name:"organization_id".to_string(),
                    data_type:"Uuid".to_string(),
                    db_data_type:"uuid".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:true, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"client_id".to_string(),
                    data_type:"Uuid".to_string(),
                    db_data_type:"uuid".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:true, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"created".to_string(),
                    data_type:"DateTime<UTC>".to_string(),
                    db_data_type:"timestamp with time zone".to_string(),
                    is_primary:false, is_unique:false, not_null:true, is_inherited:true, 
                    default:Some("now()".to_string()),
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"createdby".to_string(),
                    data_type:"Uuid".to_string(),
                    db_data_type:"uuid".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:true, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"updated".to_string(),
                    data_type:"DateTime<UTC>".to_string(),
                    db_data_type:"timestamp with time zone".to_string(),
                    is_primary:false, is_unique:false, not_null:true, is_inherited:true, 
                    default:Some("now()".to_string()),
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"updatedby".to_string(),
                    data_type:"Uuid".to_string(),
                    db_data_type:"uuid".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:true, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"priority".to_string(),
                    data_type:"f64".to_string(),
                    db_data_type:"numeric".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:true, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"name".to_string(),
                    data_type:"String".to_string(),
                    db_data_type:"character varying".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"description".to_string(),
                    data_type:"String".to_string(),
                    db_data_type:"character varying".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"help".to_string(),
                    data_type:"String".to_string(),
                    db_data_type:"text".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"active".to_string(),
                    data_type:"bool".to_string(),
                    db_data_type:"boolean".to_string(),
                    is_primary:false, is_unique:false, not_null:true, is_inherited:false, 
                    default:Some("true".to_string()),
                    comment:Some("@Active".to_string()),
                    foreign:None,
                },
            ],
        }
    }
}

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