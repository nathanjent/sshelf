pub const ENTITY_COUNT: usize = 3;

static mut X: *mut [f32; ENTITY_COUNT] =
    (0x19a0 + ENTITY_COUNT * 0) as *mut [f32; ENTITY_COUNT];
static mut Y: *mut [f32; ENTITY_COUNT] =
    (0x19a0 + ENTITY_COUNT * 1) as *mut [f32; ENTITY_COUNT];
static mut VX: *mut [f32; ENTITY_COUNT] =
    (0x19a0 + ENTITY_COUNT * 2) as *mut [f32; ENTITY_COUNT];
static mut VY: *mut [f32; ENTITY_COUNT] =
    (0x19a0 + ENTITY_COUNT * 3) as *mut [f32; ENTITY_COUNT];
static mut VX_MAX: *mut [f32; ENTITY_COUNT] =
    (0x19a0 + ENTITY_COUNT * 4) as *mut [f32; ENTITY_COUNT];
static mut VY_MAX: *mut [f32; ENTITY_COUNT] =
    (0x19a0 + ENTITY_COUNT * 5) as *mut [f32; ENTITY_COUNT];
static mut AX: *mut [f32; ENTITY_COUNT] =
    (0x19a0 + ENTITY_COUNT * 6) as *mut [f32; ENTITY_COUNT];
static mut AY: *mut [f32; ENTITY_COUNT] =
    (0x19a0 + ENTITY_COUNT * 7) as *mut [f32; ENTITY_COUNT];
static mut FLIP: *mut [bool; ENTITY_COUNT] =
    (0x19a0 + ENTITY_COUNT * 8) as *mut [bool; ENTITY_COUNT];

#[derive(Debug, Default, Clone, Copy)]
pub struct Entity<'a> {
    pub id: usize,
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub vx_max: f32,
    pub vy_max: f32,
    pub ax: f32,
    pub ay: f32,
    pub sprite: Option<&'a str>,
    pub flip: bool,
}

pub fn get_entity<'a>(i: usize) -> Option<Entity<'a>> {
    unsafe {
        Some(Entity {
            id: i,
            x: X.as_ref().map(|a| a[i])?,
            y: Y.as_ref().map(|a| a[i])?,
            vx: VX.as_ref().map(|a| a[i])?,
            vy: VY.as_ref().map(|a| a[i])?,
            vx_max: VX_MAX.as_ref().map(|a| a[i])?,
            vy_max: VY_MAX.as_ref().map(|a| a[i])?,
            ax: AX.as_ref().map(|a| a[i])?,
            ay: AY.as_ref().map(|a| a[i])?,
            sprite: None,
            flip: false,
        })
    }
}

pub fn set_entity(entity: Entity) {
    unsafe {
        let id = entity.id;
        if let Some(x) = X.as_mut() {
            x[id] = entity.x;
        }
        if let Some(y) = Y.as_mut() {
            y[id] = entity.y;
        }

        if let Some(vx) = VX.as_mut() {
            vx[id] = entity.vx;
        }
        if let Some(vy) = VY.as_mut() {
            vy[id] = entity.vy;
        }

        if let Some(vx_max) = VX_MAX.as_mut() {
            vx_max[id] = entity.vx_max;
        }
        if let Some(vy_max) = VY_MAX.as_mut() {
            vy_max[id] = entity.vy_max;
        }

        if let Some(ax) = AX.as_mut() {
            ax[id] = entity.ax;
        }
        if let Some(ay) = AY.as_mut() {
            ay[id] = entity.ay;
        }

        if let Some(flip) = FLIP.as_mut() {
            flip[id] = entity.flip;
        }
    }
}
