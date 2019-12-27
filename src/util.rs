#[cfg(target_arch = "arm")]
pub fn str_to_connarr(connstring: &str) -> [u8; 1024] {
    let end = std::cmp::min(1024, connstring.len());
    let mut connarr: [u8; 1024] = [0; 1024];
    connarr[..end].copy_from_slice(&connstring.as_bytes()[0..end]);
    connarr
}

#[cfg(not(target_arch = "arm"))]
pub fn str_to_connarr(connstring: &str) -> [i8; 1024] {
    let end = std::cmp::min(1024, connstring.len());
    let mut connarr: [i8; 1024] = [0; 1024];
    connarr[..end]
        .copy_from_slice(&unsafe { &*(connstring.as_bytes() as *const _ as *const [i8]) }[0..end]);
    connarr
}
