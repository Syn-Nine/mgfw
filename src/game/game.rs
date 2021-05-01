use crate::mgfw;

#[derive(Default)]
pub struct GameDataHeap {
    // WARNING: Anything below this line is not in cache!
    temp: i32,
}

pub struct GameData {
    pub heap: *mut GameDataHeap,
    id: usize,
    hpid: usize,
    pid: usize,
    frame: u8,
}

#[rustfmt::skip]
pub fn initialize(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) {

    world.parse_world("world.dat");
    cache.id = 0;
    cache.hpid = 1;
    cache.pid = 3;
    heap.temp = 0;    
    world.entity_set_velocity_xy(cache.id, 1.0 * 64.0, -0.5 * 64.0);
    world.entity_set_velocity_xy(cache.pid, -0.5 * 32.0, -0.85 * 32.0);
}

#[rustfmt::skip]
pub fn update(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) -> bool {
    let mut expect_blown = false;

    cache.frame = (cache.frame + 1) % 128;

    let xres = 320.0;
    let yres = 200.0;

    // Amortize workload
    if 0 == cache.frame % 8 {
        let w = world.text_get_width(cache.id) as f32 * 1.5;
        if 0.0 < w {
            let pos = world.entity_get_position(cache.id);
            let mut del = world.entity_get_velocity(cache.id);

            if (0.0 > del.y && 0.0 > pos.y) || (0.0 < del.y && yres - 14.0 < pos.y) {
                del.y = -del.y;
            }

            if (0.0 > del.x && 0.0 > pos.x) || (0.0 < del.x && xres - w < pos.x) {
                del.x = -del.x;
            }
            
            world.entity_set_velocity(cache.id, del);
        }

        let pos = world.entity_get_position(cache.pid);
        let mut del = world.entity_get_velocity(cache.pid);

        if (0.0 > del.y && 0.0 + 20.0 > pos.y) || (0.0 < del.y && yres - 20.0 < pos.y) {
            del.y = -del.y;
        }

        if (0.0 > del.x && 0.0 + 20.0 > pos.x) || (0.0 < del.x && xres - 20.0 < pos.x) {
            del.x = -del.x;
        }
        
        world.entity_set_velocity(cache.pid, del);
    }

    if 1 == cache.frame {
        heap.temp += 1;
        world.entity_set_text(cache.hpid, format!("heap data: {}", heap.temp));
        expect_blown = true;
    }
    expect_blown
}

#[rustfmt::skip]
pub fn shutdown(cache: &mut GameData, _heap: &mut GameDataHeap) {
    // re-box and consume to deallocate memory
    let _temp = unsafe { Box::from_raw(cache.heap) };
}
