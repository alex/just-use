obj-m := justuse.o
justuse-objs := just_use.rust.o

CARGO ?= cargo

$(src)/target/x86_64-linux-kernel/release/libjust_use.a: $(src)/Cargo.toml $(wildcard $(src)/src/*.rs)
	cd $(src); env -u MAKE -u MAKEFLAGS $(CARGO) build -Z build-std=core,alloc --target=x86_64-linux-kernel --release

%.rust.o: target/x86_64-linux-kernel/release/lib%.a
	$(LD) -r -o $@ --whole-archive $<
