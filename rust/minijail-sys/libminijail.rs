pub type __rlim64_t = u64;
pub type __u8 = u8;
pub type __u16 = u16;
pub type __u32 = u32;

pub type __uid_t = ::std::os::raw::c_uint;
pub type __gid_t = ::std::os::raw::c_uint;
pub type __pid_t = ::std::os::raw::c_int;
pub type rlim_t = __rlim64_t;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type pid_t = __pid_t;
#[repr(C)]
pub struct sock_filter {
    pub code: __u16,
    pub jt: __u8,
    pub jf: __u8,
    pub k: __u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sock_fprog {
    pub len: ::std::os::raw::c_ushort,
    pub filter: *mut sock_filter,
}
pub const MINIJAIL_ERR_NO_ACCESS: _bindgen_ty_1 = _bindgen_ty_1::MINIJAIL_ERR_NO_ACCESS;
pub const MINIJAIL_ERR_NO_COMMAND: _bindgen_ty_1 = _bindgen_ty_1::MINIJAIL_ERR_NO_COMMAND;
pub const MINIJAIL_ERR_SIG_BASE: _bindgen_ty_1 = _bindgen_ty_1::MINIJAIL_ERR_SIG_BASE;
pub const MINIJAIL_ERR_MOUNT: _bindgen_ty_1 = _bindgen_ty_1::MINIJAIL_ERR_MOUNT;
pub const MINIJAIL_ERR_PRELOAD: _bindgen_ty_1 = _bindgen_ty_1::MINIJAIL_ERR_PRELOAD;
pub const MINIJAIL_ERR_JAIL: _bindgen_ty_1 = _bindgen_ty_1::MINIJAIL_ERR_JAIL;
pub const MINIJAIL_ERR_INIT: _bindgen_ty_1 = _bindgen_ty_1::MINIJAIL_ERR_INIT;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_1 {
    MINIJAIL_ERR_NO_ACCESS = 126,
    MINIJAIL_ERR_NO_COMMAND = 127,
    MINIJAIL_ERR_SIG_BASE = 128,
    MINIJAIL_ERR_MOUNT = 251,
    MINIJAIL_ERR_PRELOAD = 252,
    MINIJAIL_ERR_JAIL = 253,
    MINIJAIL_ERR_INIT = 254,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct minijail {
    _unused: [u8; 0],
}
pub type minijail_hook_t = ::std::option::Option<
    unsafe extern "C" fn(context: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
>;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum minijail_hook_event_t {
    MINIJAIL_HOOK_EVENT_PRE_DROP_CAPS = 0,
    MINIJAIL_HOOK_EVENT_PRE_EXECVE = 1,
    MINIJAIL_HOOK_EVENT_PRE_CHROOT = 2,
    MINIJAIL_HOOK_EVENT_MAX = 3,
}
extern "C" {
    pub fn minijail_new() -> *mut minijail;
}
extern "C" {
    pub fn minijail_change_uid(j: *mut minijail, uid: uid_t);
}
extern "C" {
    pub fn minijail_change_gid(j: *mut minijail, gid: gid_t);
}
extern "C" {
    pub fn minijail_set_supplementary_gids(j: *mut minijail, size: usize, list: *const gid_t);
}
extern "C" {
    pub fn minijail_keep_supplementary_gids(j: *mut minijail);
}
extern "C" {
    pub fn minijail_change_user(
        j: *mut minijail,
        user: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn minijail_change_group(
        j: *mut minijail,
        group: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn minijail_use_seccomp(j: *mut minijail);
}
extern "C" {
    pub fn minijail_no_new_privs(j: *mut minijail);
}
extern "C" {
    pub fn minijail_use_seccomp_filter(j: *mut minijail);
}
extern "C" {
    pub fn minijail_set_seccomp_filter_tsync(j: *mut minijail);
}
extern "C" {
    pub fn minijail_set_using_minimalistic_mountns(j: *mut minijail);
}
extern "C" {
    pub fn minijail_add_minimalistic_mountns_fs_rules(j: *mut minijail);
}
extern "C" {
    pub fn minijail_enable_default_fs_restrictions(j: *mut minijail);
}
extern "C" {
    pub fn minijail_set_seccomp_filter_allow_speculation(j: *mut minijail);
}
extern "C" {
    pub fn minijail_set_seccomp_filters(j: *mut minijail, filter: *const sock_fprog);
}
extern "C" {
    pub fn minijail_parse_seccomp_filters(j: *mut minijail, path: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn minijail_parse_seccomp_filters_from_fd(j: *mut minijail, fd: ::std::os::raw::c_int);
}
extern "C" {
    pub fn minijail_log_seccomp_filter_failures(j: *mut minijail);
}
extern "C" {
    pub fn minijail_use_caps(j: *mut minijail, capmask: u64);
}
extern "C" {
    pub fn minijail_capbset_drop(j: *mut minijail, capmask: u64);
}
extern "C" {
    pub fn minijail_set_ambient_caps(j: *mut minijail);
}
extern "C" {
    pub fn minijail_reset_signal_mask(j: *mut minijail);
}
extern "C" {
    pub fn minijail_reset_signal_handlers(j: *mut minijail);
}
extern "C" {
    pub fn minijail_namespace_vfs(j: *mut minijail);
}
extern "C" {
    pub fn minijail_namespace_enter_vfs(j: *mut minijail, ns_path: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn minijail_new_session_keyring(j: *mut minijail);
}
extern "C" {
    pub fn minijail_skip_setting_securebits(j: *mut minijail, securebits_skip_mask: u64);
}
extern "C" {
    pub fn minijail_skip_remount_private(j: *mut minijail);
}
extern "C" {
    pub fn minijail_remount_mode(j: *mut minijail, mode: ::std::os::raw::c_ulong);
}
extern "C" {
    pub fn minijail_namespace_ipc(j: *mut minijail);
}
extern "C" {
    pub fn minijail_namespace_uts(j: *mut minijail);
}
extern "C" {
    pub fn minijail_namespace_set_hostname(
        j: *mut minijail,
        name: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn minijail_namespace_net(j: *mut minijail);
}
extern "C" {
    pub fn minijail_namespace_enter_net(j: *mut minijail, ns_path: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn minijail_namespace_cgroups(j: *mut minijail);
}
extern "C" {
    pub fn minijail_close_open_fds(j: *mut minijail);
}
extern "C" {
    pub fn minijail_namespace_pids(j: *mut minijail);
}
extern "C" {
    pub fn minijail_namespace_pids_rw_proc(j: *mut minijail);
}
extern "C" {
    pub fn minijail_namespace_user(j: *mut minijail);
}
extern "C" {
    pub fn minijail_namespace_user_disable_setgroups(j: *mut minijail);
}
extern "C" {
    pub fn minijail_uidmap(
        j: *mut minijail,
        uidmap: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn minijail_gidmap(
        j: *mut minijail,
        gidmap: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn minijail_remount_proc_readonly(j: *mut minijail);
}
extern "C" {
    pub fn minijail_run_as_init(j: *mut minijail);
}
extern "C" {
    pub fn minijail_write_pid_file(
        j: *mut minijail,
        path: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn minijail_inherit_usergroups(j: *mut minijail);
}
extern "C" {
    pub fn minijail_use_alt_syscall(
        j: *mut minijail,
        table: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn minijail_rlimit(
        j: *mut minijail,
        type_: ::std::os::raw::c_int,
        cur: rlim_t,
        max: rlim_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn minijail_add_to_cgroup(
        j: *mut minijail,
        path: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn minijail_add_fs_restriction_rx(
        j: *mut minijail,
        path: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn minijail_add_fs_restriction_ro(
        j: *mut minijail,
        path: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn minijail_add_fs_restriction_rw(
        j: *mut minijail,
        path: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn minijail_add_fs_restriction_advanced_rw(
        j: *mut minijail,
        path: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn minijail_add_fs_restriction_edit(
        j: *mut minijail,
        path: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn minijail_forward_signals(j: *mut minijail) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn minijail_create_session(j: *mut minijail) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn minijail_enter_chroot(
        j: *mut minijail,
        dir: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn minijail_enter_pivot_root(
        j: *mut minijail,
        dir: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn minijail_get_original_path(
        j: *mut minijail,
        chroot_path: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn minijail_mount_tmp(j: *mut minijail);
}
extern "C" {
    pub fn minijail_mount_tmp_size(j: *mut minijail, size: usize);
}
extern "C" {
    pub fn minijail_mount_dev(j: *mut minijail);
}
extern "C" {
    pub fn minijail_mount_with_data(
        j: *mut minijail,
        src: *const ::std::os::raw::c_char,
        dest: *const ::std::os::raw::c_char,
        type_: *const ::std::os::raw::c_char,
        flags: ::std::os::raw::c_ulong,
        data: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn minijail_mount(
        j: *mut minijail,
        src: *const ::std::os::raw::c_char,
        dest: *const ::std::os::raw::c_char,
        type_: *const ::std::os::raw::c_char,
        flags: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn minijail_bind(
        j: *mut minijail,
        src: *const ::std::os::raw::c_char,
        dest: *const ::std::os::raw::c_char,
        writeable: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn minijail_add_remount(
        j: *mut minijail,
        mount_name: *const ::std::os::raw::c_char,
        remount_mode: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn minijail_add_hook(
        j: *mut minijail,
        hook: minijail_hook_t,
        payload: *mut ::std::os::raw::c_void,
        event: minijail_hook_event_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn minijail_preserve_fd(
        j: *mut minijail,
        parent_fd: ::std::os::raw::c_int,
        child_fd: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn minijail_set_preload_path(
        j: *mut minijail,
        preload_path: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn minijail_enter(j: *const minijail);
}
extern "C" {
    pub fn minijail_run_env(
        j: *mut minijail,
        filename: *const ::std::os::raw::c_char,
        argv: *const *mut ::std::os::raw::c_char,
        envp: *const *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn minijail_run(
        j: *mut minijail,
        filename: *const ::std::os::raw::c_char,
        argv: *const *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn minijail_run_no_preload(
        j: *mut minijail,
        filename: *const ::std::os::raw::c_char,
        argv: *const *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn minijail_run_pid(
        j: *mut minijail,
        filename: *const ::std::os::raw::c_char,
        argv: *const *mut ::std::os::raw::c_char,
        pchild_pid: *mut pid_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn minijail_run_pipe(
        j: *mut minijail,
        filename: *const ::std::os::raw::c_char,
        argv: *const *mut ::std::os::raw::c_char,
        pstdin_fd: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn minijail_run_pid_pipes(
        j: *mut minijail,
        filename: *const ::std::os::raw::c_char,
        argv: *const *mut ::std::os::raw::c_char,
        pchild_pid: *mut pid_t,
        pstdin_fd: *mut ::std::os::raw::c_int,
        pstdout_fd: *mut ::std::os::raw::c_int,
        pstderr_fd: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn minijail_run_env_pid_pipes(
        j: *mut minijail,
        filename: *const ::std::os::raw::c_char,
        argv: *const *mut ::std::os::raw::c_char,
        envp: *const *mut ::std::os::raw::c_char,
        pchild_pid: *mut pid_t,
        pstdin_fd: *mut ::std::os::raw::c_int,
        pstdout_fd: *mut ::std::os::raw::c_int,
        pstderr_fd: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn minijail_run_fd_env_pid_pipes(
        j: *mut minijail,
        elf_fd: ::std::os::raw::c_int,
        argv: *const *mut ::std::os::raw::c_char,
        envp: *const *mut ::std::os::raw::c_char,
        pchild_pid: *mut pid_t,
        pstdin_fd: *mut ::std::os::raw::c_int,
        pstdout_fd: *mut ::std::os::raw::c_int,
        pstderr_fd: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn minijail_run_pid_pipes_no_preload(
        j: *mut minijail,
        filename: *const ::std::os::raw::c_char,
        argv: *const *mut ::std::os::raw::c_char,
        pchild_pid: *mut pid_t,
        pstdin_fd: *mut ::std::os::raw::c_int,
        pstdout_fd: *mut ::std::os::raw::c_int,
        pstderr_fd: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn minijail_run_env_pid_pipes_no_preload(
        j: *mut minijail,
        filename: *const ::std::os::raw::c_char,
        argv: *const *mut ::std::os::raw::c_char,
        envp: *const *mut ::std::os::raw::c_char,
        pchild_pid: *mut pid_t,
        pstdin_fd: *mut ::std::os::raw::c_int,
        pstdout_fd: *mut ::std::os::raw::c_int,
        pstderr_fd: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn minijail_fork(j: *mut minijail) -> pid_t;
}
extern "C" {
    pub fn minijail_kill(j: *mut minijail) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn minijail_wait(j: *mut minijail) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn minijail_destroy(j: *mut minijail);
}
extern "C" {
    pub fn minijail_copy_jail(from: *const minijail, out: *mut minijail) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn minijail_log_to_fd(fd: ::std::os::raw::c_int, min_priority: ::std::os::raw::c_int);
}
