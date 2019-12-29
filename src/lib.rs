#![no_std]

use linux_kernel_module::{self, cstr};

struct RandomFile;

impl linux_kernel_module::file_operations::FileOperations for RandomFile {
    const VTABLE: linux_kernel_module::file_operations::FileOperationsVtable =
        linux_kernel_module::file_operations::FileOperationsVtable::builder::<Self>()
            .read()
            .build();

    fn open() -> linux_kernel_module::KernelResult<Self> {
        Ok(RandomFile)
    }
}

impl linux_kernel_module::file_operations::Read for RandomFile {
    fn read(
        &self,
        buf: &mut linux_kernel_module::user_ptr::UserSlicePtrWriter,
        _offset: u64,
    ) -> linux_kernel_module::KernelResult<()> {
        const TMP_BUF_LEN: usize = 256;
        // TODO: Respect O_NONBLOCK
        while !buf.is_empty() {
            let mut tmp_buf = [0; TMP_BUF_LEN];
            linux_kernel_module::random::getrandom(&mut tmp_buf[..TMP_BUF_LEN.min(buf.len())])?;
            buf.write(&mut tmp_buf[..TMP_BUF_LEN.min(buf.len())])?;
        }
        Ok(())
    }
}

// TODO:
// - impl Write (add entropy to system pool)
// - impl Poll (check if Read will be non-blocking)

struct JustUseModule {
    _chrdev_registration: linux_kernel_module::chrdev::Registration,
}

impl linux_kernel_module::KernelModule for JustUseModule {
    fn init() -> linux_kernel_module::KernelResult<Self> {
        let chrdev_registration = linux_kernel_module::chrdev::builder(cstr!("justuse"), 0..1)?
            .register_device::<RandomFile>()
            .build()?;
        Ok(JustUseModule {
            _chrdev_registration: chrdev_registration,
        })
    }
}

linux_kernel_module::kernel_module!(
    JustUseModule,
    author: "Alex Gaynor <alex.gaynor@gmail.com>",
    description: "Just use /dev/urandom: Now with early-boot safety",
    license: "GPL"
);
