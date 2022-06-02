-- migrate:up
CREATE TABLE public.brands
(
    brand_id varchar(50) NOT NULL,
    name varchar(50) NOT NULL,
    company_name varchar(50),
    group_name varchar(50),
    description varchar(1000),
    email varchar(100),
    website_url varchar(100),
    kind varchar(25) NOT NULL,
    active boolean,
    address_line1 varchar(255),
    address_line2 varchar(255),
    address_city varchar(50),
    address_region varchar(50),
    address_postal_code varchar(10),
    address_country varchar(2),
    created timestamp without time zone NOT NULL,
    last_modified timestamp without time zone,
    version integer NOT NULL DEFAULT 1,
    CONSTRAINT "PK_brands" PRIMARY KEY (brand_id)
);

CREATE UNIQUE INDEX "Idx_brands_name"
    ON brands USING btree
        (name ASC NULLS LAST);

CREATE TABLE public.railways
(
    railway_id varchar(25) NOT NULL,
    name varchar(25) NOT NULL,
    company_name varchar(250),
    country varchar(2),
    operating_since timestamp without time zone,
    operating_until timestamp without time zone,
    active boolean,
    gauge_mm numeric(19,5),
    gauge_in numeric(19,5),
    track_type varchar(25),
    headquarters varchar(250),
    total_length_mi numeric(19,5),
    total_length_km numeric(19,5),
    website_url varchar(255),
    created timestamp without time zone NOT NULL,
    last_modified timestamp without time zone,
    version integer NOT NULL DEFAULT 1,
    CONSTRAINT "PK_railways" PRIMARY KEY (railway_id)
);

CREATE UNIQUE INDEX "Idx_railways_name"
    ON public.railways USING btree
        (name ASC NULLS LAST);

CREATE TABLE public.scales
(
    scale_id varchar(25) NOT NULL,
    name varchar(25) NOT NULL,
    ratio numeric(19,5) NOT NULL,
    gauge_mm numeric(19,5),
    gauge_in numeric(19,5),
    track_type varchar(25) NOT NULL,
    description varchar(250),
    standards varchar(100),
    created timestamp without time zone NOT NULL,
    last_modified timestamp without time zone,
    version integer NOT NULL DEFAULT 1,
    CONSTRAINT "PK_scales" PRIMARY KEY (scale_id)
);

CREATE UNIQUE INDEX "Idx_scales_name"
    ON public.scales USING btree
        (name ASC NULLS LAST);

CREATE TABLE public.catalog_items
(
    catalog_item_id varchar(40) NOT NULL,
    brand_id varchar(50) NOT NULL,
    scale_id varchar(25) NOT NULL,
    item_number varchar(10) NOT NULL,
    power_method varchar(2) NOT NULL,
    delivery_date varchar(10),
    available boolean,
    description varchar(250) NOT NULL,
    model_description varchar(2500),
    prototype_description varchar(2500),
    created timestamp without time zone NOT NULL,
    last_modified timestamp without time zone,
    version integer NOT NULL DEFAULT 1,
    CONSTRAINT "PK_catalog_items" PRIMARY KEY (catalog_item_id),
    CONSTRAINT "FK_catalog_items_brands" FOREIGN KEY (brand_id)
        REFERENCES public.brands (brand_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION,
    CONSTRAINT "FK_catalog_items_scales" FOREIGN KEY (scale_id)
        REFERENCES public.scales (scale_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
);

CREATE UNIQUE INDEX "Idx_catalog_items_brand_id_item_number"
    ON public.catalog_items USING btree
        (item_number ASC NULLS LAST, brand_id ASC NULLS LAST);

CREATE TABLE public.rolling_stocks
(
    rolling_stock_id uuid NOT NULL,
    catalog_item_id varchar(40) NOT NULL,
    railway_id varchar(25) NOT NULL,
    category varchar(25) NOT NULL,
    sub_category varchar(50) NOT NULL,
    epoch varchar(10) NOT NULL,
    min_radius numeric(19,5),
    couplers varchar(10),
    livery varchar(50),
    length_mm numeric(19,5),
    length_in numeric(19,5),
    type_name varchar(25),
    class_name varchar(15),
    road_number varchar(15),
    series varchar(50),
    depot varchar(100),
    dcc_interface varchar(10),
    control varchar(10),
    passenger_car_type varchar(25),
    freight_car_type varchar(25),
    service_level varchar(15),
    CONSTRAINT "PK_rolling_stocks" PRIMARY KEY (rolling_stock_id),
    CONSTRAINT "FK_rolling_stocks_catalog_items" FOREIGN KEY (catalog_item_id)
        REFERENCES public.catalog_items (catalog_item_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION,
    CONSTRAINT "FK_rolling_stocks_railways" FOREIGN KEY (railway_id)
        REFERENCES public.railways (railway_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
);

-- migrate:down
DROP TABLE public.rolling_stocks;

DROP INDEX public."Idx_catalog_items_brand_id_item_number";
DROP TABLE public.catalog_items;

DROP INDEX public."Idx_scales_name";
DROP TABLE public.scales;

DROP INDEX public."Idx_railways_name";
DROP TABLE public.railways;

DROP INDEX public."Idx_brands_name";
DROP TABLE public.brands;

