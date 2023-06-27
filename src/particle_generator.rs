use std::{ffi::c_void, rc::Rc};

use gl::types::*;

use nalgebra_glm as glm;
use rand::prelude::*;

use crate::{
    game_object::GameObject,
    graphics::{shader::Shader, texture::Texture2D},
};

#[derive(Debug, Default)]
pub struct Particle {
    position: glm::Vec2,
    velocity: glm::Vec2,
    color: glm::Vec4,
    life: f32,
}

impl Particle {
    pub fn new() -> Self {
        Self {
            position: glm::vec2(0.0, 0.0),
            velocity: glm::vec2(0.0, 0.0),
            color: glm::vec4(1.0, 1.0, 1.0, 1.0),
            life: 0.0,
        }
    }
}

#[derive(Debug)]
pub struct ParticleGenerator {
    particles: Vec<Particle>,
    amount: u32,
    shader: Rc<Shader>,
    texture: Texture2D,
    vao: u32,
    last_used_particle: usize,
    rand: ThreadRng,
}

impl ParticleGenerator {
    pub fn new(shader: Rc<Shader>, texture: Texture2D, amount: u32) -> Self {
        Self {
            particles: Vec::new(),
            amount,
            shader,
            texture,
            vao: 0,
            last_used_particle: 0,
            rand: rand::thread_rng(),
        }
    }

    pub fn update(&mut self, dt: f32, object: &GameObject, new_particles: u32, offset: glm::Vec2) {
        // add new particles
        for _ in 0..new_particles {
            let unused_particle = Self::first_unused_particle(
                &mut self.last_used_particle,
                self.amount,
                &self.particles,
            );

            if let Some(particle) = self.particles.get_mut(unused_particle) {
                Self::respawn_particle(particle, object, offset);
            }
        }

        // update all particles
        for i in 0..self.amount {
            if let Some(particle) = self.particles.get_mut(i as usize) {
                particle.life -= dt;
                if particle.life > 0.0 {
                    // particle is alive, thus update
                    particle.position -= particle.velocity * dt;
                    particle.color.w -= dt * 2.5;
                }
            }
        }
    }

    pub fn draw(&self) {
        // use additive blending to give it a 'glow' effect
        unsafe {
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE);
            self.shader.use_program();
            for particle in &self.particles {
                if particle.life > 0.0 {
                    self.shader.set_vec2("offset\0", &particle.position);
                    self.shader.set_vec4("color\0", &particle.color);
                    self.texture.bind();
                    gl::BindVertexArray(self.vao);
                    gl::DrawArrays(gl::TRIANGLES, 0, 6);
                    gl::BindVertexArray(0);
                }
            }
            // don't forget to reset to default blending mode
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }
    }

    pub fn init(&mut self) {
        // set up mesh and attribute properties
        let vbo = 0;
        #[rustfmt::skip]
        let particle_quad: [f32; 24] = [
            0.0, 1.0, 0.0, 1.0,
            1.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 1.0,
            1.0, 1.0, 1.0, 1.0,
            1.0, 0.0, 1.0, 0.0
        ];

        unsafe {
            gl::GenVertexArrays(1, self.vao as *mut u32);
            gl::GenBuffers(1, vbo as *mut u32);
            gl::BindVertexArray(self.vao);

            // fill mesh buffer
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                std::mem::size_of_val(&particle_quad) as GLsizeiptr,
                &particle_quad[0] as *const f32 as *const c_void,
                gl::STATIC_DRAW,
            );

            // set mesh attributes
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                4,
                gl::FLOAT,
                gl::FALSE,
                4 * std::mem::size_of::<f32>() as GLsizei,
                std::ptr::null(),
            );
            gl::BindVertexArray(0);

            for _ in 0..self.amount {
                self.particles.push(Particle::new());
            }
        }
    }

    fn first_unused_particle(
        last_used_particle: &mut usize,
        amount: u32,
        particles: &Vec<Particle>,
    ) -> usize {
        // first search from last used particle, this will usually return almost instantly
        for i in *last_used_particle..amount as usize {
            if let Some(particle) = particles.get(i) {
                if particle.life <= 0.0 {
                    *last_used_particle = i;
                    return i;
                }
            }
        }

        // otherwise, do a linear search
        for i in 0..*last_used_particle {
            if particles.get(i).unwrap().life <= 0.0 {
                *last_used_particle = i;
                return i;
            }
        }
        // all particles are taken, override the first one
        *last_used_particle = 0;
        0
    }

    fn respawn_particle(particle: &mut Particle, object: &GameObject, offset: glm::Vec2) {
        let random_val = ((random::<i32>() % 100) - 50) as f32 / 10.0;
        let random_color = 0.5 + ((random::<u32>() % 100) as f32 / 100.0);

        particle.position.x = object.position.x + random_val + offset.x;
        particle.position.y = object.position.y + random_val + offset.y;

        particle.color = glm::vec4(random_color, random_color, random_color, 1.0);
        particle.life = 1.0;
        particle.velocity = object.velocity * 0.1;
    }
}
