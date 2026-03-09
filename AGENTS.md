# CLAUDE.md

This file provides guidance to Claude Code when working with the hvisor hypervisor codebase.

## Project Overview

hvisor is a Type-1 bare-metal hypervisor implemented in Rust, featuring a separation kernel design. It provides efficient hardware resource virtualization and isolation through distinct zones:

- **zone0 (root zone)**: Management zone running a full Linux OS, responsible for managing other zones
- **zoneU (user zone / guest zone)**: User zones running guest operating systems with strict isolation
- **zoneR (real-time zone)**: Real-time zones for time-critical workloads

The hypervisor uses static CPU partitioning, pre-allocated memory, and supports both device passthrough and virtio paravirtualization.

## Current Development Focus: Dayu200 + OpenHarmony

The primary development target is the **platform/aarch64/dayu200** board (Rockchip RK3568-based), running:
- **Root zone (zone0)**: Full OpenHarmony OS with complete device tree
- **Guest zone (zone1)**: Trimmed OpenHarmony OS with reduced device tree

### Key Files for Dayu200 Development

- [platform/aarch64/dayu200/board.rs](platform/aarch64/dayu200/board.rs): Hardware configuration, memory layout, CPU mappings
- [platform/aarch64/dayu200/configs/zone1-ohos.json](platform/aarch64/dayu200/configs/zone1-ohos.json): Guest zone configuration
- [platform/aarch64/dayu200/configs/zone1-ohos-virtio.json](platform/aarch64/dayu200/configs/zone1-ohos-virtio.json): Guest zone with virtio devices
- [platform/aarch64/dayu200/image/dts/](platform/aarch64/dayu200/image/dts/): Device tree files for both zones
  - `zone0.dts`: Root zone device tree
  - `zone1-ohos.dts`: Guest zone device tree (trimmed)
  - `toybrick.dts`: Original board device tree

### Zone Management Workflow

1. **Build hvisor**: `make ARCH=aarch64 BOARD=dayu200`
2. **Boot into root zone**: The hypervisor boots zone0 (OpenHarmony) first
3. **Launch guest zone**: Use [hvisor-tool](hvisor-tool/) from within zone0:
   ```bash
   cd /path/to/configs
   hvisor zone start zone1-ohos.json
   ```

## Architecture

### Directory Structure

```
hvisor/
├── src/                    # Core hypervisor code
│   ├── arch/              # Architecture-specific code (aarch64, riscv64, loongarch64, x86_64)
│   ├── device/            # Device emulation and passthrough
│   ├── hypercall/         # Hypercall interface
│   ├── memory/            # Memory management and MMU
│   ├── pci/               # PCIe support
│   ├── config.rs          # Zone configuration structures
│   ├── zone.rs            # Zone management
│   └── main.rs            # Entry point
├── platform/              # Platform-specific configurations
│   ├── aarch64/
│   │   ├── dayu200/       # **Primary development target**
│   │   ├── qemu-gicv3/
│   │   ├── imx8mp/
│   │   └── ...
│   ├── riscv64/
│   ├── loongarch64/
│   └── x86_64/
├── hvisor-tool/           # Userspace tooling (separate repo, submodule)
│   ├── tools/             # CLI and virtio daemon
│   ├── driver/            # Kernel modules (hvisor.ko, ivc.ko)
│   └── examples/          # Configuration examples
└── Makefile               # Build system
```

### Build System

The build system uses Cargo with platform-specific features:

```bash
# Build for dayu200
make ARCH=aarch64 BOARD=dayu200 MODE=release

# Build with specific features (auto-detected from platform/aarch64/dayu200/cargo/features)
make ARCH=aarch64 BOARD=dayu200 FEATURES="gicv3 uart_16550 dwc_pcie"

# Quick build ID syntax
make BID=aarch64/dayu200
```

**Key Makefile variables:**
- `ARCH`: Target architecture (aarch64, riscv64, loongarch64, x86_64)
- `BOARD`: Board name (dayu200, qemu-gicv3, imx8mp, etc.)
- `BID`: Combined ARCH/BOARD shorthand
- `MODE`: Build mode (debug, release)
- `LOG`: Log level (trace, debug, info, warn, error)
- `FEATURES`: Cargo features (auto-detected from platform config)

### Zone Configuration

Zone configurations are JSON files defining:
- CPU allocation (static partitioning)
- Memory regions (RAM, IO, virtio MMIO)
- Interrupt routing
- Kernel and DTB paths
- Architecture-specific config (GIC, PLIC, APIC)

Example structure (see [zone1-ohos.json](platform/aarch64/dayu200/configs/zone1-ohos.json)):
```json
{
  "zone_id": 1,
  "cpus": [2, 3],
  "memory_regions": [...],
  "interrupts": [...],
  "kernel_filepath": "./zone1-ohos.kernel",
  "dtb_filepath": "./zone1-ohos.dtb",
  "entry_point": "0x50600000",
  "arch_config": {
    "gic_version": "v3",
    "gicd_base": "0xfd400000",
    ...
  }
}
```

### Device Tree Management

For dayu200 OpenHarmony development:
- **Root zone**: Uses full device tree with all hardware
- **Guest zone**: Uses trimmed device tree with only assigned devices
- Device tree trimming is manual - remove nodes for devices not assigned to guest zone
- Ensure memory regions, interrupts, and device nodes match zone configuration

## Development Guidelines

### Code Style

- Follow Rust standard formatting: `cargo fmt`
- Use `#[rustfmt::skip]` for data tables (e.g., memory layouts)
- Architecture-specific code uses `cfg(target_arch = "...")` attributes
- Platform-specific code lives in `platform/$(ARCH)/$(BOARD)/`

### Adding New Platform Support

1. Create directory: `platform/$(ARCH)/$(BOARD)/`
2. Required files:
   - `board.rs`: Board constants and memory layout
   - `linker.ld`: Linker script
   - `platform.mk`: Platform-specific build rules
   - `cargo/features`: Cargo feature flags
   - `image/`: Boot images and device trees
   - `configs/`: Zone configuration files
3. Update platform detection in build system

### Memory Layout Conventions

- All addresses must align to 2MB (0x200000) boundaries
- Memory regions must be in ascending order
- Use `MemoryType::Normal` for RAM, `MemoryType::Device` for MMIO
- Reserve hypervisor memory at boot (typically low memory)

### Interrupt Handling

- Interrupts are statically assigned to zones
- Root zone typically gets all interrupts initially
- Guest zones get specific interrupt numbers for assigned devices
- Interrupt numbers must match device tree and hardware

### Device Passthrough

- Zone0 can access all devices
- ZoneU devices require:
  - Memory region mapping in zone config
  - Interrupt routing in zone config
  - Device tree node in guest DTB
  - Hardware support (IOMMU for DMA devices)

### Virtio Devices

- Virtio devices are emulated by userspace daemon in zone0
- Requires virtio MMIO region in guest zone config
- Supported devices: virtio-blk, virtio-net, virtio-console, virtio-gpu
- See [hvisor-tool/CLAUDE.md](hvisor-tool/CLAUDE.md) for virtio daemon details

## Testing and Debugging

### QEMU Testing

Quick testing on QEMU before hardware deployment:
```bash
make ARCH=aarch64 BOARD=qemu-gicv3
make run  # Launches QEMU with default configuration
```

### Hardware Debugging

- Use UART serial console for early boot debugging
- Enable detailed logging: `make LOG=debug`
- Check zone status: `hvisor zone list` (from zone0)
- Kernel module logs: `dmesg` (from zone0)

### Common Issues

1. **Zone fails to start**: Check memory regions don't overlap, kernel/DTB paths are correct
2. **Device not working in guest**: Verify interrupt routing, memory mapping, and DTB node
3. **Boot hangs**: Check UART base address, entry point, and CPU assignments
4. **Virtio device issues**: See hvisor-tool debugging section

## Important Constants

- `BOARD_NCPUS`: Number of CPUs on board
- `BOARD_UART_BASE`: UART base address for console
- `ROOT_ZONE_DTB_ADDR`: Where to load root zone device tree
- `ROOT_ZONE_KERNEL_ADDR`: Where to load root zone kernel
- `ROOT_ZONE_ENTRY`: Root zone entry point
- `ROOT_ZONE_CPUS`: CPU mask for root zone (bitfield)

## Cross-Architecture Notes

### AArch64 (ARM64)
- GICv2/GICv3 interrupt controllers
- UART drivers: PL011, uart16550, imx-uart, xuartps
- PCIe: Standard ECAM or DesignWare (dwc_pcie feature)
- Dayu200 uses: GICv3, uart16550, dwc_pcie

## Related Repositories

- **hvisor-tool**: [https://github.com/syswonder/hvisor-tool](https://github.com/syswonder/hvisor-tool)
  - Userspace management tools
  - Kernel modules for zone management
  - Virtio device emulation daemon
- **Documentation**: [https://hvisor.syswonder.org/](https://hvisor.syswonder.org/)

