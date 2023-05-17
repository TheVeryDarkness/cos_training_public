#![no_std]
#![feature(asm_const)]

#[cfg(all(feature = "enable", feature = "disable"))]
compile_error!();

#[cfg(not(any(feature = "enable", feature = "disable")))]
compile_error!();

#[cfg(feature = "enable")]
use riscv::register::satp;

const KERNEL_BASE_PAGE: usize = 0x_ffff_ffff_c000_0___;
pub const KERNEL_BASE: usize = KERNEL_BASE_PAGE << BITS_PAGE_OFFSET;

const VIRTUAL_BASE_OFFSET_PAGE: usize = 0xffff_ffc0_0000_0___;
const PHYS_VIRT_OFFSET: usize = VIRTUAL_BASE_OFFSET_PAGE << BITS_PAGE_OFFSET;

/// Length in bits of bytes offset in the page
// 4096 bytes per page
const BITS_PAGE_OFFSET: usize = 12;

/// Length in bits of each level of page index.
// 3 levels in SV39, 4 levels in SV48
const BITS_PAGE_INDEX: usize = 9;

/// Length in bits of page table entries flags.
// _ _ Dirty Accessed Global User eXecute Write Read Valid
const BITS_PTE_FLAGS: usize = 10;

/// Length in bits of each page table entry.
const BITS_PTE: usize = 64;

// Count of page table entries in each page of page table
const COUNT_PTE: usize = ((1 << BITS_PAGE_OFFSET) * 8) / BITS_PTE;

#[cfg(feature = "enable")]
#[link_section = ".data.boot_page_table"]
// 64 bits per entry
static mut BOOT_PT_SV39: [u64; COUNT_PTE] = [0; COUNT_PTE];

#[cfg(feature = "enable")]
const fn full_page_index_sv39(i1: u64, i2: u64, i3: u64) -> u64 {
    (i1 << BITS_PAGE_INDEX | i2) << BITS_PAGE_INDEX | i3
}

#[cfg(feature = "enable")]
pub unsafe fn pre_mmu() {
    // Set Page Table Entries
    const FLAG: u64 = 0b11101111;
    const PTE: u64 = full_page_index_sv39(2, 0, 0) << BITS_PTE_FLAGS | FLAG;
    const _PTE: u64 = (0x80000 << 10) | 0xef;
    // 0x8000_0000..0xc000_0000
    // VRWX_GAD
    // 1G block
    BOOT_PT_SV39[2] = PTE;
    // 0xffff_ffc0_8000_0000..0xffff_ffc0_c000_0000, VRWX_GAD, 1G block
    BOOT_PT_SV39[0x102] = PTE;

    // 0xffff_ffff_c000_0000..highest, VRWX_GAD, 1G block
    BOOT_PT_SV39[0x1ff] = PTE;
}

#[cfg(feature = "enable")]
pub unsafe fn enable_mmu() {
    let page_table_root: usize = BOOT_PT_SV39.as_ptr() as usize;
    let page_table_root_page: usize = page_table_root >> BITS_PAGE_OFFSET;
    satp::set(satp::Mode::Sv39, 0, page_table_root_page);
    riscv::asm::sfence_vma_all();
}

#[cfg(feature = "enable")]
pub unsafe fn post_mmu() {
    core::arch::asm!("
        li      t0, {phys_virt_offset}  // fix up virtual high address
        add     sp, sp, t0              // convert stack pointer to virtual address
        add     ra, ra, t0              // convert return address to virtual address
        ret     ",
        phys_virt_offset = const PHYS_VIRT_OFFSET,
    )
}

#[cfg(not(feature = "enable"))]
pub fn pre_mmu() {}
#[cfg(not(feature = "enable"))]
pub fn enable_mmu() {}
#[cfg(not(feature = "enable"))]
pub fn post_mmu() {}
