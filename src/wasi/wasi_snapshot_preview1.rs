use wasmtime::*;
use wasmtime_wasi::{WasiCtx};
use wasi_common::{Error, Result};
use wiggle::{GuestPtr};

pub struct LunaticWasiCtx(pub WasiCtx);

wiggle::from_witx!({
    witx: ["$CARGO_MANIFEST_DIR/wasmtime/crates/wasi-common/WASI/phases/snapshot/witx/wasi_snapshot_preview1.witx"],
    ctx: LunaticWasiCtx,
    errors: { errno => Error },
});

wasmtime_wiggle::wasmtime_integration!({
    // The wiggle code to integrate with lives here:
    target: crate::wasi::wasi_snapshot_preview1,
    // This must be the same witx document as used above. This should be ensured by
    // the `WASI_ROOT` env variable, which is set in wasi-common's `build.rs`.
    witx: ["$CARGO_MANIFEST_DIR/wasmtime/crates/wasi-common/WASI/phases/snapshot/witx/wasi_snapshot_preview1.witx"],
    // This must be the same ctx type as used for the target:
    ctx: LunaticWasiCtx,
    // This macro will emit a struct to represent the instance,
    // with this name and docs:
    modules: { wasi_snapshot_preview1 =>
        { name: Wasi,
          docs: "An instantiated instance of the wasi exports.

This represents a wasi module which can be used to instantiate other wasm
modules. This structure exports all that various fields of the wasi instance
as fields which can be used to implement your own instantiation logic, if
necessary. Additionally [`Wasi::get_export`] can be used to do name-based
resolution.",
        // Don't use the wiggle generated code to implement proc_exit, we need
        // to hook directly into the runtime there:
        //  function_override: {
        //    proc_exit => wasi_proc_exit
        //  }
        },
    },
    // Error to return when caller module is missing memory export:
    missing_memory: { wasi_common::wasi::types::Errno::Inval },
});

use types::Errno;


impl wiggle::GuestErrorType for Errno {
    fn success() -> Self {
        Self::Success
    }
}

impl types::GuestErrorConversion for LunaticWasiCtx {
    fn into_errno(&self, e: wiggle::GuestError) -> Errno {
        println!("Guest error: {:?}", e);
        e.into()
    }
}

impl types::UserErrorConversion for LunaticWasiCtx {
    fn errno_from_error(&self, e: Error) -> Errno {
        println!("Error: {:?}", e);
        e.into()
    }
}


impl From<Error> for Errno {
    fn from(e: Error) -> Errno {
        match e {
            Error::Guest(e) => e.into(),
            Error::TryFromInt(_) => Errno::Overflow,
            Error::Utf8(_) => Errno::Ilseq,
            Error::UnexpectedIo(_) => Errno::Io,
            Error::GetRandom(_) => Errno::Io,
            Error::TooBig => Errno::TooBig,
            Error::Acces => Errno::Acces,
            Error::Badf => Errno::Badf,
            Error::Busy => Errno::Busy,
            Error::Exist => Errno::Exist,
            Error::Fault => Errno::Fault,
            Error::Fbig => Errno::Fbig,
            Error::Ilseq => Errno::Ilseq,
            Error::Inval => Errno::Inval,
            Error::Io => Errno::Io,
            Error::Isdir => Errno::Isdir,
            Error::Loop => Errno::Loop,
            Error::Mfile => Errno::Mfile,
            Error::Mlink => Errno::Mlink,
            Error::Nametoolong => Errno::Nametoolong,
            Error::Nfile => Errno::Nfile,
            Error::Noent => Errno::Noent,
            Error::Nomem => Errno::Nomem,
            Error::Nospc => Errno::Nospc,
            Error::Notdir => Errno::Notdir,
            Error::Notempty => Errno::Notempty,
            Error::Notsup => Errno::Notsup,
            Error::Overflow => Errno::Overflow,
            Error::Pipe => Errno::Pipe,
            Error::Perm => Errno::Perm,
            Error::Spipe => Errno::Spipe,
            Error::Notcapable => Errno::Notcapable,
        }
    }
}


impl From<wiggle::GuestError> for Errno {
    fn from(err: wiggle::GuestError) -> Self {
        use wiggle::GuestError::*;
        match err {
            InvalidFlagValue { .. } => Self::Inval,
            InvalidEnumValue { .. } => Self::Inval,
            PtrOverflow { .. } => Self::Fault,
            PtrOutOfBounds { .. } => Self::Fault,
            PtrNotAligned { .. } => Self::Inval,
            PtrBorrowed { .. } => Self::Fault,
            InvalidUtf8 { .. } => Self::Ilseq,
            TryFromIntError { .. } => Self::Overflow,
            InFunc { .. } => Self::Inval,
            InDataField { .. } => Self::Inval,
            SliceLengthsDiffer { .. } => Self::Fault,
            BorrowCheckerOutOfHandles { .. } => Self::Fault,
        }
    }
}

// Use trait implemented for `WasiCtx` (wrapped by `LunaticWasiCtx`)
use wasi_common::wasi::wasi_snapshot_preview1::WasiSnapshotPreview1;

impl<'a> wasi_snapshot_preview1::WasiSnapshotPreview1 for LunaticWasiCtx {
    fn args_get<'b>(
        &self,
        argv: &GuestPtr<'b, GuestPtr<'b, u8>>,
        argv_buf: &GuestPtr<'b, u8>,
    ) -> Result<()> {
        self.0.args_get(argv, argv_buf)
    }

    fn args_sizes_get(&self) -> Result<(types::Size, types::Size)> {
        self.0.args_sizes_get()
    }

    fn environ_get<'b>(
        &self,
        environ: &GuestPtr<'b, GuestPtr<'b, u8>>,
        environ_buf: &GuestPtr<'b, u8>,
    ) -> Result<()> {
        self.0.environ_get(environ, environ_buf)
    }

    fn environ_sizes_get(&self) -> Result<(types::Size, types::Size)> {
        self.0.environ_sizes_get()
    }

    fn clock_res_get(&self, id: types::Clockid) -> Result<types::Timestamp> {
        unimplemented!("oops")
    }

    fn clock_time_get(
        &self,
        id: types::Clockid,
        _precision: types::Timestamp,
    ) -> Result<types::Timestamp> {
        unimplemented!("oops")
    }

    fn fd_advise(
        &self,
        fd: types::Fd,
        offset: types::Filesize,
        len: types::Filesize,
        advice: types::Advice,
    ) -> Result<()> {
        unimplemented!("oops")
    }

    fn fd_allocate(
        &self,
        fd: types::Fd,
        offset: types::Filesize,
        len: types::Filesize,
    ) -> Result<()> {
        unimplemented!("oops")
    }

    fn fd_close(&self, fd: types::Fd) -> Result<()> {
        unimplemented!("oops")
    }

    fn fd_datasync(&self, fd: types::Fd) -> Result<()> {
        unimplemented!("oops")
        // self.0.fd_datasync(fd) HA! Wrong "Fd" type
    }

    fn fd_fdstat_get(&self, fd: types::Fd) -> Result<types::Fdstat> {
        unimplemented!("oops")
    }

    fn fd_fdstat_set_flags(&self, fd: types::Fd, flags: types::Fdflags) -> Result<()> {
        unimplemented!("oops")
    }

    fn fd_fdstat_set_rights(
        &self,
        fd: types::Fd,
        fs_rights_base: types::Rights,
        fs_rights_inheriting: types::Rights,
    ) -> Result<()> {
        unimplemented!("oops")
    }

    fn fd_filestat_get(&self, fd: types::Fd) -> Result<types::Filestat> {
        unimplemented!("oops")
    }

    fn fd_filestat_set_size(&self, fd: types::Fd, size: types::Filesize) -> Result<()> {
        unimplemented!("oops")
    }

    fn fd_filestat_set_times(
        &self,
        fd: types::Fd,
        atim: types::Timestamp,
        mtim: types::Timestamp,
        fst_flags: types::Fstflags,
    ) -> Result<()> {
        unimplemented!("oops")
    }

    fn fd_pread(
        &self,
        fd: types::Fd,
        iovs: &types::IovecArray<'_>,
        offset: types::Filesize,
    ) -> Result<types::Size> {
        unimplemented!("oops")
    }

    fn fd_prestat_get(&self, fd: types::Fd) -> Result<types::Prestat> {
        unimplemented!("oops")
    }

    fn fd_prestat_dir_name(
        &self,
        fd: types::Fd,
        path: &GuestPtr<u8>,
        path_len: types::Size,
    ) -> Result<()> {
        unimplemented!("oops")
    }

    fn fd_pwrite(
        &self,
        fd: types::Fd,
        ciovs: &types::CiovecArray<'_>,
        offset: types::Filesize,
    ) -> Result<types::Size> {
        unimplemented!("oops")
    }

    fn fd_read(&self, fd: types::Fd, iovs: &types::IovecArray<'_>) -> Result<types::Size> {
        unimplemented!("oops")
    }

    fn fd_readdir(
        &self,
        fd: types::Fd,
        buf: &GuestPtr<u8>,
        buf_len: types::Size,
        cookie: types::Dircookie,
    ) -> Result<types::Size> {
        unimplemented!("oops")
    }

    fn fd_renumber(&self, from: types::Fd, to: types::Fd) -> Result<()> {
        unimplemented!("oops")
    }

    fn fd_seek(
        &self,
        fd: types::Fd,
        offset: types::Filedelta,
        whence: types::Whence,
    ) -> Result<types::Filesize> {
        unimplemented!("oops")
    }

    fn fd_sync(&self, fd: types::Fd) -> Result<()> {
        unimplemented!("oops")
    }

    fn fd_tell(&self, fd: types::Fd) -> Result<types::Filesize> {
        unimplemented!("oops")
    }

    fn fd_write(&self, fd: types::Fd, ciovs: &types::CiovecArray<'_>) -> Result<types::Size> {
        unimplemented!("oops")
    }

    fn path_create_directory(&self, dirfd: types::Fd, path: &GuestPtr<'_, str>) -> Result<()> {
        unimplemented!("oops")
    }

    fn path_filestat_get(
        &self,
        dirfd: types::Fd,
        flags: types::Lookupflags,
        path: &GuestPtr<'_, str>,
    ) -> Result<types::Filestat> {
        unimplemented!("oops")
    }

    fn path_filestat_set_times(
        &self,
        dirfd: types::Fd,
        flags: types::Lookupflags,
        path: &GuestPtr<'_, str>,
        atim: types::Timestamp,
        mtim: types::Timestamp,
        fst_flags: types::Fstflags,
    ) -> Result<()> {
        unimplemented!("oops")
    }

    fn path_link(
        &self,
        old_fd: types::Fd,
        old_flags: types::Lookupflags,
        old_path: &GuestPtr<'_, str>,
        new_fd: types::Fd,
        new_path: &GuestPtr<'_, str>,
    ) -> Result<()> {
        unimplemented!("oops")
    }

    fn path_open(
        &self,
        dirfd: types::Fd,
        dirflags: types::Lookupflags,
        path: &GuestPtr<'_, str>,
        oflags: types::Oflags,
        fs_rights_base: types::Rights,
        fs_rights_inheriting: types::Rights,
        fdflags: types::Fdflags,
    ) -> Result<types::Fd> {
        unimplemented!("oops")
    }

    fn path_readlink(
        &self,
        dirfd: types::Fd,
        path: &GuestPtr<'_, str>,
        buf: &GuestPtr<u8>,
        buf_len: types::Size,
    ) -> Result<types::Size> {
        unimplemented!("oops")
    }

    fn path_remove_directory(&self, dirfd: types::Fd, path: &GuestPtr<'_, str>) -> Result<()> {
        unimplemented!("oops")
    }

    fn path_rename(
        &self,
        old_fd: types::Fd,
        old_path: &GuestPtr<'_, str>,
        new_fd: types::Fd,
        new_path: &GuestPtr<'_, str>,
    ) -> Result<()> {
        unimplemented!("oops")
    }

    fn path_symlink(
        &self,
        old_path: &GuestPtr<'_, str>,
        dirfd: types::Fd,
        new_path: &GuestPtr<'_, str>,
    ) -> Result<()> {
        unimplemented!("oops")
    }

    fn path_unlink_file(&self, dirfd: types::Fd, path: &GuestPtr<'_, str>) -> Result<()> {
        unimplemented!("oops")
    }

    fn poll_oneoff(
        &self,
        in_: &GuestPtr<types::Subscription>,
        out: &GuestPtr<types::Event>,
        nsubscriptions: types::Size,
    ) -> Result<types::Size> {
        unimplemented!("oops")
    }

    fn proc_exit(&self, _rval: types::Exitcode) -> std::result::Result<(), ()> {
        unimplemented!("oops")
    }

    fn proc_raise(&self, _sig: types::Signal) -> Result<()> {
        unimplemented!("oops")
    }

    fn sched_yield(&self) -> Result<()> {
        unimplemented!("oops")
    }

    fn random_get(&self, buf: &GuestPtr<u8>, buf_len: types::Size) -> Result<()> {
        unimplemented!("oops")
    }

    fn sock_recv(
        &self,
        _fd: types::Fd,
        _ri_data: &types::IovecArray<'_>,
        _ri_flags: types::Riflags,
    ) -> Result<(types::Size, types::Roflags)> {
        unimplemented!("oops")
    }

    fn sock_send(
        &self,
        _fd: types::Fd,
        _si_data: &types::CiovecArray<'_>,
        _si_flags: types::Siflags,
    ) -> Result<types::Size> {
        unimplemented!("oops")
    }

    fn sock_shutdown(&self, _fd: types::Fd, _how: types::Sdflags) -> Result<()> {
        unimplemented!("oops")
    }
} 