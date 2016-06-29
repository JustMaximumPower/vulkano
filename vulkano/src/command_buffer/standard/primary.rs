// Copyright (c) 2016 The vulkano developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

use std::sync::Arc;

use command_buffer::pool::CommandPool;
use command_buffer::pool::StandardCommandPool;
use command_buffer::standard::StdCommandBuffer;
use command_buffer::standard::StdCommandBufferBuilder;
use command_buffer::standard::StdPipelineBarrierDest;
use command_buffer::sys::Flags;
use command_buffer::sys::Kind;
use command_buffer::sys::UnsafeCommandBuffer;
use command_buffer::sys::UnsafeCommandBufferBuilder;
use framebuffer::EmptySinglePassRenderPass;

pub struct StdPrimaryCommandBufferBuilder<P = Arc<StandardCommandPool>> where P: CommandPool {
    inner: UnsafeCommandBufferBuilder<P>
}

impl<P> StdPrimaryCommandBufferBuilder<P> where P: CommandPool {
    pub fn new(pool: P) -> StdPrimaryCommandBufferBuilder<P> {
        let kind = Kind::Primary::<EmptySinglePassRenderPass, EmptySinglePassRenderPass>;
        let cb = UnsafeCommandBufferBuilder::new(pool, kind, Flags::SimultaneousUse).unwrap();  // TODO: allow handling this error

        StdPrimaryCommandBufferBuilder {
            inner: cb,
        }
    }
}

unsafe impl<P> StdCommandBufferBuilder for StdPrimaryCommandBufferBuilder<P> where P: CommandPool {
    type BuildOutput = StdPrimaryCommandBuffer<P>;
    type Pool = P;

    #[inline]
    fn add_command<F>(&mut self, cmd: F)
        where F: FnOnce(&mut UnsafeCommandBufferBuilder<P>)
    {
        cmd(&mut self.inner)
    }

    #[inline]
    fn add_barrier(&mut self, barrier: StdPipelineBarrierDest) {
        // FIXME: handle image layout transitions and queue ownership transitions
    }

    #[inline]
    fn build(self) -> StdPrimaryCommandBuffer<P> {
        StdPrimaryCommandBuffer {
            inner: self.inner.build().unwrap(),     // TODO: allow handling this error
        }
    }
}

pub struct StdPrimaryCommandBuffer<P = Arc<StandardCommandPool>> where P: CommandPool {
    inner: UnsafeCommandBuffer<P>
}

unsafe impl<P> StdCommandBuffer for StdPrimaryCommandBuffer<P> where P: CommandPool {
}
