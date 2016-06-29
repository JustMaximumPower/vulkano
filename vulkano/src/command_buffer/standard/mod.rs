// Copyright (c) 2016 The vulkano developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

//! Standard implementation of the `CommandBuffer` trait.
//! 
//! Everything in this module is dedicated to the "standard" implementation of command buffers.

use command_buffer::pool::CommandPool;
use command_buffer::sys::UnsafeCommandBufferBuilder;

use self::update_buffer::StdUpdateBufferBuilder;

pub use self::barrier::StdPipelineBarrierDest;

mod barrier;

pub mod primary;
pub mod update_buffer;

///
///
/// # How to use
///
/// In order to successfully add a command to a builder that implements this trait, you must:
///
/// - Add a pipeline barrier if necessary.   TODO: clarify
/// - Call `add_command` to add your command to the underlying unsafe builder.
/// - Somehow keep the objects alive for as long as the command buffer is alive, and properly
///   handle CPU-GPU and inter-queue synchronization.
///
/// # How to implement
///
/// Implementing this trait on a simple wrapper around a builder is very straight-forward. Just
/// don't forget to add an initial and a final pipeline barrier if needed.
///
/// Implementing this trait on a wrapper that itself adds a command in the process is also
/// straight-forward, with the additional note that you should keep the objects alive and provide
/// a `BuildOutput` object that correctly handles synchronization.
///
/// However for performance reasons you may not want to implement this trait on a straight-forward
/// way. Instead of directly adding the new barrier and the new command as soon as the wrapper is
/// created, it should keep them in memory instead. When `add_barrier` is called, it should try
/// merge the new barrier with the existing one. If the barriers can be merged, do nothing more.
/// If the barriers can't be merged, call `add_barrier` on the wrapper object with the old barrier
/// followed with `add_command`, and stop filtering calls to `add_barrier` altogether. When
/// `add_command` or `build` is called, flush your own barrier and command immediately.
///
pub unsafe trait StdCommandBufferBuilder {
    /// The finished command buffer.
    type BuildOutput: StdCommandBuffer;

    /// The command pool that was used to build the command buffer.
    type Pool: CommandPool;

    /// Adds a buffer update command at the end of the command buffer builder.
    #[inline]
    fn update_buffer<'a, D: 'a>(self, data: &'a D) -> StdUpdateBufferBuilder<'a, Self, D>
        where Self: Sized
    {
        StdUpdateBufferBuilder::new(self, data)
    }

    /// Obtains a temporary access to the command buffer builder in order to add one or multiple
    /// commands to it.
    ///
    /// The implementation **must** call the closure with a correct reference to the builder.
    /// Failure to do so is unsound.
    ///
    /// For performance reasons, you are encouraged to use `add_barrier` in order to add a pipeline
    /// barrier, but adding a barrier through `add_command` is not forbidden.
    fn add_command<F>(&mut self, cmd: F)
        where F: FnOnce(&mut UnsafeCommandBufferBuilder<Self::Pool>);

    /// Adds a pipeline barrier to the command buffer.
    fn add_barrier(&mut self, barrier: StdPipelineBarrierDest);

    /// Finishes building the command buffer.
    ///
    /// Consumes the builder and returns an implementation of `StdCommandBuffer`.
    fn build(self) -> Self::BuildOutput;
}

pub unsafe trait StdCommandBuffer/*: CommandBuffer*/ {
}
