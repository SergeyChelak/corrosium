pub fn wait_for_key() {
    uefi::system::with_stdin(|stdin| {
        let Ok(key_event) = stdin.wait_for_key_event() else {
            return;
        };
        _ = uefi::boot::wait_for_event(&mut [key_event]);
        while let Ok(Some(key)) = stdin.read_key() {
            core::hint::black_box(key);
        }
    });
}
