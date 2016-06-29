// Copyright (c) 2016 The vulkano developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

use command_buffer::standard::StdCommandBuffer;
use command_buffer::standard::StdCommandBufferBuilder;
use command_buffer::standard::StdPipelineBarrierDest;
use command_buffer::sys::UnsafeCommandBufferBuilder;

/// Wrapper around a `StdCommandBufferBuilder` that adds a buffer updating command at the end of
/// the builder.
pub struct StdUpdateBufferBuilder<'a, T, D: 'a> {
    inner: T,
    data: &'a D,
    barrier: Option<StdPipelineBarrierDest>,
}

impl<'a, T, D: 'a> StdUpdateBufferBuilder<'a, T, D> where T: StdCommandBufferBuilder {
    /// Adds the command at the end of `inner`.
    pub fn new(inner: T, data: &'a D) -> StdUpdateBufferBuilder<'a, T, D> {
        // FIXME: safety checks
        StdUpdateBufferBuilder {
            inner: inner,
            data: data,
            barrier: None,      // FIXME: add barrier with the buffer in it
        }
    }

    fn flush(&mut self) {
        if let Some(barrier) = self.barrier.take() {
            self.inner.add_barrier(barrier);
            self.inner.add_command(|cb| unimplemented!());
        }
    }
}

unsafe impl<'a, T, D: 'a> StdCommandBufferBuilder for StdUpdateBufferBuilder<'a, T, D>
    where T: StdCommandBufferBuilder
{
    type BuildOutput = StdUpdateBuffer<T::BuildOutput>;
    type Pool = T::Pool;

    #[inline]
    fn add_command<F>(&mut self, cmd: F)
        where F: FnOnce(&mut UnsafeCommandBufferBuilder<T::Pool>)
    {
        self.flush();
        self.inner.add_command(cmd);
    }

    fn add_barrier(&mut self, barrier: StdPipelineBarrierDest) {
        if let Some(curr_barrier) = self.barrier.take() {
            match curr_barrier.try_merge(barrier) {
                Ok(b) => self.barrier = Some(b),
                Err((old, new)) => {
                    self.barrier = Some(old);
                    self.flush();
                    self.inner.add_barrier(new);
                }
            }
        } else {
            self.inner.add_barrier(barrier);
        }
    }

    #[inline]
    fn build(mut self) -> StdUpdateBuffer<T::BuildOutput> {
        self.flush();

        StdUpdateBuffer {
            inner: self.inner.build(),
        }
    }
}

/// Wrapper around a `StdUpdateBuffer` that adds a buffer updating command at the end of the
/// command buffer.
pub struct StdUpdateBuffer<T> {
    inner: T,
    // TODO: store buffer ptr here
}

unsafe impl<T> StdCommandBuffer for StdUpdateBuffer<T> where T: StdCommandBuffer {
}
