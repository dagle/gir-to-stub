
// changed means that we have translated the gir
// type to a *lua* type. Like changing a gchar *
// to a lua string (because lgi does this).

// Unchanged is when the type is the same, a gtk
// window is a gtk window in C, lua etc. The only difference
// can be the namespace etc.

// Popped is when a variable changes position in the binding.
// Example: int dostuff(Ctx *ctx, err **Error) would be in lua
// int, err = dostuff(ctx) and then the lgi dynamically creates a new
// error when called.

// Removed is for arguments not applicable to the language binding.
// Example: if your language supports closures with scope, maybe the
// binding will remove having a data pointer.
enum Translate {
    Changed(String),
    UnChanged(String),
    Popped(String),
    Removed,
}

pub trait Langbinding {

    // When parsing an arg in a callable, how should an arg be translated?
    // Look at Translate for more info.
    fn translate_arg(&self, arg: &str) -> Translate;

    // something namespace

    // remove?
    fn filter(&self, typ: &str) -> bool;

    // We use only_introspectable to filter out what definitions are acceable
    // for the bindings. For example C can use all functions where lua can only 
    // use introspectable functions.
    fn only_introspectable(&self) -> bool {
        true
    }
}

fn filter(typ: &str) -> bool {
    match typ.as_ref() {
        "gpointer" => true,
        _ => false,
    }
}

fn translate(str: &str, ns: &str) -> String {
    match str {
        "utf8"|"const char*"|"char*" => "string".to_string(),
        "gboolean" => "boolean".to_string(),
        "glong"|"gssize"|"gint64"|"gint"|"gsize"|"guint32" => "num".to_string(),
        "none" => "nil".to_string(),
        rest => {
            if !rest.contains(".") {
                return format!("{}.{}", ns, str)
            }
            rest.to_string()
        }
    }
}
