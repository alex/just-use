#![no_std]

use linux_kernel_module::{self, cstr};

struct RandomFile;

impl RandomFile {
    fn read(
        &self,
        file: &linux_kernel_module::file_operations::File,
        buf: &mut linux_kernel_module::user_ptr::UserSlicePtrWriter,
        _offset: u64,
    ) -> linux_kernel_module::KernelResult<()> {
        let mut chunkbuf = [0; 256];
        while !buf.is_empty() {
            let len = chunkbuf.len().min(buf.len());
            let chunk = &mut chunkbuf[0..len];
            if file
                .flags()
                .contains(linux_kernel_module::file_operations::FileFlags::NONBLOCK)
            {
                linux_kernel_module::random::getrandom_nonblock(chunk)?;
            } else {
                linux_kernel_module::random::getrandom(chunk)?;
            }
            buf.write(chunk)?;
        }
        Ok(())
    }

    fn write(
        &self,
        buf: &mut linux_kernel_module::user_ptr::UserSlicePtrReader,
        _offset: u64,
    ) -> linux_kernel_module::KernelResult<()> {
        let mut chunkbuf = [0; 256];
        while !buf.is_empty() {
            let len = chunkbuf.len().min(buf.len());
            let chunk = &mut chunkbuf[0..len];
            buf.read(chunk)?;
            linux_kernel_module::random::add_randomness(chunk);
        }
        Ok(())
    }
}

impl linux_kernel_module::file_operations::FileOperations for RandomFile {
    fn open() -> linux_kernel_module::KernelResult<Self> {
        Ok(RandomFile)
    }

    const READ: linux_kernel_module::file_operations::ReadFn<Self> = Some(Self::read);
    const WRITE: linux_kernel_module::file_operations::WriteFn<Self> = Some(Self::write);
    // TODO:
    // - const POLL (check if Read will be non-blocking)
}

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
