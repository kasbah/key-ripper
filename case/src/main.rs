use glam::{DVec2, DVec3};

// All units are in millimeters.
const PCB_THICKNESS: f64 = 1.6;
const PCB_WIDTH: f64 = 285.75;
const PCB_HEIGHT: f64 = 114.3;
const TOP_PLATE_THICKNESS: f64 = 1.6;

// The origin point for this board is the top left corner
// of the PCB, on the top surface. The PCB rests on this
// shelf. Positive X values go to the right, positive Y
// values go down (like screen coordinates).
const ORIGIN: DVec3 = DVec3::new(0.0, 0.0, 0.0);

const PCB_TOP: f64 = 0.0;
const PCB_BOTTOM: f64 = PCB_TOP - PCB_THICKNESS;

const TOP_PLATE_BOTTOM: f64 = 3.4;
const TOP_PLATE_TOP: f64 = TOP_PLATE_BOTTOM + TOP_PLATE_THICKNESS;

const PCB_SHELF_THICKNESS_TOP: f64 = 2.75;
const PCB_SHELF_THICKNESS_BOTTOM: f64 = 4.0;

// Top plate support post locations
const SUPPORT_POST_RADIUS: f64 = 2.25;
const SUPPORT_POST_DIST_FROM_EDGE: f64 = 2.5;

enum PostDirection {
    Up,
    Down,
    Left,
    Right,
}

struct SupportPost {
    pos: DVec2,
    direction: PostDirection,
}

const SUPPORT_POSTS: &[SupportPost] = &[
    SupportPost {
        pos: DVec2::new(117.7, SUPPORT_POST_DIST_FROM_EDGE),
        direction: PostDirection::Up,
    },
    SupportPost {
        pos: DVec2::new(204.75, SUPPORT_POST_DIST_FROM_EDGE),
        direction: PostDirection::Up,
    },
    SupportPost {
        pos: DVec2::new(SUPPORT_POST_DIST_FROM_EDGE, 57.15),
        direction: PostDirection::Left,
    },
    SupportPost {
        pos: DVec2::new(PCB_WIDTH - SUPPORT_POST_DIST_FROM_EDGE, 57.15),
        direction: PostDirection::Right,
    },
    SupportPost {
        pos: DVec2::new(80.95, PCB_HEIGHT - SUPPORT_POST_DIST_FROM_EDGE),
        direction: PostDirection::Down,
    },
    SupportPost {
        pos: DVec2::new(200.05, PCB_HEIGHT - SUPPORT_POST_DIST_FROM_EDGE),
        direction: PostDirection::Down,
    },
];

// The radius of the holes to drill in the bottom of the case to allow
// for threaded M3 rubber feet to pass through and attach to the
// threaded inserts on the PCB.
const BOTTOM_CUTOUT_RADIUS: f64 = 2.1;

const FEET_CUTOUTS: &[DVec2] = &[
    DVec2::new(19.05, 18.25),
    DVec2::new(266.65, 18.25),
    DVec2::new(19.05, 96.45),
    DVec2::new(266.65, 96.45),
];

// USB C Connector Cutout
const USB_CUTOUT_PADDING: f64 = PCB_THICKNESS;
const USB_WIDTH: f64 = 9.0;
const USB_HEIGHT: f64 = 7.45;

const USB_LEFT: f64 = 21.724;
const USB_RIGHT: f64 = USB_LEFT + USB_WIDTH;
const USB_TOP: f64 = -3.04;
const USB_BOTTOM: f64 = USB_TOP + USB_HEIGHT;

// Keep out zones for space bar stabilizer
const STABILIZER_SCREW_KEEPOUT_WIDTH: f64 = 8.0;

// A list of the left sides of each stabilizer screw "zones"
// where the PCB shelf should be cut away to accomodate
// stabilizer screws on the bottom of the PCB.
const STABILIZER_KEEPOUTS: &[f64] = &[87.0, 187.24];

// Pinholes for BOOT + reset buttons
const PINHOLE_BUTTON_RADIUS: f64 = 1.1;

const PINHOLE_LOCATIONS: &[DVec2] = &[DVec2::new(35.85, 53.95), DVec2::new(8.425, 86.075)];

fn main() {}
