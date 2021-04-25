-- migrate:up
CREATE TABLE public.shops
(
    shop_id uuid NOT NULL,
    name varchar(50) NOT NULL,
    slug varchar(50) NOT NULL,
    website_url varchar(100),
    phone_number varchar(50),
    email varchar(100),
    address_line1 varchar(255),
    address_line2 varchar(255),
    address_city varchar(50),
    address_region varchar(50),
    address_postal_code varchar(10),
    address_country varchar(2),
    created timestamp without time zone NOT NULL,
    last_modified timestamp without time zone,
    version integer NOT NULL DEFAULT 1,
    CONSTRAINT "PK_shops" PRIMARY KEY (shop_id)
);

CREATE INDEX "Idx_shops_slug"
    ON public.shops USING btree
    (slug ASC NULLS LAST);

CREATE TABLE public.favourite_shops
(
    shop_id uuid NOT NULL,
    owner varchar(25) NOT NULL,
    CONSTRAINT "PK_favourite_shops" PRIMARY KEY (shop_id, owner),
    CONSTRAINT "FK_favourite_shops_users" FOREIGN KEY (owner)
        REFERENCES public.users (user_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION,
    CONSTRAINT "FK_favourite_shops_shop_id" FOREIGN KEY (shop_id)
        REFERENCES public.shops (shop_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
);

CREATE TABLE public.collections
(
    collection_id uuid NOT NULL,
    owner varchar(25) NOT NULL,
    notes varchar(250),
    created timestamp without time zone NOT NULL,
    last_modified timestamp without time zone,
    version integer NOT NULL DEFAULT 1,
    CONSTRAINT "PK_collections" PRIMARY KEY (collection_id),
    CONSTRAINT "FK_collections_users" FOREIGN KEY (owner)
        REFERENCES public.users (user_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
);

CREATE INDEX "Idx_collections_owner"
    ON public.collections USING btree
    (owner ASC NULLS LAST);

CREATE TABLE public.collection_items
(
    collection_item_id uuid NOT NULL,
    collection_id uuid NOT NULL,
    catalog_item_id uuid NOT NULL,
    condition varchar(15) NOT NULL,
    price numeric(19,5) NOT NULL,
    currency varchar(3) NOT NULL,
    purchased_at uuid,
    added_date timestamp without time zone NOT NULL,
    removed_date timestamp without time zone,
    notes varchar(150),
    CONSTRAINT "PK_collection_items" PRIMARY KEY (collection_item_id),
    CONSTRAINT "FK_collection_items_catalog_items" FOREIGN KEY (catalog_item_id)
        REFERENCES public.catalog_items (catalog_item_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION,
    CONSTRAINT "FK_collection_items_collections" FOREIGN KEY (collection_id)
        REFERENCES public.collections (collection_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION,
    CONSTRAINT "FK_collection_items_shops" FOREIGN KEY (purchased_at)
        REFERENCES public.shops (shop_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
);

CREATE TABLE public.wishlists
(
    wishlist_id uuid NOT NULL,
    owner varchar(25) NOT NULL,
    slug varchar(100) NOT NULL,
    wishlist_name varchar(100),
    visibility varchar(15) NOT NULL,
    budget numeric(19,5),
    currency varchar(3),
    created timestamp without time zone NOT NULL,
    last_modified timestamp without time zone,
    version integer NOT NULL DEFAULT 1,
    CONSTRAINT "PK_wishlists" PRIMARY KEY (wishlist_id),
    CONSTRAINT "FK_wishlists_users" FOREIGN KEY (owner)
        REFERENCES public.users (user_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
);

CREATE INDEX "Idx_wishlists_slug"
    ON public.wishlists USING btree
    (slug ASC NULLS LAST);

CREATE TABLE public.wishlist_items
(
    wishlist_item_id uuid NOT NULL,
    wishlist_id uuid NOT NULL,
    catalog_item_id uuid NOT NULL,
    priority varchar(10) NOT NULL,
    added_date timestamp without time zone NOT NULL,
    removed_date timestamp without time zone,
    price numeric(19,5),
    currency varchar(3),
    notes varchar(150),
    CONSTRAINT "PK_wishlist_items" PRIMARY KEY (wishlist_item_id),
    CONSTRAINT "FK_wishlist_items_catalog_items" FOREIGN KEY (catalog_item_id)
        REFERENCES public.catalog_items (catalog_item_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION,
    CONSTRAINT "FK_wishlist_items_wishlists" FOREIGN KEY (wishlist_id)
        REFERENCES public.wishlists (wishlist_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
);

-- migrate:down
DROP TABLE public.wishlist_items;

DROP INDEX public."Idx_wishlists_slug";
DROP TABLE public.wishlists;

DROP TABLE public.collection_items;

DROP INDEX public."Idx_collections_owner";
DROP TABLE public.collections;

DROP TABLE public.favourite_shops;

DROP INDEX public."Idx_shops_slug";
DROP TABLE public.shops;

