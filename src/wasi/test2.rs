pub mod test {
    struct XXX;
    pub mod types {
        pub trait GuestErrorConversion {
            fn into_f32(&self, e: wiggle::GuestError) -> f32;
        }
        pub trait UserErrorConversion {}
    }
    pub mod atoms {
        use super::XXX;
        use super::types::*;
        pub fn fun1(ctx: &XXX, memory: &dyn wiggle::GuestMemory, an_int: i32) -> f32 {
            let _span = {
                use ::tracing::__macro_support::Callsite as _;
                static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                    use ::tracing::__macro_support::MacroCallsite;
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "wiggle abi",
                            "lunatic_vm::wasi::test::atoms",
                            wiggle::tracing::Level::TRACE,
                            Some("src/wasi/test.rs"),
                            Some(4u32),
                            Some("lunatic_vm::wasi::test::atoms"),
                            ::tracing_core::field::FieldSet::new(
                                &["module", "function"],
                                ::tracing_core::callsite::Identifier(&CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    MacroCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if wiggle::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && wiggle::tracing::Level::TRACE
                        <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = CALLSITE.interest();
                        !interest.is_never()
                    }
                    && CALLSITE.is_enabled(interest)
                {
                    let meta = CALLSITE.metadata();
                    ::tracing::Span::new(meta, &{
                        #[allow(unused_imports)]
                        use ::tracing::field::{debug, display, Value};
                        let mut iter = meta.fields().iter();
                        meta.fields().value_set(&[
                            (
                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                Some(&"atoms" as &Value),
                            ),
                            (
                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                Some(&"fun1" as &Value),
                            ),
                        ])
                    })
                } else {
                    let span = CALLSITE.disabled_span();
                    if !::tracing::dispatcher::has_been_set() {
                        {
                            span.record_all(&{
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = CALLSITE.metadata().fields().iter();
                                CALLSITE.metadata().fields().value_set(&[
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&"atoms" as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&"fun1" as &Value),
                                    ),
                                ])
                            });
                        }
                    } else {
                        {}
                    };
                    span
                }
            };
            let _enter = _span.enter();
            let an_int = an_int as u32;
            {
                if !::tracing::dispatcher::has_been_set() {
                    {
                        use ::tracing::log;
                        let level = match wiggle::tracing::Level::TRACE {
                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                            _ => ::tracing::log::Level::Trace,
                        };
                        if level <= log::STATIC_MAX_LEVEL && level <= log::max_level() {
                            let log_meta = log::Metadata::builder()
                                .level(level)
                                .target("lunatic_vm::wasi::test::atoms")
                                .build();
                            let logger = log::logger();
                            if logger.enabled(&log_meta) {
                                logger.log(
                                    &log::Record::builder()
                                        .file(Some("src/wasi/test.rs"))
                                        .module_path(Some("lunatic_vm::wasi::test::atoms"))
                                        .line(Some(4u32))
                                        .metadata(log_meta)
                                        .args({
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display};
                                            ::core::fmt::Arguments::new_v1(
                                                &["an_int=", " "],
                                                &match (&wiggle::tracing::field::display(&an_int),)
                                                {
                                                    (arg0,) => [::core::fmt::ArgumentV1::new(
                                                        arg0,
                                                        ::core::fmt::Debug::fmt,
                                                    )],
                                                },
                                            )
                                        })
                                        .build(),
                                );
                            }
                        }
                    }
                } else {
                    {}
                };
                if wiggle::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && wiggle::tracing::Level::TRACE
                        <= ::tracing::level_filters::LevelFilter::current()
                {
                    use ::tracing::__macro_support::*;
                    static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                        use ::tracing::__macro_support::MacroCallsite;
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event src/wasi/test.rs:4",
                                "lunatic_vm::wasi::test::atoms",
                                wiggle::tracing::Level::TRACE,
                                Some("src/wasi/test.rs"),
                                Some(4u32),
                                Some("lunatic_vm::wasi::test::atoms"),
                                ::tracing_core::field::FieldSet::new(
                                    &["an_int"],
                                    ::tracing_core::callsite::Identifier(&CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        MacroCallsite::new(&META)
                    };
                    let interest = CALLSITE.interest();
                    if !interest.is_never() && CALLSITE.is_enabled(interest) {
                        let meta = CALLSITE.metadata();
                        ::tracing::Event::dispatch(meta, &{
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = meta.fields().iter();
                            meta.fields().value_set(&[(
                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                Some(&wiggle::tracing::field::display(&an_int) as &Value),
                            )])
                        });
                    }
                }
            };
            let _ = match Atoms::fun1(ctx, an_int) {
                Ok(_) => {}
                Err(e) => {
                    let e = e;
                    {
                        if !::tracing::dispatcher::has_been_set() {
                            {
                                use ::tracing::log;
                                let level = match wiggle::tracing::Level::TRACE {
                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                    _ => ::tracing::log::Level::Trace,
                                };
                                if level <= log::STATIC_MAX_LEVEL && level <= log::max_level() {
                                    let log_meta = log::Metadata::builder()
                                        .level(level)
                                        .target("lunatic_vm::wasi::test::atoms")
                                        .build();
                                    let logger = log::logger();
                                    if logger.enabled(&log_meta) {
                                        logger.log(
                                            &log::Record::builder()
                                                .file(Some("src/wasi/test.rs"))
                                                .module_path(Some("lunatic_vm::wasi::test::atoms"))
                                                .line(Some(4u32))
                                                .metadata(log_meta)
                                                .args({
                                                    #[allow(unused_imports)]
                                                    use ::tracing::field::{debug, display};
                                                    ::core::fmt::Arguments::new_v1(
                                                        &["error=", " "],
                                                        &match (&wiggle::tracing::field::debug(&e),)
                                                        {
                                                            (arg0,) => {
                                                                [::core::fmt::ArgumentV1::new(
                                                                    arg0,
                                                                    ::core::fmt::Debug::fmt,
                                                                )]
                                                            }
                                                        },
                                                    )
                                                })
                                                .build(),
                                        );
                                    }
                                }
                            }
                        } else {
                            {}
                        };
                        if wiggle::tracing::Level::TRACE
                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && wiggle::tracing::Level::TRACE
                                <= ::tracing::level_filters::LevelFilter::current()
                        {
                            use ::tracing::__macro_support::*;
                            static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                                use ::tracing::__macro_support::MacroCallsite;
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event src/wasi/test.rs:4",
                                        "lunatic_vm::wasi::test::atoms",
                                        wiggle::tracing::Level::TRACE,
                                        Some("src/wasi/test.rs"),
                                        Some(4u32),
                                        Some("lunatic_vm::wasi::test::atoms"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["error"],
                                            ::tracing_core::callsite::Identifier(&CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                MacroCallsite::new(&META)
                            };
                            let interest = CALLSITE.interest();
                            if !interest.is_never() && CALLSITE.is_enabled(interest) {
                                let meta = CALLSITE.metadata();
                                ::tracing::Event::dispatch(meta, &{
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = meta.fields().iter();
                                    meta.fields().value_set(&[(
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&wiggle::tracing::field::debug(&e) as &Value),
                                    )])
                                });
                            }
                        }
                    };
                    return f32::from(e);
                }
            };
            let success: f32 = wiggle::GuestErrorType::success();
            {
                if !::tracing::dispatcher::has_been_set() {
                    {
                        use ::tracing::log;
                        let level = match wiggle::tracing::Level::TRACE {
                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                            _ => ::tracing::log::Level::Trace,
                        };
                        if level <= log::STATIC_MAX_LEVEL && level <= log::max_level() {
                            let log_meta = log::Metadata::builder()
                                .level(level)
                                .target("lunatic_vm::wasi::test::atoms")
                                .build();
                            let logger = log::logger();
                            if logger.enabled(&log_meta) {
                                logger.log(
                                    &log::Record::builder()
                                        .file(Some("src/wasi/test.rs"))
                                        .module_path(Some("lunatic_vm::wasi::test::atoms"))
                                        .line(Some(4u32))
                                        .metadata(log_meta)
                                        .args({
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display};
                                            ::core::fmt::Arguments::new_v1(
                                                &["success=", " "],
                                                &match (&wiggle::tracing::field::display(&success),)
                                                {
                                                    (arg0,) => [::core::fmt::ArgumentV1::new(
                                                        arg0,
                                                        ::core::fmt::Debug::fmt,
                                                    )],
                                                },
                                            )
                                        })
                                        .build(),
                                );
                            }
                        }
                    }
                } else {
                    {}
                };
                if wiggle::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && wiggle::tracing::Level::TRACE
                        <= ::tracing::level_filters::LevelFilter::current()
                {
                    use ::tracing::__macro_support::*;
                    static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                        use ::tracing::__macro_support::MacroCallsite;
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event src/wasi/test.rs:4",
                                "lunatic_vm::wasi::test::atoms",
                                wiggle::tracing::Level::TRACE,
                                Some("src/wasi/test.rs"),
                                Some(4u32),
                                Some("lunatic_vm::wasi::test::atoms"),
                                ::tracing_core::field::FieldSet::new(
                                    &["success"],
                                    ::tracing_core::callsite::Identifier(&CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        MacroCallsite::new(&META)
                    };
                    let interest = CALLSITE.interest();
                    if !interest.is_never() && CALLSITE.is_enabled(interest) {
                        let meta = CALLSITE.metadata();
                        ::tracing::Event::dispatch(meta, &{
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = meta.fields().iter();
                            meta.fields().value_set(&[(
                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                Some(&wiggle::tracing::field::display(&success) as &Value),
                            )])
                        });
                    }
                }
            };
            f32::from(success)
        }
        pub fn fun2(
            ctx: &XXX,
            memory: &dyn wiggle::GuestMemory,
            an_int: i32,
            doubled_it_ptr: i32,
        ) -> f32 {
            let _span = {
                use ::tracing::__macro_support::Callsite as _;
                static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                    use ::tracing::__macro_support::MacroCallsite;
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "wiggle abi",
                            "lunatic_vm::wasi::test::atoms",
                            wiggle::tracing::Level::TRACE,
                            Some("src/wasi/test.rs"),
                            Some(4u32),
                            Some("lunatic_vm::wasi::test::atoms"),
                            ::tracing_core::field::FieldSet::new(
                                &["module", "function"],
                                ::tracing_core::callsite::Identifier(&CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    MacroCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if wiggle::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && wiggle::tracing::Level::TRACE
                        <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = CALLSITE.interest();
                        !interest.is_never()
                    }
                    && CALLSITE.is_enabled(interest)
                {
                    let meta = CALLSITE.metadata();
                    ::tracing::Span::new(meta, &{
                        #[allow(unused_imports)]
                        use ::tracing::field::{debug, display, Value};
                        let mut iter = meta.fields().iter();
                        meta.fields().value_set(&[
                            (
                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                Some(&"atoms" as &Value),
                            ),
                            (
                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                Some(&"fun2" as &Value),
                            ),
                        ])
                    })
                } else {
                    let span = CALLSITE.disabled_span();
                    if !::tracing::dispatcher::has_been_set() {
                        {
                            span.record_all(&{
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = CALLSITE.metadata().fields().iter();
                                CALLSITE.metadata().fields().value_set(&[
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&"atoms" as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&"fun2" as &Value),
                                    ),
                                ])
                            });
                        }
                    } else {
                        {}
                    };
                    span
                }
            };
            let _enter = _span.enter();
            let an_int = an_int as u32;
            let doubled_it_ptr = wiggle::GuestPtr::<f32>::new(memory, doubled_it_ptr as u32);
            {
                if !::tracing::dispatcher::has_been_set() {
                    {
                        use ::tracing::log;
                        let level = match wiggle::tracing::Level::TRACE {
                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                            _ => ::tracing::log::Level::Trace,
                        };
                        if level <= log::STATIC_MAX_LEVEL && level <= log::max_level() {
                            let log_meta = log::Metadata::builder()
                                .level(level)
                                .target("lunatic_vm::wasi::test::atoms")
                                .build();
                            let logger = log::logger();
                            if logger.enabled(&log_meta) {
                                logger.log(
                                    &log::Record::builder()
                                        .file(Some("src/wasi/test.rs"))
                                        .module_path(Some("lunatic_vm::wasi::test::atoms"))
                                        .line(Some(4u32))
                                        .metadata(log_meta)
                                        .args({
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display};
                                            ::core::fmt::Arguments::new_v1(
                                                &["an_int=", " "],
                                                &match (&wiggle::tracing::field::display(&an_int),)
                                                {
                                                    (arg0,) => [::core::fmt::ArgumentV1::new(
                                                        arg0,
                                                        ::core::fmt::Debug::fmt,
                                                    )],
                                                },
                                            )
                                        })
                                        .build(),
                                );
                            }
                        }
                    }
                } else {
                    {}
                };
                if wiggle::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && wiggle::tracing::Level::TRACE
                        <= ::tracing::level_filters::LevelFilter::current()
                {
                    use ::tracing::__macro_support::*;
                    static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                        use ::tracing::__macro_support::MacroCallsite;
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event src/wasi/test.rs:4",
                                "lunatic_vm::wasi::test::atoms",
                                wiggle::tracing::Level::TRACE,
                                Some("src/wasi/test.rs"),
                                Some(4u32),
                                Some("lunatic_vm::wasi::test::atoms"),
                                ::tracing_core::field::FieldSet::new(
                                    &["an_int"],
                                    ::tracing_core::callsite::Identifier(&CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        MacroCallsite::new(&META)
                    };
                    let interest = CALLSITE.interest();
                    if !interest.is_never() && CALLSITE.is_enabled(interest) {
                        let meta = CALLSITE.metadata();
                        ::tracing::Event::dispatch(meta, &{
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = meta.fields().iter();
                            meta.fields().value_set(&[(
                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                Some(&wiggle::tracing::field::display(&an_int) as &Value),
                            )])
                        });
                    }
                }
            };
            let (doubled_it) = match Atoms::fun2(ctx, an_int) {
                Ok((doubled_it)) => {
                    {
                        if !::tracing::dispatcher::has_been_set() {
                            {
                                use ::tracing::log;
                                let level = match wiggle::tracing::Level::TRACE {
                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                    _ => ::tracing::log::Level::Trace,
                                };
                                if level <= log::STATIC_MAX_LEVEL && level <= log::max_level() {
                                    let log_meta = log::Metadata::builder()
                                        .level(level)
                                        .target("lunatic_vm::wasi::test::atoms")
                                        .build();
                                    let logger = log::logger();
                                    if logger.enabled(&log_meta) {
                                        logger.log(
                                            &log::Record::builder()
                                                .file(Some("src/wasi/test.rs"))
                                                .module_path(Some("lunatic_vm::wasi::test::atoms"))
                                                .line(Some(4u32))
                                                .metadata(log_meta)
                                                .args({
                                                    #[allow(unused_imports)]
                                                    use ::tracing::field::{debug, display};
                                                    ::core::fmt::Arguments::new_v1(
                                                        &["doubled_it=", " "],
                                                        &match (&wiggle::tracing::field::display(
                                                            &doubled_it,
                                                        ),)
                                                        {
                                                            (arg0,) => {
                                                                [::core::fmt::ArgumentV1::new(
                                                                    arg0,
                                                                    ::core::fmt::Debug::fmt,
                                                                )]
                                                            }
                                                        },
                                                    )
                                                })
                                                .build(),
                                        );
                                    }
                                }
                            }
                        } else {
                            {}
                        };
                        if wiggle::tracing::Level::TRACE
                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && wiggle::tracing::Level::TRACE
                                <= ::tracing::level_filters::LevelFilter::current()
                        {
                            use ::tracing::__macro_support::*;
                            static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                                use ::tracing::__macro_support::MacroCallsite;
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event src/wasi/test.rs:4",
                                        "lunatic_vm::wasi::test::atoms",
                                        wiggle::tracing::Level::TRACE,
                                        Some("src/wasi/test.rs"),
                                        Some(4u32),
                                        Some("lunatic_vm::wasi::test::atoms"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["doubled_it"],
                                            ::tracing_core::callsite::Identifier(&CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                MacroCallsite::new(&META)
                            };
                            let interest = CALLSITE.interest();
                            if !interest.is_never() && CALLSITE.is_enabled(interest) {
                                let meta = CALLSITE.metadata();
                                ::tracing::Event::dispatch(meta, &{
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = meta.fields().iter();
                                    meta.fields().value_set(&[(
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(
                                            &wiggle::tracing::field::display(&doubled_it) as &Value
                                        ),
                                    )])
                                });
                            }
                        }
                    };
                    (doubled_it)
                }
                Err(e) => {
                    let e = e;
                    {
                        if !::tracing::dispatcher::has_been_set() {
                            {
                                use ::tracing::log;
                                let level = match wiggle::tracing::Level::TRACE {
                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                    _ => ::tracing::log::Level::Trace,
                                };
                                if level <= log::STATIC_MAX_LEVEL && level <= log::max_level() {
                                    let log_meta = log::Metadata::builder()
                                        .level(level)
                                        .target("lunatic_vm::wasi::test::atoms")
                                        .build();
                                    let logger = log::logger();
                                    if logger.enabled(&log_meta) {
                                        logger.log(
                                            &log::Record::builder()
                                                .file(Some("src/wasi/test.rs"))
                                                .module_path(Some("lunatic_vm::wasi::test::atoms"))
                                                .line(Some(4u32))
                                                .metadata(log_meta)
                                                .args({
                                                    #[allow(unused_imports)]
                                                    use ::tracing::field::{debug, display};
                                                    ::core::fmt::Arguments::new_v1(
                                                        &["error=", " "],
                                                        &match (&wiggle::tracing::field::debug(&e),)
                                                        {
                                                            (arg0,) => {
                                                                [::core::fmt::ArgumentV1::new(
                                                                    arg0,
                                                                    ::core::fmt::Debug::fmt,
                                                                )]
                                                            }
                                                        },
                                                    )
                                                })
                                                .build(),
                                        );
                                    }
                                }
                            }
                        } else {
                            {}
                        };
                        if wiggle::tracing::Level::TRACE
                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && wiggle::tracing::Level::TRACE
                                <= ::tracing::level_filters::LevelFilter::current()
                        {
                            use ::tracing::__macro_support::*;
                            static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                                use ::tracing::__macro_support::MacroCallsite;
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event src/wasi/test.rs:4",
                                        "lunatic_vm::wasi::test::atoms",
                                        wiggle::tracing::Level::TRACE,
                                        Some("src/wasi/test.rs"),
                                        Some(4u32),
                                        Some("lunatic_vm::wasi::test::atoms"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["error"],
                                            ::tracing_core::callsite::Identifier(&CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                MacroCallsite::new(&META)
                            };
                            let interest = CALLSITE.interest();
                            if !interest.is_never() && CALLSITE.is_enabled(interest) {
                                let meta = CALLSITE.metadata();
                                ::tracing::Event::dispatch(meta, &{
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = meta.fields().iter();
                                    meta.fields().value_set(&[(
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&wiggle::tracing::field::debug(&e) as &Value),
                                    )])
                                });
                            }
                        }
                    };
                    return f32::from(e);
                }
            };
            if let Err(e) = doubled_it_ptr.write(doubled_it) {
                let e = wiggle::GuestError::InFunc {
                    funcname: "fun2",
                    location: "doubled_it:result_ptr_mut",
                    err: Box::new(e.into()),
                };
                let err: f32 = GuestErrorConversion::into_f32(ctx, e);
                return f32::from(err);
            }
            let success: f32 = wiggle::GuestErrorType::success();
            {
                if !::tracing::dispatcher::has_been_set() {
                    {
                        use ::tracing::log;
                        let level = match wiggle::tracing::Level::TRACE {
                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                            _ => ::tracing::log::Level::Trace,
                        };
                        if level <= log::STATIC_MAX_LEVEL && level <= log::max_level() {
                            let log_meta = log::Metadata::builder()
                                .level(level)
                                .target("lunatic_vm::wasi::test::atoms")
                                .build();
                            let logger = log::logger();
                            if logger.enabled(&log_meta) {
                                logger.log(
                                    &log::Record::builder()
                                        .file(Some("src/wasi/test.rs"))
                                        .module_path(Some("lunatic_vm::wasi::test::atoms"))
                                        .line(Some(4u32))
                                        .metadata(log_meta)
                                        .args({
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display};
                                            ::core::fmt::Arguments::new_v1(
                                                &["success=", " "],
                                                &match (&wiggle::tracing::field::display(&success),)
                                                {
                                                    (arg0,) => [::core::fmt::ArgumentV1::new(
                                                        arg0,
                                                        ::core::fmt::Debug::fmt,
                                                    )],
                                                },
                                            )
                                        })
                                        .build(),
                                );
                            }
                        }
                    }
                } else {
                    {}
                };
                if wiggle::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && wiggle::tracing::Level::TRACE
                        <= ::tracing::level_filters::LevelFilter::current()
                {
                    use ::tracing::__macro_support::*;
                    static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                        use ::tracing::__macro_support::MacroCallsite;
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event src/wasi/test.rs:4",
                                "lunatic_vm::wasi::test::atoms",
                                wiggle::tracing::Level::TRACE,
                                Some("src/wasi/test.rs"),
                                Some(4u32),
                                Some("lunatic_vm::wasi::test::atoms"),
                                ::tracing_core::field::FieldSet::new(
                                    &["success"],
                                    ::tracing_core::callsite::Identifier(&CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        MacroCallsite::new(&META)
                    };
                    let interest = CALLSITE.interest();
                    if !interest.is_never() && CALLSITE.is_enabled(interest) {
                        let meta = CALLSITE.metadata();
                        ::tracing::Event::dispatch(meta, &{
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = meta.fields().iter();
                            meta.fields().value_set(&[(
                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                Some(&wiggle::tracing::field::display(&success) as &Value),
                            )])
                        });
                    }
                }
            };
            f32::from(success)
        }
        pub trait Atoms {
            fn fun1(&self, an_int: u32) -> Result<(), f32>;
            fn fun2(&self, an_int: u32) -> Result<(f32), f32>;
        }
    }
    pub mod metadata {
        pub const DOC_TEXT : & str = "(module $atoms (@interface func (export \"fun1\") (param $an_int u32) (result $error f32)) (@interface func (export \"fun2\") (param $an_int u32) (result $error f32) (result $doubled_it f32)))\n" ;
        pub fn document() -> wiggle::witx::Document {
            wiggle::witx::parse(DOC_TEXT).unwrap()
        }
    }
}
