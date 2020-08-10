#![no_std]

use linux_kernel_module::{self, cstr};

struct RandomFile;

impl linux_kernel_module::file_operations::FileOperations for RandomFile {
    const VTABLE: linux_kernel_module::file_operations::FileOperationsVtable =
        linux_kernel_module::file_operations::FileOperationsVtable::builder::<Self>()
            .read()
            .write()
            .build();

    fn open() -> linux_kernel_module::KernelResult<Self> {
        Ok(RandomFile)
    }
}

impl linux_kernel_module::file_operations::Read for RandomFile {
    fn read(
        &self,
        file: &linux_kernel_module::file_operations::File,
        buf: &mut linux_kernel_module::user_ptr::UserSlicePtrWriter,
        _offset: u64,
    ) -> linux_kernel_module::KernelResult<()> {
        const TMP_BUF_LEN: usize = 256;
        while !buf.is_empty() {
            let mut tmp_buf = [0; TMP_BUF_LEN];
            if file
                .flags()
                .contains(linux_kernel_module::file_operations::FileFlags::NONBLOCK)
            {
                linux_kernel_module::random::getrandom_nonblock(
                    &mut tmp_buf[..TMP_BUF_LEN.min(buf.len())],
                )?;
            } else {
                linux_kernel_module::random::getrandom(&mut tmp_buf[..TMP_BUF_LEN.min(buf.len())])?;
            }
            buf.write(&mut tmp_buf[..TMP_BUF_LEN.min(buf.len())])?;
        }
        Ok(())
    }
}

impl linux_kernel_module::file_operations::Write for RandomFile {
    fn write(
        &self,
        buf: &mut linux_kernel_module::user_ptr::UserSlicePtrReader,
        _offset: u64,
    ) -> linux_kernel_module::KernelResult<()> {
        const TMP_BUF_LEN: usize = 256;
        while !buf.is_empty() {
            let mut tmp_buf = [0; TMP_BUF_LEN];
            buf.read(&mut tmp_buf[..TMP_BUF_LEN.min(buf.len())])?;
            linux_kernel_module::random::add_randomness(&tmp_buf[..TMP_BUF_LEN.min(buf.len())]);
        }
        Ok(())
    }
}

// TODO:
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
    author: b"Alex Gaynor <alex.gaynor@gmail.com>",
    description: b"Just use /dev/urandom: Now with early-boot safety",
    license: b"GPL"
);
