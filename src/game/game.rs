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
    frame: u8,
    pos: mgfw::ecs::Position,
    del: mgfw::ecs::Position,
}

#[rustfmt::skip]
pub fn initialize(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) {

    world.parse_world("world.dat");
    cache.id = 0;
    cache.hpid = 1;
    heap.temp = 0;    
    cache.del = mgfw::ecs::Position { x: 1.0, y: -0.5 };
    cache.pos = mgfw::ecs::Position { x: 100.0, y: 100.0 };
}

#[rustfmt::skip]
pub fn update(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World, micros: u128) -> bool {
    let mut expect_blown = false;

    cache.frame = (cache.frame + 1) % 128;

    // Amortize workload
    if 0 == cache.frame % 8 {
        let w = world.text_get_width(cache.id) as f32;
        if 0.0 < w {
            let spd = 16.0 * micros as f32 / 16000.0;
            cache.pos.x += cache.del.x * spd;
            cache.pos.y += cache.del.y * spd;

            if (0.0 > cache.del.y && 0.0 > cache.pos.y) || (0.0 < cache.del.y && 200.0 - 14.0 < cache.pos.y) {
                cache.del.y = -cache.del.y;
            }

            if (0.0 > cache.del.x && 0.0 > cache.pos.x) || (0.0 < cache.del.x && 320.0 - w < cache.pos.x) {
                cache.del.x = -cache.del.x;
            }
            
            world.entity_set_position(cache.id, cache.pos);
        }
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
