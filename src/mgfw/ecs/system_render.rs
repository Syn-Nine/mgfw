use super::*;

struct RenderSystemData {
    vao_pri: u32,
    vao_alt: u32, // not yet used
    vbo_pri: u32,
    vbo_alt: u32, // not yet used
}

pub struct RenderSystem {
    data: *mut RenderSystemData,
    // WARNING: Anything below this line is not in cache!
    frame: usize,
}

#[allow(dead_code)]
impl RenderSystem {
    pub fn new(mgr: &mut CacheManager, gl: &Gl) -> RenderSystem {
        println!("Constructing RenderSystem");
        // allocate system memory in cache
        let sz_bytes = std::mem::size_of::<RenderSystemData>() * ENTITY_SZ;
        let data = mgr.allocate(sz_bytes) as *mut RenderSystemData;

        let ret = RenderSystem { data, frame: 0 };

        // pre-generate a VAO/VBO for each entity
        for i in 0..ENTITY_SZ {
            let d = ret.get_data_ref_mut(i);
            d.vao_pri = gl.gen_vao();
            d.vao_alt = gl.gen_vao();
            d.vbo_pri = gl.gen_vbo();
            d.vbo_alt = gl.gen_vbo();
        }

        ret
    }

    fn skip_entity(&self, idx: usize, world: &World) -> bool {
        let ent = world.get_entities();
        let rcm = world.get_manager_render();
        if !ent.is_active(idx)
            || !ent.is_visible(idx)
            || !ent.has_component(idx, COMPONENT_RENDER)
            || rcm.has_type(idx, RENDER_TYPE_INVALID)
        {
            return true;
        }
        false
    }

    pub fn update(&mut self, gl: &Gl, world: &mut World) -> bool {
        let mut expect_blown = false;

        let span = world.get_entities().get_id_span();
        for i in span.first..=span.last {
            if self.skip_entity(i, world) {
                continue;
            }

            // Amortize workload
            match self.frame % 2 {
                // priority 1
                0 => (),

                // priority 2
                1 => {
                    expect_blown |= self.update_vbo(i, gl, world);
                }
                _ => (),
            }
        }
        self.frame += 1;
        expect_blown
    }

    // WARNING: Expect Blown
    fn update_vbo(&self, idx: usize, gl: &Gl, world: &mut World) -> bool {
        let mut expect_blown = false;

        // Update Text VBOs
        match world.get_manager_render().get_type(idx) {
            RENDER_TYPE_LINE_BUFFER => {
                if world.get_manager_line().reconstruct(idx) {
                    let dat = self.get_data_ref(idx);
                    world.line_buffer_construct(idx, gl, dat.vao_pri, dat.vbo_pri);
                    expect_blown = true;
                }
            }
            RENDER_TYPE_TRIANGLE_BUFFER => {
                if world.get_manager_triangle().reconstruct(idx) {
                    let dat = self.get_data_ref(idx);
                    world.triangle_buffer_construct(idx, gl, dat.vao_pri, dat.vbo_pri);
                    expect_blown = true;
                }
            }
            RENDER_TYPE_TEXT => {
                if world.get_manager_text().reconstruct(idx) {
                    let dat = self.get_data_ref(idx);
                    world.text_construct(idx, gl, dat.vao_pri, dat.vbo_pri);
                    expect_blown = true;
                }
            }
            _ => (),
        }
        expect_blown
    }

    pub fn render(&self, gl: &Gl, world: &World, start_time: std::time::Instant) {
        let pcm = world.get_manager_position();
        let phcm = world.get_manager_physics();
        let rcm = world.get_manager_render();
        let tcm = world.get_manager_text();
        let lcm = world.get_manager_line();
        let trm = world.get_manager_triangle();

        let span = world.get_entities().get_id_span();
        for i in span.first..=span.last {
            if self.skip_entity(i, world) {
                continue;
            }

            let dt = std::time::Instant::now()
                .duration_since(start_time)
                .as_micros() as f32
                * 1.0e-6;

            match rcm.get_type(i) {
                RENDER_TYPE_LINE_BUFFER => {
                    if lcm.is_constructed(i) {
                        let vao = self.get_data_ref(i).vao_pri;
                        let pos = pcm.get_data_ref(i);
                        let phys = phcm.get_data_ref(i);
                        gl.draw_lines(
                            pos.x + phys.velocity.x * dt,
                            pos.y + phys.velocity.y * dt,
                            vao,
                            lcm.get_num_lines(i),
                        );
                    }
                }
                RENDER_TYPE_TRIANGLE_BUFFER => {
                    if trm.is_constructed(i) {
                        let vao = self.get_data_ref(i).vao_pri;
                        let pos = pcm.get_data_ref(i);
                        let phys = phcm.get_data_ref(i);
                        gl.draw_triangles(
                            pos.x + phys.velocity.x * dt,
                            pos.y + phys.velocity.y * dt,
                            vao,
                            trm.get_num_triangles(i),
                        );
                    }
                }
                RENDER_TYPE_TEXT => {
                    if tcm.is_constructed(i) {
                        let vao = self.get_data_ref(i).vao_pri;
                        let pos = pcm.get_data_ref(i);
                        let phys = phcm.get_data_ref(i);
                        gl.draw_text(
                            pos.x + phys.velocity.x * dt,
                            pos.y + phys.velocity.y * dt,
                            vao,
                            tcm.get_length(i),
                        );
                    }
                }
                _ => (),
            }
        }
    }

    fn get_data_ref_mut(&self, idx: usize) -> &mut RenderSystemData {
        assert!(idx < ENTITY_SZ);
        unsafe { &mut *(self.data.offset(idx as isize)) }
    }

    fn get_data_ref(&self, idx: usize) -> &RenderSystemData {
        assert!(idx < ENTITY_SZ);
        unsafe { &*(self.data.offset(idx as isize)) }
    }
}
