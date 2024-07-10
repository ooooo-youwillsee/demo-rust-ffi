use std::{mem, ptr};
use std::ffi::{CStr, CString};

use demo_rust_ffi::git::check;
use demo_rust_ffi::git::raw::{git_commit, git_commit_author, git_commit_free, git_commit_lookup, git_commit_message, git_libgit2_init, git_libgit2_shutdown, git_reference_name_to_id, git_repository_free, git_repository_open};

fn main() {
    unsafe {
        check("init library", git_libgit2_init());

        let path = CString::new("/Users/ooooo/Code/IdeaProjects/demo-rust-ffi").unwrap();
        let mut repo = ptr::null_mut();
        check("open repository", git_repository_open(&mut repo, path.as_ptr()));

        let name = CString::new("HEAD").unwrap();
        let mut oid = mem::MaybeUninit::uninit();
        check("look up HEAD", git_reference_name_to_id(oid.as_mut_ptr(), repo, name.as_ptr()));
        let oid = oid.assume_init();

        let mut commit = ptr::null_mut();
        check("look up commit", git_commit_lookup(&mut commit, repo, &oid));

        show_commit(commit);

        git_commit_free(commit);

        git_repository_free(repo);

        check("showdown library", git_libgit2_shutdown());
    }
}


unsafe fn show_commit(commit: *const git_commit) {
    let author = &*git_commit_author(commit);
    let name = CStr::from_ptr(author.name).to_string_lossy();
    let email = CStr::from_ptr(author.email).to_string_lossy();
    println!("{} <{}>\n", name, email);

    let message = git_commit_message(commit);
    println!("{}", CStr::from_ptr(message).to_string_lossy());
}

