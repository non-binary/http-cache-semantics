#[macro_use(lazy_static)]
extern crate lazy_static;

#[allow(dead_code)]
mod http_cache_semantics {
    use std::collections::HashSet;

    lazy_static! {
        static ref STATUS_CODE_CACHEABLE_BY_DEFAULT: HashSet<i32> = {
            let mut set = HashSet::new();
            return set;
        };
    }

    lazy_static! {
        static ref UNDERSTOOD_STATUSES: HashSet<i32> = {
            let mut set = HashSet::new();
            return set;
        };
    }

    lazy_static! {
        static ref HOP_BY_HOP_HEADERS: HashSet<&'static str> = {
            let mut set = HashSet::new();
            return set;
        };
    }

    lazy_static! {
        static ref EXCLUDED_FROM_REVALIDATION_UPDATE: HashSet<&'static str> = {
            let mut set = HashSet::new();                    
            return set;
        };
    }

    fn parse_cache_control() -> () {
        unimplemented!();
    }

    fn format_cache_control() -> () {
        unimplemented!();
    }

    struct CachePolicy;
    impl CachePolicy {
        pub fn now() -> String {
            unimplemented!();
        }

        pub fn storable() {
            unimplemented!();
        }

        fn has_explicit_expiration() {
            unimplemented!();
        }

        fn assert_request_has_headers() {
            unimplemented!();
        }

        pub fn satisfies_without_revalidation() {
            unimplemented!();
        }

        fn request_matches() {
            unimplemented!();
        }

        fn allows_storing_authenticated() {
            unimplemented!();
        }

        fn vary_matches() {
            unimplemented!();
        }

        fn copy_without_hop_by_hop_headers() {
            unimplemented!();
        }

        pub fn response_headers() {
            unimplemented!();
        }

        pub fn date() {
            unimplemented!();
        }

        fn server_date() {
            unimplemented!();
        }

        pub fn age() {
            unimplemented!();
        }

        fn age_value() {
            unimplemented!();
        }
        
        pub fn max_age() {
            unimplemented!();
        }

        pub fn time_to_live() {
            unimplemented!();
        }

        pub fn stale() {
            unimplemented!();
        }

        pub fn from_object() {
            unimplemented!();
        }

        pub fn to_object() {
            unimplemented!();
        }

        pub fn revalidation_headers() {
            unimplemented!();
        }

        pub fn revalidated_policy() {
            unimplemented!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn assert_cached() {
        assert!(false);
    }
    
    #[test]
    fn test_ok_http_response_caching_by_response_code() {
        assert!(false);
    }
    
    #[test]
    fn test_default_expiration_date_fully_cached_for_less_than_24_hours() {
        assert!(false);
    }
    
    #[test]
    fn test_default_expiration_date_fully_cached_for_more_than_24_hours() {
        assert!(false);
    }
    
    #[test]
    fn test_max_age_in_the_past_with_date_header_but_no_last_modified_header() {
        assert!(false);
    }
    
    #[test]
    fn test_max_age_preferred_over_lower_shared_max_age() {
        assert!(false);
    }
    
    #[test]
    fn test_max_age_preferred_over_higher_max_age() {
        assert!(false);
    }
    
    fn request_method_not_cached() {
        assert!(false);
    }
    
    #[test]
    fn test_request_method_options_is_not_cached() {
        assert!(false);
    }
    
    #[test]
    fn test_request_method_put_is_not_cached() {
        assert!(false);
    }
    
    #[test]
    fn test_request_method_delete_is_not_cached() {
        assert!(false);
    }
    
    #[test]
    fn test_request_method_trace_is_not_cached() {
        assert!(false);
    }
    
    #[test]
    fn test_etag_and_expiration_date_in_the_future() {
        assert!(false);
    }
    
    #[test]
    fn test_client_side_no_store() {
        assert!(false);
    }
    
    #[test]
    fn test_request_max_age() {
        assert!(false);
    }
    
    #[test]
    fn test_request_min_fresh() {
        assert!(false);
    }
    
    #[test]
    fn test_request_max_stale() {
        assert!(false);
    }
    
    #[test]
    fn test_request_max_stale_not_honored_with_must_revalidate() {
        assert!(false);
    }
    
    #[test]
    fn test_get_headers_deletes_cached_100_level_warnings() {
        assert!(false);
    }
    
    #[test]
    fn test_do_not_cache_partial_response() {
        assert!(false);
    }
    
    fn format_date() {
        assert!(false);
    }
    
    #[test]
    fn test_no_store_kills_cache() {
        assert!(false);
    }
    
    #[test]
    fn test_post_not_cacheable_by_default() {
        assert!(false);
    }
    
    #[test]
    fn test_post_cacheable_explicitly() {
        assert!(false);
    }
    
    #[test]
    fn test_public_cacheable_auth_is_ok() {
        assert!(false);
    }
    
    #[test]
    fn test_proxy_cacheable_auth_is_ok() {
        assert!(false);
    }
    
    #[test]
    fn test_private_auth_is_ok() {
        assert!(false);
    }
    
    #[test]
    fn test_revalidate_auth_is_ok() {
        assert!(false);
    }
    
    #[test]
    fn test_auth_prevents_caching_by_default() {
        assert!(false);
    }
    
    #[test]
    fn test_simple_miss() {
        assert!(false);
    }
    
    #[test]
    fn test_simple_hit() {
        assert!(false);
    }
    
    #[test]
    fn test_weird_syntax() {
        assert!(false);
    }
    
    #[test]
    fn test_quoted_syntax() {
        assert!(false);
    }
    
    #[test]
    fn test_iis() {
        assert!(false);
    }
    
    #[test]
    fn test_pre_check_tolerated() {
        assert!(false);
    }
    
    #[test]
    fn test_pre_check_poison() {
        assert!(false);
    }
    
    #[test]
    fn test_pre_check_poison_undefined_header() {
        assert!(false);
    }
    
    #[test]
    fn test_cache_with_expires() {
        assert!(false);
    }
    
    #[test]
    fn test_cache_with_expires_always_relative_to_date() {
        assert!(false);
    }
    
    #[test]
    fn test_cache_expires_no_date() {
        assert!(false);
    }
    
    #[test]
    fn test_ages() {
        assert!(false);
    }
    
    #[test]
    fn test_age_can_make_stale() {
        assert!(false);
    }
    
    #[test]
    fn test_age_not_always_stale() {
        assert!(false);
    }
    
    #[test]
    fn test_bogus_age_ignored() {
        assert!(false);
    }
    
    #[test]
    fn test_cache_old_files() {
        assert!(false);
    }
    
    #[test]
    fn test_immutable_simple_hit() {
        assert!(false);
    }
    
    #[test]
    fn test_immutable_can_expire() {
        assert!(false);
    }
    
    #[test]
    fn test_cache_immutable_files() {
        assert!(false);
    }
    
    #[test]
    fn test_immutable_can_be_off() {
        assert!(false);
    }
    
    #[test]
    fn test_pragma_no_cache() {
        assert!(false);
    }
    
    #[test]
    fn test_blank_cache_control_and_pragma_no_cache() {
        assert!(false);
    }
    
    #[test]
    fn test_no_store() {
        assert!(false);
    }
    
    #[test]
    fn test_observe_private_cache() {
        assert!(false);
    }
    
    #[test]
    fn test_do_not_share_cookies() {
        assert!(false);
    }
    
    #[test]
    fn test_do_share_cookies_if_immutable() {
        assert!(false);
    }
    
    #[test]
    fn test_cache_explicitly_public_cookie() {
        assert!(false);
    }
    
    #[test]
    fn test_miss_max_age_equals_zero() {
        assert!(false);
    }
    
    #[test]
    fn test_uncacheable_503() {
        assert!(false);
    }
    
    #[test]
    fn test_cacheable_301() {
        assert!(false);
    }
    
    #[test]
    fn test_uncacheable_303() {
        assert!(false);
    }
    
    #[test]
    fn test_cacheable_303() {
        assert!(false);
    }
    
    #[test]
    fn test_uncacheable_412() {
        assert!(false);
    }
    
    #[test]
    fn test_expired_expires_cache_with_max_age() {
        assert!(false);
    }
    
    #[test]
    fn test_expired_expires_cached_with_s_maxage() {
        assert!(false);
    }
    
    #[test]
    fn test_max_age_wins_over_future_expires() {
        assert!(false);
    }
    
    #[test]
    fn test_remove_hop_headers() {
        assert!(false);
    }    
    
    fn assert_headers_passed() {
        assert!(false);
    }
    
    fn assert_no_validators() {
        assert!(false);
    }
    
    #[test]
    fn test_ok_if_method_changes_to_head() {
        assert!(false);
    }
    
    #[test]
    fn test_not_if_method_mismatch_other_than_head() {
        assert!(false);
    }
    
    #[test]
    fn test_not_if_url_mismatch() {
        assert!(false);
    }
    
    #[test]
    fn test_not_if_host_mismatch() {
        assert!(false);
    }
    
    #[test]
    fn test_not_if_vary_fields_prevent() {
        assert!(false);
    }
    
    #[test]
    fn test_when_entity_tag_validator_is_present() {
        assert!(false);
    }
    
    #[test]
    fn test_skips_weak_validators_on_post_2() {
        assert!(false);
    }
    
    #[test]
    fn test_merges_validators() {
        assert!(false);
    }
    
    #[test]
    fn test_when_last_modified_validator_is_present() {
        assert!(false);
    }
    
    #[test]
    fn test_not_without_validators() {
        assert!(false);
    }
    
    #[test]
    fn test_113_added() {
        assert!(false);
    }
    
    #[test]
    fn test_removes_warnings() {
        assert!(false);
    }
    
    #[test]
    fn test_must_contain_any_etag() {
        assert!(false);
    }
    
    #[test]
    fn test_merges_etags() {
        assert!(false);
    }
    
    #[test]
    fn test_should_send_the_last_modified_value() {
        assert!(false);
    }
    
    #[test]
    fn test_should_not_send_the_last_modified_value_for_post() {
        assert!(false);
    }
    
    #[test]
    fn test_should_not_send_the_last_modified_value_for_range_request() {
        assert!(false);
    }
    
    #[test]
    fn test_when_urls_match() {
        assert!(false);
    }
    
    #[test]
    fn test_when_expires_is_present() {
        assert!(false);
    }
    
    #[test]
    fn test_not_when_urls_mismatch() {
        assert!(false);
    }
    
    #[test]
    fn test_when_methods_match() {
        assert!(false);
    }
    
    #[test]
    fn test_not_when_hosts_mismatch() {
        assert!(false);
    }
    
    #[test]
    fn test_when_methods_match_head() {
        assert!(false);
    }
    
    #[test]
    fn test_not_when_methods_mismatch() {
        assert!(false);
    }
    
    #[test]
    fn test_not_when_methods_mismatch_head() {
        assert!(false);
    }
    
    #[test]
    fn test_not_when_proxy_revalidating() {
        assert!(false);
    }
    
    #[test]
    fn test_when_not_a_proxy_revalidating() {
        assert!(false);
    }
    
    #[test]
    fn test_not_when_no_cache_requesting() {
        assert!(false);
    }
    
    fn not_modified_response_headers() {
        assert!(false);
    }
    
    fn assert_updates() {
        assert!(false);
    }
    
    #[test]
    fn test_matching_etags_are_updated() {
        assert!(false);
    }
    
    #[test]
    fn test_matching_weak_etags_are_updated() {
        assert!(false);
    }
    
    #[test]
    fn test_matching_last_mod_are_updated() {
        assert!(false);
    }
    
    #[test]
    fn test_both_matching_are_updated() {
        assert!(false);
    }
    
    #[test]
    fn test_check_status() {
        assert!(false);
    }
    
    #[test]
    fn test_last_mod_ignored_if_etag_is_wrong() {
        assert!(false);
    }
    
    #[test]
    fn test_ignored_if_validator_is_missing() {
        assert!(false);
    }
    
    #[test]
    fn test_skips_update_of_content_length() {
        assert!(false);
    }
    
    #[test]
    fn test_ignored_if_validator_is_different() {
        assert!(false);
    }
    
    #[test]
    fn test_ignored_if_validator_does_not_match() {
        assert!(false);
    }
    
    #[test]
    fn test_vary_basic() {
        assert!(false);
    }
    
    #[test]
    fn test_asterisks_does_not_match() {
        assert!(false);
    }
    
    #[test]
    fn test_asterisks_is_stale() {
        assert!(false);
    }
    
    #[test]
    fn test_values_are_case_sensitive() {
        assert!(false);
    }
    
    #[test]
    fn test_irrelevant_headers_ignored() {
        assert!(false);
    }
    
    #[test]
    fn test_absence_is_meaningful() {
        assert!(false);
    }
    
    #[test]
    fn test_all_values_must_match() {
        assert!(false);
    }
    
    #[test]
    fn test_whitespace_is_okay() {
        assert!(false);
    }
    
    #[test]
    fn test_order_is_irrelevant() {
        assert!(false);
    }
    
    #[test]
    fn test_thaw_wrong_object() {
        assert!(false);
    }
    
    #[test]
    fn test_missing_headers() {
        assert!(false);
    }
    
    #[test]
    fn test_github_response_with_small_clock_skew() {
        assert!(false);
    }
    
}    
