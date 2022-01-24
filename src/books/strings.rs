use lazy_static::lazy_static;
use std::sync::Arc;
use std::collections::HashMap;
use dropshot::RequestContext;

lazy_static! {
    // Hash of strings for translations.
    static ref I18N: HashMap<&'static str, HashMap<&'static str, &'static str>> = {
        let mut m = HashMap::new();
        m.insert(
            "emptytitle",
            HashMap::from([("ca", "no has especificat un títol"), ("en", "title not specified")]),
        );
        m.insert(
            "emptypublisher",
            HashMap::from([("ca", "no has especificat una editorial"), ("en", "publisher not specified")]),
        );
        m.insert(
            "emptylanguage",
            HashMap::from([("ca", "no has especificat un idioma"), ("en", "language not specified")]),
        );
        m.insert(
            "badrating",
            HashMap::from([("ca", "s'ha d'especificar una qualificació entre 0 i 10"), ("en", "rate not between 0 and 10")]),
        );
        m.insert(
            "badstatus",
            HashMap::from([("ca", "l'estat conté un valor estrany"), ("en", "status contains a bad value")]),
        );
        m.insert(
            "badkind",
            HashMap::from([("ca", "tipus desconegut"), ("en", "unknown kind given")]),
        );
        m.insert(
            "notfound",
            HashMap::from([("ca", "no trobat"), ("en", "not found")]),
        );
        m.insert(
            "oops",
            HashMap::from([("ca", "ups"), ("en", "oops")]),
        );
        m.insert(
            "unknown",
            HashMap::from([("ca", "error desconegut"), ("en", "unknown error")]),
        );

        m
    };

    // Array with codes of supported languages.
    static ref SUPPORTED_LANGUAGES: [&'static str; 2] = ["ca", "en"];
}

// Returns a String containing the language identifier for the given
// RequestContext. This function is `async` because accessing the request itself
// requires `await`.
pub async fn lang_from_context(ctx: Arc<RequestContext<()>>) -> String {
    let request = ctx.request.lock().await;
    let lang = &request.headers()["accept-language"];
    let lang = lang.to_str().unwrap_or("en");

    for it in SUPPORTED_LANGUAGES.iter() {
        let s: &'static str = it;
        if s == lang {
            return String::from(lang);
        }
    }
    String::from("en")
}

// Returns a String containing the translated value for `key` into the given
// language `lang`.
pub fn t(lang: &str, key: &'static str) -> String {
    match I18N.get(key) {
        Some(hsh) => match hsh.get(lang) {
            Some(v) => String::from(*v),
            None => String::from(""),
        },
        None => String::from("")
    }
}
