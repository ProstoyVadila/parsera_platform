mod crawlers;
mod users;
mod common;

use rocket::Route;

use crawlers::{
    get_crawler,
    add_crawler,
    delete_crawler,
    update_crawler,
    get_crawlers,
    add_site,
};
use common::{
    get_healthcheck,
    set_bo,
    test_get_site,
};


pub fn get_routes() -> Vec<Route> {
    routes![
        get_healthcheck,
        set_bo,
        test_get_site,
    
        get_crawler,
        add_crawler,
        delete_crawler,
        update_crawler,
        get_crawlers,
        add_site
    ]
}