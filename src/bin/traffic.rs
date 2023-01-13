#![allow(dead_code)]
#![allow(unused_variables)]

use std::marker::PhantomData;

// Traffic light state transitions:
// Red -> Green
// Green -> Yellow
// Yellow -> Red
//
// All -> Fault
// Fault -> Red

// We use a trait so only specific structures can be used as states.
trait SignalState {}

// Each state is represented by a struct:
struct Red;
struct Yellow;
struct Green;
struct Fault; // flashing red

// No need for an actual implementation. This is sufficient:
impl SignalState for Red {}
impl SignalState for Yellow {}
impl SignalState for Green {}
impl SignalState for Fault {}

struct TrafficSignal<S: SignalState> {
    // We are using a generic structure, but we aren't using the generic data within
    // the structure itself. Rust disallows unused generics, so we use a `PhantomData`
    // `PhantomData` allows the generic to be used as a field in the struct and is an
    // indication that we are aware of unused generics.
    _marker: PhantomData<S>,
}

// Functionality that applies to all states of the TrafficSignal.
impl<S: SignalState> TrafficSignal<S> {
    // Creates a new `TrafficSignal`. Used by all states when transitioning.
    fn transition() -> TrafficSignal<S> {
        TrafficSignal {
            // Set the PhantomData for the compiler:
            _marker: PhantomData,
        }
    }

    // Something went wrong (car crashed into pole, power outage, communication failure
    // with other signals, maintenance, etc). This state can be reached from any other state,
    // so we create it within this impl block.
    pub fn fault(self) -> TrafficSignal<Fault> {
        TrafficSignal::transition()
    }
}

// Functionality when we are in the `Fault` state.
impl TrafficSignal<Fault> {
    // We can only clear a fault when in the `Fault` state. The signal transitions to
    // `Red`.
    pub fn clear_fault(self) -> TrafficSignal<Red> {
        TrafficSignal::transition()
    }

    pub fn initial() -> TrafficSignal<Fault> {
        TrafficSignal::transition()
    }
}

// Functionality when we are in the `Red` state.
impl TrafficSignal<Red> {
    // `Red` transitions to `Green`.
    pub fn next(self) -> TrafficSignal<Green> {
        TrafficSignal::transition()
    }
}

// Functionality when we are in the `Green` state.
impl TrafficSignal<Green> {
    // `Green` transitions to `Yellow`.
    pub fn next(self) -> TrafficSignal<Yellow> {
        TrafficSignal::transition()
    }
}

// Functionality when we are in the `Yellow` state.
impl TrafficSignal<Yellow> {
    // `Yellow` transitions to `Red`.
    pub fn next(self) -> TrafficSignal<Red> {
        TrafficSignal::transition()
    }
}

fn main() {
    // We start the traffic signal in the `Fault` state. Once the system is fully
    // operational, the fault can be cleared.
    let signal = TrafficSignal::initial();
    let signal: TrafficSignal<Red> = signal.clear_fault();

    let signal: TrafficSignal<Green> = signal.next();
    // Since Rust is keeping track of which state we are in, each call to `.next()` is
    // accessing a distinct `next` function implemented on the states above.
    let signal: TrafficSignal<Yellow> = signal.next();
    let signal: TrafficSignal<Red> = signal.next();

    // Since the correct state transitions are guaranteed by the compiler, we can
    // omit the type annotations:
    let signal = signal.next(); // green
    let signal = signal.next(); // yellow
    let signal = signal.next(); // red
}
