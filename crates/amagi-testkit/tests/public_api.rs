#[cfg(feature = "catalog")]
#[test]
fn catalog_compatibility_surface_is_available() {
    let spec = amagi::catalog::platform_spec(amagi::Platform::Douyin);

    assert_eq!(spec.platform, amagi::Platform::Douyin);
    assert!(!amagi::catalog::all_platform_api_specs().is_empty());
    assert!(!amagi::catalog::all_platform_specs().is_empty());
    assert!(!amagi::catalog::operation_specs(amagi::Platform::Douyin).is_empty());
    assert!(!amagi::catalog::method_specs(amagi::Platform::Douyin).is_empty());
    assert_eq!(
        amagi::catalog::get_operation_route(amagi::Platform::Douyin, "search"),
        Some("/search")
    );
    assert_eq!(
        amagi::catalog::get_api_route(amagi::Platform::Douyin, "search"),
        Some("/search")
    );
}

#[cfg(feature = "catalog")]
#[test]
fn platform_order_and_api_base_paths_stay_stable() {
    let expected = [
        (amagi::Platform::Douyin, "douyin", "/api/douyin"),
        (amagi::Platform::Bilibili, "bilibili", "/api/bilibili"),
        (amagi::Platform::Kuaishou, "kuaishou", "/api/kuaishou"),
        (
            amagi::Platform::Xiaohongshu,
            "xiaohongshu",
            "/api/xiaohongshu",
        ),
        (amagi::Platform::Twitter, "twitter", "/api/twitter"),
    ];

    assert_eq!(amagi::Platform::ALL.len(), expected.len());

    for (index, (platform, name, api_base_path)) in expected.into_iter().enumerate() {
        assert_eq!(amagi::Platform::ALL[index], platform);
        assert_eq!(platform.as_str(), name);
        assert_eq!(platform.api_base_path(), api_base_path);
        assert_eq!(
            amagi::catalog::platform_spec(platform).api_base_path,
            api_base_path
        );
        assert_eq!(
            amagi::catalog::all_platform_specs()[index].platform,
            platform
        );
    }
}

#[cfg(feature = "client")]
#[test]
fn client_compatibility_surface_is_available() {
    let client = amagi::create_amagi_client(amagi::ClientOptions::default());

    let _client_type: amagi::AmagiClient = client.clone();
    let _catalog_spec = amagi::catalog::platform_spec(amagi::Platform::Bilibili);
    let _platform_client = client.platform(amagi::Platform::Bilibili);
    assert_event_type::<amagi::AmagiEvent>();
    assert_event_type::<amagi::AmagiEventType>();
    assert_event_type::<amagi::EventLogLevel>();
    assert_eq!(client.catalog().len(), amagi::Platform::ALL.len());
    assert_eq!(client.api_specs().len(), amagi::Platform::ALL.len());
}

#[cfg(feature = "client")]
fn assert_event_type<T>() {}

#[cfg(any(feature = "adapters", feature = "platforms"))]
#[test]
fn adapter_fetcher_surface_is_available() {
    let client = amagi::AdapterContext {
        platform: amagi::Platform::Douyin,
        cookie: None,
        request: amagi::RequestConfig::default(),
    };

    let _alias_client: amagi::PlatformClient = client.clone();
    let _from_root = amagi::douyin::DouyinFetcher::new(client.clone());
    let _from_adapters = amagi::adapters::douyin::DouyinFetcher::new(client.clone());
    let _from_platforms = amagi::platforms::douyin::DouyinFetcher::new(client);
}

#[cfg(any(feature = "adapters", feature = "platforms"))]
#[test]
fn adapter_spec_primary_and_compatibility_names_match() {
    let platform = amagi::Platform::Douyin;
    let spec: amagi::PlatformApiSpec = amagi::platform_api_spec(platform);
    let compat_spec: amagi::PlatformSpec = amagi::platform_spec(platform);
    let operations: &'static [amagi::ApiOperationSpec] = amagi::operation_specs(platform);
    let methods: &'static [amagi::ApiMethodSpec] = amagi::method_specs(platform);

    assert_eq!(spec, compat_spec);
    assert_eq!(operations, methods);
    assert_eq!(amagi::all_platform_api_specs(), amagi::all_platform_specs());

    let operation = amagi::find_operation(platform, "search").unwrap();
    let method = amagi::find_method(platform, "search").unwrap();

    assert_eq!(operation, method);
    assert_eq!(operation.fetcher_name, "searchContent");
    assert_eq!(
        amagi::get_fetcher_name(platform, operation.chinese_name),
        Some(operation.fetcher_name)
    );
    assert_eq!(
        amagi::get_english_method_name(platform, operation.chinese_name),
        Some(operation.fetcher_name)
    );
    assert_eq!(
        amagi::get_chinese_operation_name(platform, operation.fetcher_name),
        Some(operation.chinese_name)
    );
    assert_eq!(
        amagi::get_chinese_method_name(platform, operation.fetcher_name),
        Some(operation.chinese_name)
    );
    assert_eq!(
        amagi::get_operation_route(platform, "search"),
        Some("/search")
    );
    assert_eq!(amagi::get_api_route(platform, "search"), Some("/search"));
}

#[cfg(any(feature = "adapters", feature = "platforms"))]
#[test]
fn api_spec_json_field_names_stay_downstream_compatible() {
    let spec = amagi::platform_api_spec(amagi::Platform::Douyin);
    let value = serde_json::to_value(spec).unwrap();
    let first_operation = value["methods"].as_array().unwrap().first().unwrap();

    assert!(value.get("methods").is_some());
    assert!(value.get("operations").is_none());
    assert!(first_operation.get("method_key").is_some());
    assert!(first_operation.get("operation_key").is_none());
}

#[cfg(feature = "server")]
#[test]
fn server_compatibility_surface_is_available() {
    let config = amagi::config::ServeConfig {
        host: amagi::DEFAULT_HOST.to_owned(),
        port: amagi::DEFAULT_PORT,
        runtime_overrides: Default::default(),
    };
    let output = amagi::config::OutputConfig {
        locale: "en-US".to_owned(),
        format: amagi::config::OutputFormat::Text,
        file: None,
        pretty: false,
        append: false,
        create_parent_dirs: false,
    };
    let printer = amagi::output::Printer::new(output);

    let _serve = amagi::server::serve::<amagi::output::Printer>;
    let _client = amagi::create_amagi_client(amagi::ClientOptions::default());
    let _printer_type: amagi::output::Printer = printer;
    assert_eq!(config.base_url(), "http://127.0.0.1:4567");
}
