// Copyright 2025-2030 PyFrame Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use anyhow::Result;
use directories::BaseDirs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use tao::dpi::{LogicalPosition, LogicalSize};

pub type Size = LogicalSize<f64>;
pub type Position = LogicalPosition<f64>;