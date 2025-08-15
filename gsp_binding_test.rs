// SPDX-License-Identifier: GPL-2.0

//! Stub module to build GSP bindings
use kernel::prelude::*;

mod r570_144;
use r570_144 as fw;

module! {
    type: GspBindingTest,
    name: "GspBindings",
    authors: ["Alistair Popple"],
    description: "GSP Binding Generation Stub Module",
    license: "GPL v2",
    firmware: [],
}

struct GspBindingTest {}

impl kernel::Module for GspBindingTest {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        // Use a definition as a basic test to see we're buildiing something
        pr_info!("GSP_FW_WPR_META_MAGIC = {}\n", fw::GSP_FW_WPR_META_MAGIC);

        Ok(GspBindingTest {})
    }
}

impl Drop for GspBindingTest {
    fn drop(&mut self) {}
}
