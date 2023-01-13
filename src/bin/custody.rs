#![allow(dead_code)]
#![allow(unused_variables)]

use std::rc::Rc;

// type aliases for clarity
type Name = &'static str;
type Address = &'static str;
type Date = &'static str;

trait PackageState {
    fn audit(&self) -> String;
}

macro_rules! impl_package_state {
    ($($state:ident),+) => {
        $(
        impl PackageState for $state {
            fn audit(&self) -> String {
                format!("{self:?}")
            }
        }
        )+
    };
}

#[derive(Debug)]
struct Queued;

#[derive(Debug)]
struct Picking {
    picker: Name,
}

#[derive(Debug)]
struct Loading {
    truck_id: u32,
}

#[derive(Debug)]
struct OutForDelivery {
    driver: Name,
    address: Address,
    truck_id: u32,
}

#[derive(Debug)]
struct Delivered {
    date: Date,
}

#[derive(Debug)]
struct Finalized;

impl_package_state!(
    Queued,
    Picking,
    Loading,
    OutForDelivery,
    Delivered,
    Finalized
);

struct Package<S: PackageState> {
    item_number: u32,
    custody: Vec<Rc<dyn PackageState>>,
    state: Rc<S>,
}

impl<S: PackageState> Package<S> {
    fn transition<N: PackageState + 'static>(self, next: N) -> Package<N> {
        let mut custody = self.custody;
        let next = Rc::new(next);
        custody.push(next.clone());

        Package {
            item_number: self.item_number,
            custody,
            state: next,
        }
    }

    pub fn chain_of_custody(&self) -> String {
        self.custody
            .iter()
            .enumerate()
            .map(|(i, ev)| {
                let i = i + 1;
                format!("{i}: {}", ev.audit())
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl Package<Queued> {
    pub fn new(item_number: u32) -> Package<Queued> {
        Package {
            item_number,
            custody: vec![Rc::new(Queued)],
            state: Rc::new(Queued),
        }
    }

    pub fn pick(self, picker: Name) -> Package<Picking> {
        self.transition(Picking { picker })
    }
}

impl Package<Picking> {
    pub fn load(self, truck_id: u32) -> Package<Loading> {
        let next_state = Loading { truck_id };
        self.transition(next_state)
    }
}

impl Package<Loading> {
    pub fn deliver_to(self, driver: Name, address: Address) -> Package<OutForDelivery> {
        let truck_id = self.state.truck_id;
        self.transition(OutForDelivery {
            driver,
            address,
            truck_id,
        })
    }
}

impl Package<OutForDelivery> {
    pub fn delivered(self, date: Date) -> Package<Delivered> {
        self.transition(Delivered { date })
    }
}

impl Package<Delivered> {
    pub fn finalize(self) -> Package<Finalized> {
        self.transition(Finalized)
    }
}

fn main() {
    // Type annotations included to see transitions:
    let pkg: Package<Queued> = Package::new(123);
    let pkg: Package<Picking> = pkg.pick("Sandra");
    let pkg: Package<Loading> = pkg.load(8);
    let pkg: Package<OutForDelivery> = pkg.deliver_to("Gary", "Alaska");
    let pkg: Package<Delivered> = pkg.delivered("Friday");
    let pkg: Package<Finalized> = pkg.finalize();

    println!("{}", pkg.chain_of_custody());

    // No annotations:
    let pkg = Package::new(123);
    let pkg = pkg.pick("Sandra");
    let pkg = pkg.load(8);
    let pkg = pkg.deliver_to("Gary", "Alaska");
    let pkg = pkg.delivered("Friday");
    let pkg = pkg.finalize();

    // Bonus - chained state transitions:
    let pkg = Package::new(123)
        .pick("Sandra")
        .load(8)
        .deliver_to("gary", "alaska")
        .delivered("Friday")
        .finalize();
}
