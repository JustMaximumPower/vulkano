// Copyright (c) 2016 The vulkano developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

use command_buffer::pool::CommandPool;
use command_buffer::sys::UnsafeCommandBufferBuilder;
use sync::AccessFlagBits;
use sync::PipelineStages;

/// Prototype for a pipeline barrier.
pub struct StdPipelineBarrierDest {

}

impl StdPipelineBarrierDest {
    pub fn empty() -> StdPipelineBarrierDest {
        unimplemented!()
    }

    pub fn try_merge(self, other: StdPipelineBarrierDest)
                     -> Result<StdPipelineBarrierDest,
                               (StdPipelineBarrierDest, StdPipelineBarrierDest)>
    {
        unimplemented!()
    }

    pub fn submit<P>(self, src_stages: PipelineStages, src_accesses: AccessFlagBits,
                     dest: &mut UnsafeCommandBufferBuilder<P>)
        where P: CommandPool
    {
        unimplemented!()
    }
}
