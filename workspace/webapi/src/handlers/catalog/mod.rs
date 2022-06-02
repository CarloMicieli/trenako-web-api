use crate::handlers::catalog::brands as brands_handler;
use crate::handlers::catalog::catalog_items as catalog_items_handler;
use crate::handlers::catalog::railways as railways_handler;
use crate::handlers::catalog::scales as scales_handler;
use actix_web::web;

pub mod brands;
pub mod catalog_items;
pub mod railways;
pub mod scales;

pub fn config_services(cfg: &mut web::ServiceConfig) {
    #[rustfmt::skip]
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/brands")
                    .service(
                        web::resource("")
                            .route(web::get().to(brands_handler::get_all_brands))
                            .route(web::post().to(brands_handler::post_new_brand))
                    )
                    .service(
                        web::resource("/{brand}")
                            .route(web::delete().to(brands_handler::delete_brand))
                            .route(web::get().to(brands_handler::get_brand))
                            .route(web::put().to(brands_handler::edit_brand))
                    )
            )
            .service(
                web::scope("/catalog-items")
                    .service(
                        web::resource("")
                            .route(web::post().to(catalog_items_handler::post_new_catalog_item))
                    )
                    .service(
                        web::resource("/{catalog_item_id}")
                            .route(web::delete().to(catalog_items_handler::delete_catalog_item))
                            .route(web::get().to(catalog_items_handler::get_catalog_item))
                            .route(web::put().to(catalog_items_handler::edit_catalog_item))
                    )
            )
            .service(
                web::scope("/railways")
                    .service(
                        web::resource("")
                            .route(web::get().to(railways_handler::get_all_railways))
                            .route(web::post().to(railways_handler::post_new_railway))
                    )
                    .service(
                        web::resource("/{railway}")
                            .route(web::delete().to(railways_handler::delete_railway))
                            .route(web::get().to(railways_handler::get_railway))
                            .route(web::put().to(railways_handler::edit_railway))
                    )
            )
            .service(
                web::scope("/scales")
                    .service(
                        web::resource("")
                            .route(web::get().to(scales_handler::get_all_scales))
                            .route(web::post().to(scales_handler::post_new_scale))
                    )
                    .service(
                        web::resource("/{scale}")
                            .route(web::delete().to(scales_handler::delete_scale))
                            .route(web::get().to(scales_handler::get_scale))
                            .route(web::put().to(scales_handler::edit_scale))
                    )
            )
    );
}
