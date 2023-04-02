///////////////////////////////////////////////////////////////////////////////
//
//  Copyright 2018-2021 Robonomics Network <research@robonomics.network>
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
//
///////////////////////////////////////////////////////////////////////////////
//! Virtual sinkable devices.
extern crate structopt;
use clap;
use clap::Parser;

use crate::error::Result;

#[derive(Debug, Parser)]
pub struct PubsubCmd {
    /// Pubsub operation to run.
    #[clap(subcommand)]
    pub subcommand: Option<PubsubSubCmds>,
}

impl PubsubCmd {
    pub fn run(&self) -> Result<()> {
        Ok(())
    }
}

#[derive(Debug, clap::Subcommand)]
pub enum PubsubSubCmds {
    Address(AddressCmd),
    Enable(EnableCmd),
}

#[derive(Debug, Parser)]
pub struct AddressCmd {
    /// Pubsub listen MULTIADDRESS: i.e. /ip4/127.0.0.1/tcp/30400
    #[clap(long, value_parser)]
    pub robonomics_network_listen: Option<String>,
}

impl AddressCmd {
    pub fn run(&self) -> Result<()> {
        let address: String;

        if let Some(x) = &self.robonomics_network_listen {
            address = x.to_string();
        } else {
            address = "/ip4/127.0.0.1/tcp/30400".to_string();
        }
        println!("Address: {address}");
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct EnableCmd {
    #[clap(long, value_parser)]
    pub peer: Option<String>,

    #[clap(long, value_parser)]
    pub addr: Option<String>,
}

impl EnableCmd {
    pub fn run(&self) -> Result<()> {
        Ok(())
    }
}
