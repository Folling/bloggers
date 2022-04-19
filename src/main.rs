#![feature(register_tool)]
#![feature(iter_intersperse)]
#![feature(main_separator_str)]
#![feature(let_chains)]
#![feature(stmt_expr_attributes)]
#![feature(is_some_with)]
#![register_tool(rust_analyzer)]
#![deny(
    ambiguous_associated_items,
    cenum_impl_drop_cast,
    coherence_leak_check,
    conflicting_repr_hints,
    const_err,
    const_evaluatable_unchecked,
    deprecated_cfg_attr_crate_type_name,
    deref_into_dyn_supertrait,
    forbidden_lint_groups,
    ill_formed_attribute_input,
    illegal_floating_point_literal_pattern,
    indirect_structural_match,
    deprecated_cfg_attr_crate_type_name,
    invalid_doc_attributes,
    invalid_type_param_default,
    late_bound_lifetime_arguments,
    legacy_derive_helpers,
    macro_expanded_macro_exports_accessed_by_absolute_paths,
    missing_fragment_specifier,
    mutable_borrow_reservation_conflict,
    nontrivial_structural_match,
    order_dependent_trait_objects,
    patterns_in_fns_without_body,
    pointer_structural_match,
    private_in_public,
    proc_macro_back_compat,
    proc_macro_derive_resolution_fallback,
    pub_use_of_private_extern_crate,
    semicolon_in_expressions_from_macros,
    soft_unstable,
    suspicious_auto_trait_impls,
    unaligned_references,
    uninhabited_static,
    unstable_name_collisions,
    unsupported_calling_conventions,
    where_clauses_object_safety,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    path_statements,
    redundant_semicolons,
    unused_allocation,
    unused_braces,
    unused_doc_comments,
    unused_imports,
    unused_mut,
    unused_parens,
    unused_unsafe,
    unused_attributes
)]
#![deny(
    clippy::allow_attributes_without_reason,
    clippy::almost_swapped,
    clippy::approx_constant,
    clippy::as_conversions,
    clippy::assertions_on_constants,
    clippy::assign_op_pattern,
    clippy::async_yields_async,
    clippy::await_holding_lock,
    clippy::await_holding_refcell_ref,
    clippy::bad_bit_mask,
    clippy::bind_instead_of_map,
    clippy::blacklisted_name,
    clippy::blanket_clippy_restriction_lints,
    clippy::blocks_in_if_conditions,
    clippy::bool_assert_comparison,
    clippy::bool_comparison,
    clippy::borrow_as_ptr,
    clippy::borrow_interior_mutable_const,
    clippy::borrowed_box,
    clippy::box_collection,
    clippy::boxed_local,
    clippy::branches_sharing_code,
    clippy::builtin_type_shadow,
    clippy::bytes_nth,
    clippy::cargo_common_metadata,
    clippy::chars_last_cmp,
    clippy::chars_next_cmp,
    clippy::clone_on_copy,
    clippy::clone_on_ref_ptr,
    clippy::cloned_instead_of_copied,
    clippy::cmp_nan,
    clippy::cmp_null,
    clippy::cmp_owned,
    clippy::comparison_chain,
    clippy::comparison_to_empty,
    clippy::crate_in_macro_def,
    clippy::debug_assert_with_mut_call,
    clippy::declare_interior_mutable_const,
    clippy::default_trait_access,
    clippy::default_union_representation,
    clippy::deprecated_cfg_attr,
    clippy::deprecated_semver,
    clippy::deref_addrof,
    clippy::derivable_impls,
    clippy::derive_hash_xor_eq,
    clippy::derive_ord_xor_partial_ord,
    clippy::double_comparisons,
    clippy::double_must_use,
    clippy::double_neg,
    clippy::drop_copy,
    clippy::drop_non_drop,
    clippy::drop_ref,
    clippy::drop_ref,
    clippy::duplicate_underscore_argument,
    clippy::duration_subsec,
    clippy::else_if_without_else,
    clippy::empty_enum,
    clippy::empty_line_after_outer_attr,
    clippy::empty_loop,
    clippy::empty_structs_with_brackets,
    clippy::enum_clike_unportable_variant,
    clippy::enum_glob_use,
    clippy::enum_variant_names,
    clippy::eq_op,
    clippy::equatable_if_let,
    clippy::erasing_op,
    clippy::err_expect,
    clippy::eval_order_dependence,
    clippy::excessive_precision,
    clippy::exit,
    clippy::expect_fun_call,
    clippy::expect_used,
    clippy::expl_impl_clone_on_copy,
    clippy::explicit_counter_loop,
    clippy::explicit_deref_methods,
    clippy::explicit_into_iter_loop,
    clippy::explicit_iter_loop,
    clippy::explicit_write,
    clippy::extend_with_drain,
    clippy::extra_unused_lifetimes,
    clippy::fallible_impl_from,
    clippy::filetype_is_file,
    clippy::filter_map_identity,
    clippy::filter_map_next,
    clippy::filter_next,
    clippy::flat_map_identity,
    clippy::flat_map_option,
    clippy::float_cmp,
    clippy::float_cmp_const,
    clippy::float_equality_without_abs,
    clippy::fn_address_comparisons,
    clippy::fn_params_excessive_bools,
    clippy::fn_to_numeric_cast,
    clippy::fn_to_numeric_cast_with_truncation,
    clippy::for_kv_map,
    clippy::forget_copy,
    clippy::forget_non_drop,
    clippy::forget_ref,
    clippy::format_in_format_args,
    clippy::from_iter_instead_of_collect,
    clippy::from_over_into,
    clippy::from_str_radix_10,
    clippy::future_not_send,
    clippy::get_last_with_len,
    clippy::get_unwrap,
    clippy::identity_op,
    clippy::if_let_mutex,
    clippy::if_same_then_else,
    clippy::if_then_some_else_none,
    clippy::ifs_same_cond,
    clippy::implicit_clone,
    clippy::implicit_hasher,
    clippy::implicit_saturating_sub,
    clippy::imprecise_flops,
    clippy::inconsistent_digit_grouping,
    clippy::inconsistent_struct_constructor,
    clippy::index_refutable_slice,
    clippy::indexing_slicing,
    clippy::ineffective_bit_mask,
    clippy::inefficient_to_string,
    clippy::infallible_destructuring_match,
    clippy::infinite_iter,
    clippy::inherent_to_string,
    clippy::inherent_to_string_shadow_display,
    clippy::init_numbered_fields,
    clippy::inline_fn_without_body,
    clippy::inspect_for_each,
    clippy::int_plus_one,
    clippy::into_iter_on_ref,
    clippy::invalid_null_ptr_usage,
    clippy::invalid_regex,
    clippy::invalid_upcast_comparisons,
    clippy::invisible_characters,
    clippy::invalid_upcast_comparisons,
    clippy::items_after_statements,
    clippy::iter_cloned_collect,
    clippy::iter_count,
    clippy::iter_next_loop,
    clippy::iter_next_slice,
    clippy::iter_not_returning_iterator,
    clippy::iter_nth,
    clippy::iter_nth_zero,
    clippy::iter_overeager_cloned,
    clippy::iter_skip_next,
    clippy::iter_with_drain,
    clippy::iterator_step_by_zero,
    clippy::just_underscores_and_digits,
    clippy::large_const_arrays,
    clippy::large_digit_groups,
    clippy::just_underscores_and_digits,
    clippy::len_without_is_empty,
    clippy::len_zero,
    clippy::let_and_return,
    clippy::let_underscore_drop,
    clippy::let_underscore_lock,
    clippy::let_underscore_must_use,
    clippy::let_unit_value,
    clippy::let_underscore_must_use,
    clippy::logic_bug,
    clippy::lossy_float_literal,
    clippy::macro_use_imports,
    clippy::main_recursion,
    clippy::manual_assert,
    clippy::manual_async_fn,
    clippy::manual_bits,
    clippy::manual_filter_map,
    clippy::manual_find_map,
    clippy::manual_flatten,
    clippy::manual_map,
    clippy::manual_memcpy,
    clippy::manual_non_exhaustive,
    clippy::manual_ok_or,
    clippy::manual_range_contains,
    clippy::manual_saturating_arithmetic,
    clippy::manual_split_once,
    clippy::manual_str_repeat,
    clippy::manual_strip,
    clippy::manual_swap,
    clippy::manual_unwrap_or,
    clippy::many_single_char_names,
    clippy::map_clone,
    clippy::map_collect_result_unit,
    clippy::map_entry,
    clippy::map_flatten,
    clippy::map_identity,
    clippy::map_unwrap_or,
    clippy::match_as_ref,
    clippy::match_bool,
    clippy::match_like_matches_macro,
    clippy::match_overlapping_arm,
    clippy::match_ref_pats,
    clippy::match_result_ok,
    clippy::match_same_arms,
    clippy::match_single_binding,
    clippy::match_str_case_mismatch,
    clippy::match_wildcard_for_single_variants,
    clippy::mem_replace_option_with_none,
    clippy::mem_replace_with_default,
    clippy::min_max,
    clippy::mismatched_target_os,
    clippy::missing_inline_in_public_items,
    clippy::missing_spin_loop,
    clippy::mixed_case_hex_literals,
    clippy::module_inception,
    clippy::module_name_repetitions,
    clippy::modulo_one,
    clippy::mut_from_ref,
    clippy::mut_mut,
    clippy::mut_mutex_lock,
    clippy::mutable_key_type,
    clippy::mutex_atomic,
    clippy::mutex_integer,
    clippy::naive_bytecount,
    clippy::needless_arbitrary_self_type,
    clippy::needless_bitwise_bool,
    clippy::needless_bool,
    clippy::needless_borrow,
    clippy::needless_borrowed_reference,
    clippy::needless_collect,
    clippy::needless_continue,
    clippy::needless_doctest_main,
    clippy::needless_for_each,
    clippy::needless_late_init,
    clippy::needless_lifetimes,
    clippy::needless_match,
    clippy::needless_option_as_deref,
    clippy::needless_pass_by_value,
    clippy::needless_question_mark,
    clippy::needless_range_loop,
    clippy::needless_return,
    clippy::needless_splitn,
    clippy::needless_update,
    clippy::neg_cmp_op_on_partial_ord,
    clippy::neg_multiply,
    clippy::never_loop,
    clippy::new_without_default,
    clippy::no_effect,
    clippy::no_effect_underscore_binding,
    clippy::non_octal_unix_permissions,
    clippy::nonsensical_open_options,
    clippy::nonstandard_macro_braces,
    clippy::not_unsafe_ptr_arg_deref,
    clippy::octal_escapes,
    clippy::ok_expect,
    clippy::only_used_in_recursion,
    clippy::op_ref,
    clippy::option_as_ref_deref,
    clippy::option_env_unwrap,
    clippy::option_filter_map,
    clippy::option_if_let_else,
    clippy::option_map_or_none,
    clippy::option_map_unit_fn,
    clippy::option_option,
    clippy::or_fun_call,
    clippy::or_then_unwrap,
    clippy::out_of_bounds_indexing,
    clippy::overflow_check_conditional,
    clippy::panic_in_result_fn,
    clippy::panicking_unwrap,
    clippy::partialeq_ne_impl,
    clippy::path_buf_push_overwrite,
    clippy::pattern_type_mismatch,
    clippy::precedence,
    clippy::print_in_format_impl,
    clippy::print_literal,
    clippy::print_with_newline,
    clippy::println_empty_string,
    clippy::ptr_arg,
    clippy::ptr_eq,
    clippy::ptr_offset_with_cast,
    clippy::question_mark,
    clippy::range_minus_one,
    clippy::range_plus_one,
    clippy::range_zip_with_len,
    clippy::rc_buffer,
    clippy::rc_mutex,
    clippy::recursive_format_impl,
    clippy::redundant_allocation,
    clippy::redundant_clone,
    clippy::redundant_closure,
    clippy::redundant_closure_call,
    clippy::redundant_closure_for_method_calls,
    clippy::redundant_else,
    clippy::redundant_feature_names,
    clippy::redundant_pattern,
    clippy::redundant_pattern_matching,
    clippy::redundant_pub_crate,
    clippy::redundant_slicing,
    clippy::redundant_static_lifetimes,
    clippy::ref_binding_to_reference,
    clippy::ref_option_ref,
    clippy::repeat_once,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::result_map_or_into_option,
    clippy::result_map_unit_fn,
    clippy::result_unit_err,
    clippy::return_self_not_must_use,
    clippy::reversed_empty_ranges,
    clippy::same_functions_in_if_condition,
    clippy::same_item_push,
    clippy::search_is_some,
    clippy::self_assignment,
    clippy::self_named_constructors,
    clippy::self_named_module_files,
    clippy::semicolon_if_nothing_returned,
    clippy::serde_api_misuse,
    clippy::should_implement_trait,
    clippy::single_char_add_str,
    clippy::single_char_pattern,
    clippy::single_component_path_imports,
    clippy::single_element_loop,
    clippy::single_match,
    clippy::single_match_else,
    clippy::size_of_in_element_count,
    clippy::skip_while_next,
    clippy::slow_vector_initialization,
    clippy::stable_sort_primitive,
    clippy::str_to_string,
    clippy::string_add,
    clippy::string_add_assign,
    clippy::string_extend_chars,
    clippy::string_from_utf8_as_bytes,
    clippy::string_lit_as_bytes,
    clippy::string_slice,
    clippy::string_to_string,
    clippy::strlen_on_c_strings,
    clippy::struct_excessive_bools,
    clippy::suboptimal_flops,
    clippy::suspicious_assignment_formatting,
    clippy::suspicious_else_formatting,
    clippy::suspicious_splitn,
    clippy::suspicious_unary_op_formatting,
    clippy::tabs_in_doc_comments,
    clippy::temporary_assignment,
    clippy::to_digit_is_some,
    clippy::to_string_in_format_args,
    clippy::toplevel_ref_arg,
    clippy::trailing_empty_array,
    clippy::trait_duplication_in_bounds,
    clippy::transmute_bytes_to_str,
    clippy::transmute_float_to_int,
    clippy::transmute_int_to_bool,
    clippy::transmute_int_to_char,
    clippy::transmute_int_to_float,
    clippy::transmute_num_to_bytes,
    clippy::transmute_ptr_to_ptr,
    clippy::transmute_ptr_to_ref,
    clippy::transmutes_expressible_as_ptr_casts,
    clippy::transmuting_null,
    clippy::trivial_regex,
    clippy::trivially_copy_pass_by_ref,
    clippy::try_err,
    clippy::type_repetition_in_bounds,
    clippy::undropped_manually_drops,
    clippy::unicode_not_nfc,
    clippy::unit_arg,
    clippy::unit_hash,
    clippy::unit_return_expecting_ord,
    clippy::unnecessary_cast,
    clippy::unnecessary_filter_map,
    clippy::unnecessary_find_map,
    clippy::unnecessary_fold,
    clippy::unnecessary_join,
    clippy::unnecessary_lazy_evaluations,
    clippy::unnecessary_mut_passed,
    clippy::unnecessary_operation,
    clippy::unnecessary_self_imports,
    clippy::unnecessary_sort_by,
    clippy::unnecessary_to_owned,
    clippy::unnecessary_unwrap,
    clippy::unnecessary_wraps,
    clippy::unneeded_field_pattern,
    clippy::unneeded_wildcard_pattern,
    clippy::unnested_or_patterns,
    clippy::unreadable_literal,
    clippy::unseparated_literal_suffix,
    clippy::unused_async,
    clippy::unused_io_amount,
    clippy::unused_self,
    clippy::unused_unit,
    clippy::unusual_byte_groupings,
    clippy::unwrap_in_result,
    clippy::unwrap_or_else_default,
    clippy::unwrap_used,
    clippy::upper_case_acronyms,
    clippy::use_self,
    clippy::used_underscore_binding,
    clippy::useless_asref,
    clippy::useless_attribute,
    clippy::useless_conversion,
    clippy::useless_format,
    clippy::useless_let_if_seq,
    clippy::useless_transmute,
    clippy::useless_vec,
    clippy::vec_init_then_push,
    clippy::vec_resize_to_zero,
    clippy::verbose_bit_mask,
    clippy::verbose_file_reads,
    clippy::vtable_address_comparisons,
    clippy::while_immutable_condition,
    clippy::while_let_loop,
    clippy::while_let_on_iterator,
    clippy::wildcard_dependencies,
    clippy::wildcard_imports,
    clippy::wildcard_in_or_patterns,
    clippy::write_literal,
    clippy::write_with_newline,
    clippy::writeln_empty_string,
    clippy::write_with_newline,
    clippy::wrong_self_convention,
    clippy::wrong_transmute,
    clippy::zero_divided_by_zero,
    clippy::zero_prefixed_literal,
    clippy::zero_ptr,
    clippy::zero_sized_map_values,
    clippy::zst_offset
)]
#![warn(
    clippy::cast_abs_to_unsigned,
    clippy::cast_enum_constructor,
    clippy::cast_enum_truncation,
    clippy::cast_ptr_alignment,
    clippy::cast_slice_different_sizes,
    clippy::cast_lossless,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_ref_to_mut,
    clippy::cast_sign_loss,
    clippy::char_lit_as_u8,
    clippy::checked_conversions,
    clippy::clone_double_ref,
    clippy::cognitive_complexity,
    clippy::collapsible_else_if,
    clippy::collapsible_if,
    clippy::collapsible_match,
    clippy::create_dir,
    clippy::crosspointer_transmute,
    clippy::dbg_macro,
    clippy::decimal_literal_representation,
    clippy::default_numeric_fallback,
    clippy::diverging_sub_expression,
    clippy::doc_markdown,
    clippy::double_parens,
    clippy::exhaustive_enums,
    clippy::exhaustive_structs,
    clippy::fn_to_numeric_cast_any,
    clippy::if_not_else,
    clippy::imprecise_flops,
    clippy::integer_division,
    clippy::large_enum_variant,
    clippy::large_stack_arrays,
    clippy::large_types_passed_by_value,
    clippy::linkedlist,
    clippy::map_err_ignore,
    clippy::match_on_vec_items,
    clippy::match_wild_err_arm,
    clippy::maybe_infinite_iter,
    clippy::mem_forget,
    clippy::mem_replace_with_uninit,
    clippy::misrefactored_assign_op,
    clippy::missing_const_for_fn,
    clippy::mistyped_literal_suffixes,
    clippy::mut_range_bound,
    clippy::negative_feature_names,
    clippy::new_ret_no_self,
    clippy::possible_missing_comma,
    clippy::print_stderr,
    clippy::print_stdout,
    clippy::ptr_as_ptr,
    clippy::redundant_field_names,
    clippy::same_name_method,
    clippy::short_circuit_statement,
    clippy::similar_names,
    clippy::suspicious_arithmetic_impl,
    clippy::suspicious_map,
    clippy::suspicious_op_assign_impl,
    clippy::suspicious_operation_groupings,
    clippy::todo,
    clippy::too_many_arguments,
    clippy::too_many_lines,
    clippy::transmute_undefined_repr,
    clippy::type_complexity,
    clippy::unimplemented,
    clippy::uninit_assumed_init,
    clippy::uninit_vec,
    clippy::unimplemented,
    clippy::unreachable,
    clippy::unsafe_derive_deserialize,
    clippy::unsafe_removed_from_name,
    clippy::unsound_collection_transmute,
    clippy::use_debug,
    clippy::vec_box,
    clippy::wildcard_enum_match_arm
)]

mod generation;
mod markdown;

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{anyhow, bail, Ok, Result};

use log::{info, warn, Level};

fn sanitise_path<P: AsRef<Path>>(name: &'static str, path: P, dir: bool, must_exist: bool) -> Result<(bool, PathBuf)> {
    let path = path.as_ref();

    let exists = match path.metadata() {
        std::io::Result::Ok(data) => {
            if dir {
                if !data.is_dir() {
                    bail!("{} isn't a directory", name);
                }
            } else if data.is_dir() {
                bail!("{} is a directory", name);
            } else {
                // all is well
            }

            true
        }
        std::io::Result::Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                if must_exist {
                    bail!("{} doesn't exist", name);
                }

                false
            } else {
                bail!("unable to verify whether {} exists", name);
            }
        }
    };

    info!("{} resolves to {}", name, path.display());

    Ok((exists, path.to_path_buf()))
}

fn main() -> Result<()> {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Trace)
        .write_style(env_logger::WriteStyle::Always)
        .format(|buf, record| {
            let color = match record.level() {
                Level::Trace => env_logger::fmt::Color::Rgb(35, 175, 200),
                Level::Debug => env_logger::fmt::Color::Rgb(35, 200, 150),
                Level::Info => env_logger::fmt::Color::Rgb(200, 200, 35),
                Level::Warn => env_logger::fmt::Color::Rgb(200, 125, 35),
                Level::Error => env_logger::fmt::Color::Rgb(200, 35, 35),
            };
            let mut style = buf.style();
            style.set_color(color);

            write!(buf, "[{:<5}] ", style.value(record.level()))?;

            style.set_color(env_logger::fmt::Color::Rgb(184, 184, 184));

            write!(
                buf,
                "{} {}{}{}{}",
                style.value(buf.timestamp()),
                style.value(record.module_path().unwrap_or("-")),
                style.value("::"),
                style.value(record.file().unwrap_or("-")),
                style.value("@")
            )?;

            if let Some(line) = record.line() {
                write!(buf, "{}{}", style.value(line), style.value(": "))?;
            } else {
                write!(buf, "{}", style.value("-: "))?;
            }

            writeln!(buf, "{}", record.args())
        })
        .parse_env(env_logger::Env::default())
        .init();

    let args: Vec<_> = std::env::args().collect();

    let first_arg = args
        .get(1)
        .ok_or_else(|| anyhow!("first argument must be the input directory's path"))?;
    let (_, input_path) = sanitise_path("input path", first_arg, true, true)?;

    let second_arg = args
        .get(2)
        .ok_or_else(|| anyhow!("second argument must be the output directory's path"))?;
    let (output_exists, output_path) = sanitise_path("output path", second_arg, true, false)?;

    // recreate output directory from scratch
    if output_exists {
        info!("output path exists, removing");
        std::fs::remove_dir_all(&output_path)?;
    }

    info!("creating output directory");

    std::fs::create_dir_all(&output_path)?;

    let (_, template) = sanitise_path("html template path", input_path.join("template.html"), false, true)?;

    let content = std::fs::read_to_string(template)?;

    let content_needle = "<div id='content'>";

    // the following isn't optimal, we're searching through the string twice instead of once
    // there doesn't appear to be a standard function that covers this usecase and writing up a custom function
    // doesn't seem worth the effort

    let content_idx = content
        .rfind(content_needle)
        .and_then(|v| v.checked_add(content_needle.len()))
        .ok_or_else(|| anyhow!("template.html doesn't contain content needle or integer overflow occurred"))?;

    info!("found content insertion point at idx {}", content_idx);

    // since we're using find we are guaranteed to get the proper byte index
    #[allow(clippy::indexing_slicing, clippy::string_slice)]
    let first = &content[0..content_idx];
    #[allow(clippy::indexing_slicing, clippy::string_slice)]
    let last = &content[content_idx..];

    let mut files = Vec::with_capacity(1024);

    files.extend(std::fs::read_dir(&input_path)?.filter(|v| v.is_ok_and(|v| !v.path().ends_with("template.html"))));

    while let Some(file) = files.pop() {
        let path = file?.path();

        info!("generating content for {}", path.display());

        let new_path = path.strip_prefix(&input_path)?;

        let mut output_new_path = output_path.join(new_path);

        if path.is_dir() {
            if let Some(".sass-cache" | ".git") = path.file_name().and_then(std::ffi::OsStr::to_str) {
                info!("ignoring .sass-cache & .git");
            } else {
                info!(
                    "path is directory, creating dir {} and appending files to stack",
                    output_new_path.display()
                );
                // no need to call create_dir_all here as all previous dirs are guaranteed to have been created beforehand
                #[allow(clippy::create_dir)]
                std::fs::create_dir(output_new_path)?;
                files.extend(std::fs::read_dir(path)?);
            }
        } else {
            info!("path is a file");

            let extension = path.extension().and_then(std::ffi::OsStr::to_str);

            match extension {
                Some("bmd") => {
                    info!("found bmd file, parsing and generating HTML");
                    let content = std::fs::read_to_string(&path)?;
                    output_new_path.set_file_name("index.html");
                    let mut file = std::fs::File::create(output_new_path)?;

                    write!(&mut file, "{}{}{}", first, markdown::parser::parse(content)?, last)?;
                }
                // generate css files for scss files
                Some("scss") => {
                    println!(
                        "{:?}",
                        path.file_name()
                            .and_then(std::ffi::OsStr::to_str)
                            .is_some_and(|v| v.starts_with('_'))
                    );
                    info!("encountered scss file");

                    // ignore partials
                    if path
                        .file_name()
                        .and_then(std::ffi::OsStr::to_str)
                        .is_some_and(|v| v.starts_with('_'))
                    {
                        info!("scss file is a partial, ignoring");
                    } else {
                        info!("path is scss, executing sass to generate css file");

                        let output = Command::new("sass")
                            .args(
                                path.to_str()
                                    .ok_or_else(|| anyhow!("unable to convert path {} to str", path.display())),
                            )
                            .output()?;

                        if !output.status.success() {
                            bail!(
                                "unable to generate sass content for {}. Message: {}",
                                path.display(),
                                String::from_utf8(output.stderr)?
                            )
                        }

                        std::fs::write(output_new_path.with_extension("css"), String::from_utf8(output.stdout)?)?;
                    }
                }
                v => {
                    #[allow(clippy::option_if_let_else)]
                    if let Some(v) = v {
                        info!("extension {} is unknown, copying file to {}", v, output_new_path.display());
                    } else {
                        info!("no extension exists, copying file to {}", output_new_path.display());
                    }

                    std::fs::copy(path, output_new_path)?;
                }
            }
        }
    }

    Ok(())
}
