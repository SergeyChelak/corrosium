Responsibilities of the Bootloader

   1. Hardware & Firmware Abstraction (UEFI):
       * Watchdog Management: Disables the UEFI watchdog timer to prevent the system from rebooting during a long boot process.
       * Console Initialization: Negotiates with UEFI to set the maximum supported text resolution for the console.
   2. Display & User Interaction:
       * Splash Screen: Loads and displays a splash image (splash.bmp) to provide visual feedback.
       * Graphics Mode Selection: Queries available GOP (Graphics Output Protocol) modes and allows the user to select a resolution. This is critical for the kernel to know the framebuffer parameters.
   3. Kernel & OS Loading:
       * Filesystem Discovery: Searches for the kernel binary on the UEFI system partition or within a RedoxFS partition.
       * Memory Allocation: Uses UEFI boot services to allocate physical memory pages for the kernel, stack, and environment variables.
       * Binary Loading: Reads the kernel from disk into the allocated memory.
       * Header Parsing: Parses the kernel's specific header (at offset 0x18 for Redox) to extract the entry point.
   4. Environment Preparation:
       * Argument Passing: Prepares a KernelArgs structure containing physical addresses of the kernel, stack, environment, and ACPI tables.
       * Environment Variables: Passes system information (like framebuffer address/width/height and RedoxFS block details) as a string.
       * ACPI Discovery: Locates the ACPI RSDP (Root System Description Pointer) tables to pass to the kernel for hardware discovery (IOAPIC, HPET, etc.).
   5. Memory Management:
       * Memory Mapping: Retrieves the final memory map from UEFI, which tells the kernel which memory is free, reserved, or occupied by ACPI.
       * Paging Initialization: Sets up the initial 4-level page tables (PML4). It typically identity-maps the first 8 GiB and maps the kernel into the higher-half address space.
   6. The Handoff (Final Transition):
       * Exit Boot Services: Calls ExitBootServices, which terminates UEFI's control over the hardware.
       * CPU State Setup: Disables interrupts (cli), enables architecture-specific features (SSE, Long Mode, NX bit), and loads the new CR3 (page tables).
       * Stack Switch: Switches the rsp (stack pointer) to the kernel-specific stack.
       * Jump to Kernel: Executes the kernel entry point, passing the KernelArgs pointer.

  ---

  Order & Sequence of Execution

   1. Entry Point (main.rs): UEFI firmware loads the bootloader and calls the main function.
   2. Early UEFI Setup:
       * Disable Watchdog Timer.
       * Initialize Console Output and set max mode.
   3. Architecture Initialization (arch/main):
       * Load and display the Splash Screen.
       * Mode Selection: Execute select_mode to let the user pick a resolution.
   4. Kernel Loading (inner function):
       * Search for the kernel file on available disks.
       * Allocate pages and read the kernel into memory.
       * Parse the entry point from the kernel header.
   5. Preparation of Resources:
       * Allocate memory for the Kernel Stack.
       * Allocate and copy Environment Variables (framebuffer info, etc.).
       * Scan for ACPI Tables (find_acpi_table_pointers).
   6. Paging Setup (paging_create):
       * Build the page tables (PML4/PDP/PD/PT).
       * Establish identity and kernel mappings.
   7. UEFI Exit:
       * Memory Map: Fetch the final memory map from UEFI.
       * Exit Boot Services: Transition from "Firmware Environment" to "Bare Metal".
   8. Kernel Handoff:
       * Disable Interrupts: Execute cli.
       * Enable Paging: Load CR3 and set paging bits in CR0/CR4.
       * Set Stack: Move the stack pointer to the newly allocated kernel stack.
       * Jump: Call the kernel entry point with the address of KernelArgs.
