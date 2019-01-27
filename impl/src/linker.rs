pub mod linux {
    use syn::Ident;

    pub fn section(ident: &Ident) -> String {
        format!("linkme_{}", ident)
    }

    pub fn section_start(ident: &Ident) -> String {
        format!("__start_linkme_{}", ident)
    }

    pub fn section_stop(ident: &Ident) -> String {
        format!("__stop_linkme_{}", ident)
    }
}

pub mod macos {
    use syn::Ident;

    pub fn section(ident: &Ident) -> String {
        format!("__DATA,__{}", ident)
    }

    pub fn section_start(ident: &Ident) -> String {
        format!("\x01section$start$__DATA$__{}", ident)
    }

    pub fn section_stop(ident: &Ident) -> String {
        format!("\x01section$end$__DATA$__{}", ident)
    }
}