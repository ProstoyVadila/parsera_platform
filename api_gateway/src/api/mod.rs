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
    test_state_rabbit,
    get_spawn_task,
};


pub fn get_routes() -> Vec<Route> {
    routes![
        get_healthcheck,
        set_bo,
        test_get_site,
        get_spawn_task,
        test_state_rabbit,
    
        get_crawler,
        add_crawler,
        delete_crawler,
        update_crawler,
        get_crawlers,
        add_site
    ]
}