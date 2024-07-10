#![allow(non_camel_case_types)]
use std::ffi::{c_char, c_int, c_uchar};

#[link(name = "git2")]
extern {
    // const git_error * giterr_last();
    pub fn giterr_last() -> *const git_error;

    // int git_libgit2_init();
    pub fn git_libgit2_init() -> c_int;

    // int git_libgit2_shutdown();
    pub fn git_libgit2_shutdown() -> c_int;

    // int git_repository_open(git_repository **out, const char *path);
    pub fn git_repository_open(out: *mut *mut git_repository, path: *const c_char) -> c_int;

    // void git_repository_free(git_repository *repo);
    pub fn git_repository_free(repo: *const git_repository);

    // int git_reference_name_to_id(git_oid *out, git_repository *repo, const char *name);
    pub fn git_reference_name_to_id(out: *mut git_oid, repo: *const git_repository, name: *const c_char) -> c_int;

    // int git_commit_lookup(git_commit **commit, git_repository *repo, const git_oid *id);
    pub fn git_commit_lookup(git_commit: *mut *mut git_commit, repo: *const git_repository, oid: *const git_oid) -> c_int;

    // void git_commit_free(git_commit *commit);
    pub fn git_commit_free(git_commit: *const git_commit);

    // const git_signature * git_commit_author(const git_commit *commit);
    pub fn git_commit_author(commit: *const git_commit) -> *const git_signature;

    // const char * git_commit_message(const git_commit *commit);
    pub fn git_commit_message(commit: *const git_commit) -> *const c_char;
}


#[repr(C)]
pub struct git_error {
    pub message: *const c_char,
    pub klass: c_int,
}

#[repr(C)]
pub struct git_repository {
    __private: [u8; 0],
}

#[repr(C)]
pub struct git_commit {
    __private: [u8; 0],
}

pub const GIT_OID_RAWSZ: usize = 20;

#[repr(C)]
pub struct git_oid {
    pub id: [c_uchar; GIT_OID_RAWSZ],
}

pub type git_time_t = i64;

#[repr(C)]
pub struct git_time {
    pub time: git_time_t,
    pub offset: c_int,
}

#[repr(C)]
pub struct git_signature {
    pub name: *const c_char,
    pub email: *const c_char,
    pub when: git_time,
}
