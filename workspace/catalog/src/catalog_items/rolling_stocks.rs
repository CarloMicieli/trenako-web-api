use crate::catalog_items::categories::{
    Category, FreightCarType, LocomotiveType, PassengerCarType, TrainType,
};
use crate::catalog_items::controls::{Control, DccInterface};
use crate::catalog_items::epoch::Epoch;
use crate::catalog_items::length::LengthOverBuffer;
use crate::catalog_items::service_levels::ServiceLevel;
use crate::railways::Railway;

#[derive(Debug, PartialEq)]
pub enum RollingStock {
    Locomotive {
        class_name: String,
        road_number: String,
        series: Option<String>,
        railway: Railway,
        epoch: Epoch,
        category: LocomotiveType,
        depot: Option<String>,
        livery: Option<String>,
        length_over_buffer: Option<LengthOverBuffer>,
        control: Option<Control>,
        dcc_interface: Option<DccInterface>,
    },
    FreightCar {
        type_name: String,
        road_number: Option<String>,
        railway: Railway,
        epoch: Epoch,
        category: Option<FreightCarType>,
        depot: Option<String>,
        livery: Option<String>,
        length_over_buffer: Option<LengthOverBuffer>,
    },
    PassengerCar {
        type_name: String,
        road_number: Option<String>,
        railway: Railway,
        epoch: Epoch,
        category: Option<PassengerCarType>,
        service_level: Option<ServiceLevel>,
        depot: Option<String>,
        livery: Option<String>,
        length_over_buffer: Option<LengthOverBuffer>,
    },
    Train {
        type_name: String,
        road_number: Option<String>,
        n_of_elements: u8,
        railway: Railway,
        epoch: Epoch,
        category: Option<TrainType>,
        depot: Option<String>,
        livery: Option<String>,
        length_over_buffer: Option<LengthOverBuffer>,
        control: Option<Control>,
        dcc_interface: Option<DccInterface>,
    },
}

impl RollingStock {
    pub fn depot(&self) -> Option<&str> {
        match self {
            RollingStock::Locomotive {
                depot: Some(depot), ..
            } => Some(depot),
            _ => None,
        }
    }

    pub fn class_name(&self) -> Option<&str> {
        match self {
            RollingStock::Locomotive { class_name, .. } => Some(class_name),
            _ => None,
        }
    }

    pub fn road_number(&self) -> Option<&str> {
        match self {
            RollingStock::Locomotive { road_number, .. } => Some(road_number),
            _ => None,
        }
    }

    pub fn series(&self) -> Option<&str> {
        match self {
            RollingStock::Locomotive {
                series: Some(series),
                ..
            } => Some(series),
            _ => None,
        }
    }

    pub fn livery(&self) -> Option<&str> {
        match self {
            RollingStock::Locomotive {
                livery: Some(livery),
                ..
            } => Some(livery),
            _ => None,
        }
    }

    /// Returns the category for this rolling stock
    pub fn category(&self) -> Category {
        match self {
            RollingStock::Locomotive { .. } => Category::Locomotives,
            RollingStock::FreightCar { .. } => Category::FreightCars,
            RollingStock::PassengerCar { .. } => Category::PassengerCars,
            RollingStock::Train { .. } => Category::Trains,
        }
    }

    // pub fn epoch(&self) -> Epoch {
    //     match &self {
    //         RollingStock::Locomotive { epoch, .. } => *epoch.clone(),
    //         RollingStock::FreightCar { epoch, .. } => *epoch.clone(),
    //         RollingStock::PassengerCar { epoch, .. } => *epoch.clone(),
    //         RollingStock::Train { epoch, .. } => *epoch.clone(),
    //     }
    // }

    pub fn is_locomotive(&self) -> bool {
        self.category() == Category::Locomotives
    }

    pub fn with_decoder(&self) -> bool {
        match self {
            RollingStock::Locomotive {
                control: Some(control),
                ..
            } => *control != Control::DccReady,
            RollingStock::Train {
                control: Some(control),
                ..
            } => *control != Control::DccReady,
            _ => false,
        }
    }

    pub fn dcc_interface(&self) -> Option<DccInterface> {
        match self {
            RollingStock::Locomotive {
                dcc_interface: Some(dcc_interface),
                ..
            } => Some(*dcc_interface),
            RollingStock::Train {
                dcc_interface: Some(dcc_interface),
                ..
            } => Some(*dcc_interface),
            _ => None,
        }
    }

    /// Creates a new freight car rolling stock
    pub fn new_freight_car(
        type_name: String,
        road_number: Option<String>,
        railway: Railway,
        epoch: Epoch,
        category: Option<FreightCarType>,
        depot: Option<String>,
        livery: Option<String>,
        length_over_buffer: Option<LengthOverBuffer>,
    ) -> Self {
        RollingStock::FreightCar {
            type_name,
            road_number,
            railway,
            epoch,
            category,
            depot,
            livery,
            length_over_buffer,
        }
    }

    /// Creates a new train rolling stock
    pub fn new_train(
        type_name: String,
        road_number: Option<String>,
        n_of_elements: u8,
        railway: Railway,
        epoch: Epoch,
        category: Option<TrainType>,
        depot: Option<String>,
        livery: Option<String>,
        length_over_buffer: Option<LengthOverBuffer>,
        control: Option<Control>,
        dcc_interface: Option<DccInterface>,
    ) -> Self {
        RollingStock::Train {
            type_name,
            road_number,
            n_of_elements,
            railway,
            epoch,
            category,
            depot,
            livery,
            length_over_buffer,
            control,
            dcc_interface,
        }
    }

    /// Creates a new locomotive rolling stock
    pub fn new_locomotive(
        class_name: String,
        road_number: String,
        series: Option<String>,
        railway: Railway,
        epoch: Epoch,
        category: LocomotiveType,
        depot: Option<String>,
        livery: Option<String>,
        length_over_buffer: Option<LengthOverBuffer>,
        control: Option<Control>,
        dcc_interface: Option<DccInterface>,
    ) -> Self {
        RollingStock::Locomotive {
            class_name,
            road_number,
            series,
            railway,
            epoch,
            category,
            depot,
            livery,
            length_over_buffer,
            control,
            dcc_interface,
        }
    }

    /// Creates a new passenger car rolling stock
    pub fn new_passenger_car(
        type_name: String,
        road_number: Option<String>,
        railway: Railway,
        epoch: Epoch,
        category: Option<PassengerCarType>,
        service_level: Option<ServiceLevel>,
        depot: Option<String>,
        livery: Option<String>,
        length_over_buffer: Option<LengthOverBuffer>,
    ) -> Self {
        RollingStock::PassengerCar {
            type_name,
            road_number,
            railway,
            epoch,
            category,
            service_level,
            depot,
            livery,
            length_over_buffer,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod rolling_stock_tests {
        use super::*;
        use crate::catalog_items::categories::{
            FreightCarType, LocomotiveType, TrainType,
        };
        use crate::catalog_items::controls::{Control, DccInterface};
        use crate::railways::Railway;

        #[test]
        fn it_should_create_new_locomotives() {
            let railway_fs = Railway::new("FS");

            let rs = RollingStock::new_locomotive(
                String::from("E.656"),
                String::from("E.656 210"),
                Some(String::from("1a serie")),
                railway_fs.clone(),
                Epoch::IV,
                LocomotiveType::ElectricLocomotive,
                Some(String::from("Milano Centrale")),
                Some(String::from("blu/grigio")),
                Some(LengthOverBuffer::new(210)),
                Some(Control::DccReady),
                Some(DccInterface::Nem652),
            );

            match rs {
                RollingStock::Locomotive {
                    class_name,
                    road_number,
                    series,
                    railway,
                    epoch,
                    category,
                    depot,
                    livery,
                    length_over_buffer,
                    control,
                    dcc_interface,
                    ..
                } => {
                    assert_eq!(class_name, String::from("E.656"));
                    assert_eq!(road_number, String::from("E.656 210"));
                    assert_eq!(series, Some(String::from("1a serie")));
                    assert_eq!(railway, railway_fs);
                    assert_eq!(epoch, Epoch::IV);
                    assert_eq!(category, LocomotiveType::ElectricLocomotive);
                    assert_eq!(depot, Some(String::from("Milano Centrale")));
                    assert_eq!(livery, Some(String::from("blu/grigio")));
                    assert_eq!(
                        length_over_buffer,
                        Some(LengthOverBuffer::new(210))
                    );
                    assert_eq!(control, Some(Control::DccReady));
                    assert_eq!(dcc_interface, Some(DccInterface::Nem652));
                }
                _ => panic!(
                    "Invalid rolling stock type - expect a locomotive here!!!!"
                ),
            }
        }

        #[test]
        fn it_should_create_new_trains() {
            let railway_fs = Railway::new("FS");

            let rs = RollingStock::new_train(
                String::from("Etr 220"),
                None,
                4,
                railway_fs.clone(),
                Epoch::IV,
                Some(TrainType::ElectricMultipleUnits),
                Some(String::from("Milano Centrale")),
                Some(String::from("grigio nebbia/verde magnolia")),
                Some(LengthOverBuffer::new(800)),
                Some(Control::DccReady),
                Some(DccInterface::Nem652),
            );

            match rs {
                RollingStock::Train {
                    type_name,
                    road_number,
                    n_of_elements,
                    railway,
                    epoch,
                    category,
                    depot,
                    livery,
                    length_over_buffer,
                    control,
                    dcc_interface,
                    ..
                } => {
                    assert_eq!(type_name, String::from("Etr 220"));
                    assert_eq!(road_number, None);
                    assert_eq!(n_of_elements, 4);
                    assert_eq!(railway, railway_fs);
                    assert_eq!(epoch, Epoch::IV);
                    assert_eq!(
                        category,
                        Some(TrainType::ElectricMultipleUnits)
                    );
                    assert_eq!(depot, Some(String::from("Milano Centrale")));
                    assert_eq!(
                        livery,
                        Some(String::from("grigio nebbia/verde magnolia"))
                    );
                    assert_eq!(
                        length_over_buffer,
                        Some(LengthOverBuffer::new(800))
                    );
                    assert_eq!(control, Some(Control::DccReady));
                    assert_eq!(dcc_interface, Some(DccInterface::Nem652));
                }
                _ => panic!(
                    "Invalid rolling stock type - expect a train here!!!!"
                ),
            }
        }

        #[test]
        fn it_should_create_new_passenger_cars() {
            let railway_fs = Railway::new("FS");

            let rs = RollingStock::new_passenger_car(
                String::from("UIC-Z"),
                None,
                railway_fs.clone(),
                Epoch::IV,
                Some(PassengerCarType::OpenCoach),
                Some(ServiceLevel::FirstClass),
                None,
                Some(String::from("bandiera")),
                Some(LengthOverBuffer::new(303)),
            );

            match rs {
                RollingStock::PassengerCar {
                    type_name,
                    road_number,
                    railway,
                    epoch,
                    category,
                    depot,
                    livery,
                    length_over_buffer,
                    service_level,
                    ..
                } => {
                    assert_eq!(type_name, String::from("UIC-Z"));
                    assert_eq!(road_number, None);
                    assert_eq!(service_level, Some(ServiceLevel::FirstClass));
                    assert_eq!(railway, railway_fs);
                    assert_eq!(epoch, Epoch::IV);
                    assert_eq!(None, depot);
                    assert_eq!(category, Some(PassengerCarType::OpenCoach));
                    assert_eq!(livery, Some(String::from("bandiera")));
                    assert_eq!(length_over_buffer, Some(LengthOverBuffer::new(303)));
                }
                _ => panic!("Invalid rolling stock type - expect a passenger car here!!!!"),
            }
        }

        #[test]
        fn it_should_create_new_freight_cars() {
            let railway_fs = Railway::new("FS");

            let rs = RollingStock::new_freight_car(
                String::from("Gbhs"),
                None,
                railway_fs.clone(),
                Epoch::V,
                Some(FreightCarType::SwingRoofWagon),
                None,
                Some(String::from("marrone")),
                Some(LengthOverBuffer::new(122)),
            );

            match rs {
                RollingStock::FreightCar {
                    type_name,
                    road_number,
                    railway,
                    epoch,
                    category,
                    depot,
                    livery,
                    length_over_buffer,
                    ..
                } => {
                    assert_eq!(type_name, String::from("Gbhs"));
                    assert_eq!(road_number, None);
                    assert_eq!(railway, railway_fs);
                    assert_eq!(epoch, Epoch::V);
                    assert_eq!(None, depot);
                    assert_eq!(category, Some(FreightCarType::SwingRoofWagon));
                    assert_eq!(livery, Some(String::from("marrone")));
                    assert_eq!(length_over_buffer, Some(LengthOverBuffer::new(122)));
                }
                _ => panic!("Invalid rolling stock type - expect a freight car here!!!!"),
            }
        }
    }
}
