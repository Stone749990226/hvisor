// Copyright (c) 2025 Syswonder
// hvisor is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan PSL v2.
// You may obtain a copy of Mulan PSL v2 at:
//     http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND, EITHER
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT, MERCHANTABILITY OR
// FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.
//
// Syswonder Website:
//      https://www.syswonder.org
//
// Authors:
//

use crate::{
    arch::{
        mmu::MemoryType,
        zone::{GicConfig, Gicv3Config, HvArchZoneConfig},
    },
    config::*,
};

pub const BOARD_NAME: &str = "dayu200-rk3568";

pub const BOARD_NCPUS: usize = 4;
pub const BOARD_UART_BASE: u64 = 0xfe660000;

#[rustfmt::skip]
pub static BOARD_MPIDR_MAPPINGS: [u64; BOARD_NCPUS] = [
    0x0,     // cpu0
    0x100,   // cpu1
    0x200,   // cpu2
    0x300,   // cpu3
];

/// The physical memory layout of the board.
/// Each address should align to 2M (0x200000).
/// Addresses must be in ascending order.
#[rustfmt::skip]
pub const BOARD_PHYSMEM_LIST: &[(u64, u64, MemoryType)] = &[
 // (       start,           end,                type)
    (  0x0,          0xf0_000_000,   MemoryType::Normal),
    (  0xf0_000_000,   0x100000000,  MemoryType::Device),
];
pub const ROOT_ZONE_DTB_ADDR: u64 = 0x60000000;
pub const ROOT_ZONE_KERNEL_ADDR: u64 = 0x61000000;
pub const ROOT_ZONE_ENTRY: u64 = 0x61000000;
//pub const ROOT_ZONE_CPUS: u64 = (1 << 0) ;
pub const ROOT_ZONE_CPUS: u64 = (1 << 0) | (1 << 1);

pub const ROOT_ZONE_NAME: &str = "root-linux";
pub const ROOT_ZONE_MEMORY_REGIONS: [HvConfigMemoryRegion; 6] = [
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0x100000,
        virtual_start: 0x100000,
        size: 0x10000,
    },
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xF0000000,
        virtual_start: 0xF0000000,
        size: 0xF000000,
    },
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0x3c0800000,
        virtual_start: 0x3c0800000,
        size: 0x400000,
    }, // pcie@fe280000
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_RAM,
        physical_start: 0x200000,
        virtual_start: 0x200000,
        size: 0x8200000,
    }, // memory
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_RAM,
        physical_start: 0x9400000,
        virtual_start: 0x9400000,
        size: 0x76C00000,
    }, // memory
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_RAM,
        physical_start: 0x110000,
        virtual_start: 0x110000,
        size: 0xf0000,
    }, // memory
];

// pub const ROOT_ZONE_MEMORY_REGIONS: [HvConfigMemoryRegion; 18] = [
//     HvConfigMemoryRegion {
//         mem_type: MEM_TYPE_IO,
//         physical_start: 0xfe600000,
//         virtual_start: 0xfe600000,
//         size: 0x1000,
//     }, //watchdog@fe600000
//     HvConfigMemoryRegion {
//         mem_type: MEM_TYPE_IO,
//         physical_start: 0xfe8c0000,
//         virtual_start: 0xfe8c0000,
//         size: 0x20000,
//     }, //phy@fe8c0000
//     HvConfigMemoryRegion {
//         mem_type: MEM_TYPE_IO,
//         physical_start: 0xfe8a0000,
//         virtual_start: 0xfe8a0000,
//         size: 0x10000,
//     }, //usb2-phy@fe8a0000
//     HvConfigMemoryRegion {
//         mem_type: MEM_TYPE_IO,
//         physical_start: 0xfdd60000,
//         virtual_start: 0xfdd60000,
//         size: 0x1000,
//     }, //gpio0@fdd60000
//     HvConfigMemoryRegion {
//         mem_type: MEM_TYPE_IO,
//         physical_start: 0xfd904000,
//         virtual_start: 0xfd904000,
//         size: 0x4000,
//     }, //debug@fd904000
//     HvConfigMemoryRegion {
//         mem_type: MEM_TYPE_IO,
//         physical_start: 0xfe0a0000,
//         virtual_start: 0xfe0a0000,
//         size: 0x20000,
//     }, //hdmi
//     // HvConfigMemoryRegion {
//     //     mem_type: MEM_TYPE_IO,
//     //     physical_start: 0xfdff1a00,
//     //     virtual_start: 0xfdff1a00,
//     //     size: 0x1000,
//     // }, //iommu@fdff1a00
//     HvConfigMemoryRegion {
//         mem_type: MEM_TYPE_IO,
//         physical_start: 0xfdc60000,
//         virtual_start: 0xfdc60000,
//         size: 0x10000,
//     }, //
//     HvConfigMemoryRegion {
//         mem_type: MEM_TYPE_RAM,
//         physical_start: 0x200000,
//         virtual_start: 0x200000,
//         size: 0x8200000,
//     }, // memory
//     HvConfigMemoryRegion {
//         mem_type: MEM_TYPE_RAM,
//         physical_start: 0x9400000,
//         virtual_start: 0x9400000,
//         size: 0x76C00000,
//     }, // memory
//     HvConfigMemoryRegion {
//         mem_type: MEM_TYPE_RAM,
//         physical_start: 0x110000,
//         virtual_start: 0x110000,
//         size: 0xf0000,
//     }, // memory
//     HvConfigMemoryRegion {
//         mem_type: MEM_TYPE_IO,
//         physical_start: 0x10f000,
//         virtual_start: 0x10f000,
//         size: 0x1000,
//     }, //scmi-shmem
//     HvConfigMemoryRegion {
//         mem_type: MEM_TYPE_IO,
//         physical_start: 0xFE660000,
//         virtual_start: 0xFE660000,
//         size: 0x10000,
//     }, //serial
//     HvConfigMemoryRegion {
//         mem_type: MEM_TYPE_IO,
//         physical_start: 0xFDC20000,
//         virtual_start: 0xFDC20000,
//         size: 0x10000,
//     }, //syscon
//     HvConfigMemoryRegion {
//         mem_type: MEM_TYPE_IO,
//         physical_start: 0xFDD20000,
//         virtual_start: 0xFDD20000,
//         size: 0x10000,
//     }, //clock-controller
//     HvConfigMemoryRegion {
//         mem_type: MEM_TYPE_IO,
//         physical_start: 0xFDD00000,
//         virtual_start: 0xFDD00000,
//         size: 0x10000,
//     }, //clock-controller
//     HvConfigMemoryRegion {
//         mem_type: MEM_TYPE_IO,
//         physical_start: 0xFE310000,
//         virtual_start: 0xFE310000,
//         size: 0x10000,
//     }, //sdhci
//     HvConfigMemoryRegion {
//         mem_type: MEM_TYPE_IO,
//         physical_start: 0xFE5f0000,
//         virtual_start: 0xFE5f0000,
//         size: 0x10000,
//     }, //timer
//     HvConfigMemoryRegion {
//         mem_type: MEM_TYPE_IO,
//         physical_start: 0xfe670000,
//         virtual_start: 0xfe670000,
//         size: 0x10000,
//     }, //serial
//     HvConfigMemoryRegion {
//         mem_type: MEM_TYPE_IO,
//         physical_start: 0xfe680000,
//         virtual_start: 0xfe680000,
//         size: 0x10000,
//     }, //serial
//     HvConfigMemoryRegion {
//         mem_type: MEM_TYPE_IO,
//         physical_start: 0xfe6c0000,
//         virtual_start: 0xfe6c0000,
//         size: 0x10000,
//     }, //serial
//     HvConfigMemoryRegion {
//         mem_type: MEM_TYPE_IO,
//         physical_start: 0xfe440000,
//         virtual_start: 0xfe440000,
//         size: 0x20000,
//     }, //serial
// ];

pub const ROOT_ZONE_IRQS_BITMAP: &[BitmapWord] = &get_irqs_bitmap(&[
    0x21, 0x22, 0x23, 0x24, 0x28, 0x29, 0x2a, 0x2b, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x38,
    0x3b, 0x3d, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x47, 0x48, 0x49, 0x4d, 0x4e, 0x4f, 0x50, 0x51,
    0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x64,
    0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79,
    0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89,
    0x8a, 0x8d, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0xa2, 0xa3, 0xa5,
    0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xb2, 0xb4, 0xb5, 0xb7, 0xbc, 0xbd, 0xbe,
    0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc9, 0xca, 0xcd, 0xcf, 0xd2, 0xd7, 0xd8, 0xd9, 0xda,
    0x104, 0x105, 0x106, 0x107, 0x11c,
]);

pub const ROOT_ARCH_ZONE_CONFIG: HvArchZoneConfig = HvArchZoneConfig {
    is_aarch32: 0,
    gic_config: GicConfig::Gicv3(Gicv3Config {
        gicd_base: 0xfd400000,
        gicd_size: 0x10000,
        gicr_base: 0xfd460000,
        gicr_size: 0xc0000,
        gits_base: 0,
        gits_size: 0,
    }),
};

pub const ROOT_ZONE_IVC_CONFIG: [HvIvcConfig; 0] = [];

pub const ROOT_PCI_DEVS: [u64; 0] = [];
