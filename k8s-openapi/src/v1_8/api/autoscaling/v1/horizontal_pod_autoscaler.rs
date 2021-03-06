// Generated from definition io.k8s.api.autoscaling.v1.HorizontalPodAutoscaler

/// configuration of a horizontal pod autoscaler.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HorizontalPodAutoscaler {
    /// Standard object metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
    pub metadata: Option<crate::v1_8::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// behaviour of autoscaler. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status.
    pub spec: Option<crate::v1_8::api::autoscaling::v1::HorizontalPodAutoscalerSpec>,

    /// current information about the autoscaler.
    pub status: Option<crate::v1_8::api::autoscaling::v1::HorizontalPodAutoscalerStatus>,
}

// Begin autoscaling/v1/HorizontalPodAutoscaler

// Generated from operation createAutoscalingV1NamespacedHorizontalPodAutoscaler

impl HorizontalPodAutoscaler {
    /// create a HorizontalPodAutoscaler
    ///
    /// Use [`CreateNamespacedHorizontalPodAutoscalerResponse`](./enum.CreateNamespacedHorizontalPodAutoscalerResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn create_namespaced_horizontal_pod_autoscaler(
        namespace: &str,
        body: &crate::v1_8::api::autoscaling::v1::HorizontalPodAutoscaler,
        optional: CreateNamespacedHorizontalPodAutoscalerOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let CreateNamespacedHorizontalPodAutoscalerOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/autoscaling/v1/namespaces/{namespace}/horizontalpodautoscalers?", namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::post(__url);
        let __body = serde_json::to_vec(&body).map_err(crate::RequestError::Json)?;
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`HorizontalPodAutoscaler::create_namespaced_horizontal_pod_autoscaler`](./struct.HorizontalPodAutoscaler.html#method.create_namespaced_horizontal_pod_autoscaler)
#[derive(Clone, Copy, Debug, Default)]
pub struct CreateNamespacedHorizontalPodAutoscalerOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`HorizontalPodAutoscaler::create_namespaced_horizontal_pod_autoscaler`](./struct.HorizontalPodAutoscaler.html#method.create_namespaced_horizontal_pod_autoscaler)
#[derive(Debug)]
pub enum CreateNamespacedHorizontalPodAutoscalerResponse {
    Ok(crate::v1_8::api::autoscaling::v1::HorizontalPodAutoscaler),
    Unauthorized,
    Other,
}

impl crate::Response for CreateNamespacedHorizontalPodAutoscalerResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateNamespacedHorizontalPodAutoscalerResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((CreateNamespacedHorizontalPodAutoscalerResponse::Unauthorized, 0)),
            _ => Ok((CreateNamespacedHorizontalPodAutoscalerResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteAutoscalingV1CollectionNamespacedHorizontalPodAutoscaler

impl HorizontalPodAutoscaler {
    /// delete collection of HorizontalPodAutoscaler
    ///
    /// Use [`DeleteCollectionNamespacedHorizontalPodAutoscalerResponse`](./enum.DeleteCollectionNamespacedHorizontalPodAutoscalerResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn delete_collection_namespaced_horizontal_pod_autoscaler(
        namespace: &str,
        optional: DeleteCollectionNamespacedHorizontalPodAutoscalerOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let DeleteCollectionNamespacedHorizontalPodAutoscalerOptional {
            continue_,
            field_selector,
            include_uninitialized,
            label_selector,
            limit,
            pretty,
            resource_version,
            timeout_seconds,
            watch,
        } = optional;
        let __url = format!("/apis/autoscaling/v1/namespaces/{namespace}/horizontalpodautoscalers?", namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(continue_) = continue_ {
            __query_pairs.append_pair("continue", continue_);
        }
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", field_selector);
        }
        if let Some(include_uninitialized) = include_uninitialized {
            __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", label_selector);
        }
        if let Some(limit) = limit {
            __query_pairs.append_pair("limit", &limit.to_string());
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        if let Some(resource_version) = resource_version {
            __query_pairs.append_pair("resourceVersion", resource_version);
        }
        if let Some(timeout_seconds) = timeout_seconds {
            __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
        }
        if let Some(watch) = watch {
            __query_pairs.append_pair("watch", &watch.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::delete(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`HorizontalPodAutoscaler::delete_collection_namespaced_horizontal_pod_autoscaler`](./struct.HorizontalPodAutoscaler.html#method.delete_collection_namespaced_horizontal_pod_autoscaler)
#[derive(Clone, Copy, Debug, Default)]
pub struct DeleteCollectionNamespacedHorizontalPodAutoscalerOptional<'a> {
    /// The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server the server will respond with a 410 ResourceExpired error indicating the client must restart their list without the continue field. This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
    pub continue_: Option<&'a str>,
    /// A selector to restrict the list of returned objects by their fields. Defaults to everything.
    pub field_selector: Option<&'a str>,
    /// If true, partially initialized resources are included in the response.
    pub include_uninitialized: Option<bool>,
    /// A selector to restrict the list of returned objects by their labels. Defaults to everything.
    pub label_selector: Option<&'a str>,
    /// limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.
    ///
    /// The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
    pub limit: Option<i64>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
    pub resource_version: Option<&'a str>,
    /// Timeout for the list/watch call.
    pub timeout_seconds: Option<i64>,
    /// Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub watch: Option<bool>,
}

/// Parses the HTTP response of [`HorizontalPodAutoscaler::delete_collection_namespaced_horizontal_pod_autoscaler`](./struct.HorizontalPodAutoscaler.html#method.delete_collection_namespaced_horizontal_pod_autoscaler)
#[derive(Debug)]
pub enum DeleteCollectionNamespacedHorizontalPodAutoscalerResponse {
    OkStatus(crate::v1_8::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_8::api::autoscaling::v1::HorizontalPodAutoscaler),
    Unauthorized,
    Other,
}

impl crate::Response for DeleteCollectionNamespacedHorizontalPodAutoscalerResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result: serde_json::Map<String, serde_json::Value> = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                let is_status = match result.get("kind") {
                    Some(serde_json::Value::String(s)) if s == "Status" => true,
                    _ => false,
                };
                if is_status {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteCollectionNamespacedHorizontalPodAutoscalerResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteCollectionNamespacedHorizontalPodAutoscalerResponse::OkValue(result), buf.len()))
                }
            },
            http::StatusCode::UNAUTHORIZED => Ok((DeleteCollectionNamespacedHorizontalPodAutoscalerResponse::Unauthorized, 0)),
            _ => Ok((DeleteCollectionNamespacedHorizontalPodAutoscalerResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteAutoscalingV1NamespacedHorizontalPodAutoscaler

impl HorizontalPodAutoscaler {
    /// delete a HorizontalPodAutoscaler
    ///
    /// Use [`DeleteNamespacedHorizontalPodAutoscalerResponse`](./enum.DeleteNamespacedHorizontalPodAutoscalerResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the HorizontalPodAutoscaler
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn delete_namespaced_horizontal_pod_autoscaler(
        name: &str,
        namespace: &str,
        optional: DeleteNamespacedHorizontalPodAutoscalerOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let DeleteNamespacedHorizontalPodAutoscalerOptional {
            grace_period_seconds,
            orphan_dependents,
            pretty,
            propagation_policy,
        } = optional;
        let __url = format!("/apis/autoscaling/v1/namespaces/{namespace}/horizontalpodautoscalers/{name}?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(grace_period_seconds) = grace_period_seconds {
            __query_pairs.append_pair("gracePeriodSeconds", &grace_period_seconds.to_string());
        }
        if let Some(orphan_dependents) = orphan_dependents {
            __query_pairs.append_pair("orphanDependents", &orphan_dependents.to_string());
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        if let Some(propagation_policy) = propagation_policy {
            __query_pairs.append_pair("propagationPolicy", propagation_policy);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::delete(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`HorizontalPodAutoscaler::delete_namespaced_horizontal_pod_autoscaler`](./struct.HorizontalPodAutoscaler.html#method.delete_namespaced_horizontal_pod_autoscaler)
#[derive(Clone, Copy, Debug, Default)]
pub struct DeleteNamespacedHorizontalPodAutoscalerOptional<'a> {
    /// The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately.
    pub grace_period_seconds: Option<i64>,
    /// Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the "orphan" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both.
    pub orphan_dependents: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy.
    pub propagation_policy: Option<&'a str>,
}

/// Parses the HTTP response of [`HorizontalPodAutoscaler::delete_namespaced_horizontal_pod_autoscaler`](./struct.HorizontalPodAutoscaler.html#method.delete_namespaced_horizontal_pod_autoscaler)
#[derive(Debug)]
pub enum DeleteNamespacedHorizontalPodAutoscalerResponse {
    OkStatus(crate::v1_8::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_8::api::autoscaling::v1::HorizontalPodAutoscaler),
    Unauthorized,
    Other,
}

impl crate::Response for DeleteNamespacedHorizontalPodAutoscalerResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result: serde_json::Map<String, serde_json::Value> = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                let is_status = match result.get("kind") {
                    Some(serde_json::Value::String(s)) if s == "Status" => true,
                    _ => false,
                };
                if is_status {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteNamespacedHorizontalPodAutoscalerResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteNamespacedHorizontalPodAutoscalerResponse::OkValue(result), buf.len()))
                }
            },
            http::StatusCode::UNAUTHORIZED => Ok((DeleteNamespacedHorizontalPodAutoscalerResponse::Unauthorized, 0)),
            _ => Ok((DeleteNamespacedHorizontalPodAutoscalerResponse::Other, 0)),
        }
    }
}

// Generated from operation listAutoscalingV1HorizontalPodAutoscalerForAllNamespaces

impl HorizontalPodAutoscaler {
    /// list or watch objects of kind HorizontalPodAutoscaler
    ///
    /// Use [`ListHorizontalPodAutoscalerForAllNamespacesResponse`](./enum.ListHorizontalPodAutoscalerForAllNamespacesResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn list_horizontal_pod_autoscaler_for_all_namespaces(
        optional: ListHorizontalPodAutoscalerForAllNamespacesOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ListHorizontalPodAutoscalerForAllNamespacesOptional {
            continue_,
            field_selector,
            include_uninitialized,
            label_selector,
            limit,
            pretty,
            resource_version,
            timeout_seconds,
            watch,
        } = optional;
        let __url = "/apis/autoscaling/v1/horizontalpodautoscalers?".to_string();
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(continue_) = continue_ {
            __query_pairs.append_pair("continue", continue_);
        }
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", field_selector);
        }
        if let Some(include_uninitialized) = include_uninitialized {
            __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", label_selector);
        }
        if let Some(limit) = limit {
            __query_pairs.append_pair("limit", &limit.to_string());
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        if let Some(resource_version) = resource_version {
            __query_pairs.append_pair("resourceVersion", resource_version);
        }
        if let Some(timeout_seconds) = timeout_seconds {
            __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
        }
        if let Some(watch) = watch {
            __query_pairs.append_pair("watch", &watch.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`HorizontalPodAutoscaler::list_horizontal_pod_autoscaler_for_all_namespaces`](./struct.HorizontalPodAutoscaler.html#method.list_horizontal_pod_autoscaler_for_all_namespaces)
#[derive(Clone, Copy, Debug, Default)]
pub struct ListHorizontalPodAutoscalerForAllNamespacesOptional<'a> {
    /// The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server the server will respond with a 410 ResourceExpired error indicating the client must restart their list without the continue field. This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
    pub continue_: Option<&'a str>,
    /// A selector to restrict the list of returned objects by their fields. Defaults to everything.
    pub field_selector: Option<&'a str>,
    /// If true, partially initialized resources are included in the response.
    pub include_uninitialized: Option<bool>,
    /// A selector to restrict the list of returned objects by their labels. Defaults to everything.
    pub label_selector: Option<&'a str>,
    /// limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.
    ///
    /// The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
    pub limit: Option<i64>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
    pub resource_version: Option<&'a str>,
    /// Timeout for the list/watch call.
    pub timeout_seconds: Option<i64>,
    /// Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub watch: Option<bool>,
}

/// Parses the HTTP response of [`HorizontalPodAutoscaler::list_horizontal_pod_autoscaler_for_all_namespaces`](./struct.HorizontalPodAutoscaler.html#method.list_horizontal_pod_autoscaler_for_all_namespaces)
#[derive(Debug)]
pub enum ListHorizontalPodAutoscalerForAllNamespacesResponse {
    Ok(crate::v1_8::api::autoscaling::v1::HorizontalPodAutoscalerList),
    Unauthorized,
    Other,
}

impl crate::Response for ListHorizontalPodAutoscalerForAllNamespacesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ListHorizontalPodAutoscalerForAllNamespacesResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ListHorizontalPodAutoscalerForAllNamespacesResponse::Unauthorized, 0)),
            _ => Ok((ListHorizontalPodAutoscalerForAllNamespacesResponse::Other, 0)),
        }
    }
}

// Generated from operation listAutoscalingV1NamespacedHorizontalPodAutoscaler

impl HorizontalPodAutoscaler {
    /// list or watch objects of kind HorizontalPodAutoscaler
    ///
    /// Use [`ListNamespacedHorizontalPodAutoscalerResponse`](./enum.ListNamespacedHorizontalPodAutoscalerResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn list_namespaced_horizontal_pod_autoscaler(
        namespace: &str,
        optional: ListNamespacedHorizontalPodAutoscalerOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ListNamespacedHorizontalPodAutoscalerOptional {
            continue_,
            field_selector,
            include_uninitialized,
            label_selector,
            limit,
            pretty,
            resource_version,
            timeout_seconds,
            watch,
        } = optional;
        let __url = format!("/apis/autoscaling/v1/namespaces/{namespace}/horizontalpodautoscalers?", namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(continue_) = continue_ {
            __query_pairs.append_pair("continue", continue_);
        }
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", field_selector);
        }
        if let Some(include_uninitialized) = include_uninitialized {
            __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", label_selector);
        }
        if let Some(limit) = limit {
            __query_pairs.append_pair("limit", &limit.to_string());
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        if let Some(resource_version) = resource_version {
            __query_pairs.append_pair("resourceVersion", resource_version);
        }
        if let Some(timeout_seconds) = timeout_seconds {
            __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
        }
        if let Some(watch) = watch {
            __query_pairs.append_pair("watch", &watch.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`HorizontalPodAutoscaler::list_namespaced_horizontal_pod_autoscaler`](./struct.HorizontalPodAutoscaler.html#method.list_namespaced_horizontal_pod_autoscaler)
#[derive(Clone, Copy, Debug, Default)]
pub struct ListNamespacedHorizontalPodAutoscalerOptional<'a> {
    /// The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server the server will respond with a 410 ResourceExpired error indicating the client must restart their list without the continue field. This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
    pub continue_: Option<&'a str>,
    /// A selector to restrict the list of returned objects by their fields. Defaults to everything.
    pub field_selector: Option<&'a str>,
    /// If true, partially initialized resources are included in the response.
    pub include_uninitialized: Option<bool>,
    /// A selector to restrict the list of returned objects by their labels. Defaults to everything.
    pub label_selector: Option<&'a str>,
    /// limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.
    ///
    /// The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
    pub limit: Option<i64>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
    pub resource_version: Option<&'a str>,
    /// Timeout for the list/watch call.
    pub timeout_seconds: Option<i64>,
    /// Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub watch: Option<bool>,
}

/// Parses the HTTP response of [`HorizontalPodAutoscaler::list_namespaced_horizontal_pod_autoscaler`](./struct.HorizontalPodAutoscaler.html#method.list_namespaced_horizontal_pod_autoscaler)
#[derive(Debug)]
pub enum ListNamespacedHorizontalPodAutoscalerResponse {
    Ok(crate::v1_8::api::autoscaling::v1::HorizontalPodAutoscalerList),
    Unauthorized,
    Other,
}

impl crate::Response for ListNamespacedHorizontalPodAutoscalerResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ListNamespacedHorizontalPodAutoscalerResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ListNamespacedHorizontalPodAutoscalerResponse::Unauthorized, 0)),
            _ => Ok((ListNamespacedHorizontalPodAutoscalerResponse::Other, 0)),
        }
    }
}

// Generated from operation patchAutoscalingV1NamespacedHorizontalPodAutoscaler

impl HorizontalPodAutoscaler {
    /// partially update the specified HorizontalPodAutoscaler
    ///
    /// Use [`PatchNamespacedHorizontalPodAutoscalerResponse`](./enum.PatchNamespacedHorizontalPodAutoscalerResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the HorizontalPodAutoscaler
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn patch_namespaced_horizontal_pod_autoscaler(
        name: &str,
        namespace: &str,
        body: &crate::v1_8::apimachinery::pkg::apis::meta::v1::Patch,
        optional: PatchNamespacedHorizontalPodAutoscalerOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let PatchNamespacedHorizontalPodAutoscalerOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/autoscaling/v1/namespaces/{namespace}/horizontalpodautoscalers/{name}?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::patch(__url);
        let __body = serde_json::to_vec(&body).map_err(crate::RequestError::Json)?;
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`HorizontalPodAutoscaler::patch_namespaced_horizontal_pod_autoscaler`](./struct.HorizontalPodAutoscaler.html#method.patch_namespaced_horizontal_pod_autoscaler)
#[derive(Clone, Copy, Debug, Default)]
pub struct PatchNamespacedHorizontalPodAutoscalerOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`HorizontalPodAutoscaler::patch_namespaced_horizontal_pod_autoscaler`](./struct.HorizontalPodAutoscaler.html#method.patch_namespaced_horizontal_pod_autoscaler)
#[derive(Debug)]
pub enum PatchNamespacedHorizontalPodAutoscalerResponse {
    Ok(crate::v1_8::api::autoscaling::v1::HorizontalPodAutoscaler),
    Unauthorized,
    Other,
}

impl crate::Response for PatchNamespacedHorizontalPodAutoscalerResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchNamespacedHorizontalPodAutoscalerResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((PatchNamespacedHorizontalPodAutoscalerResponse::Unauthorized, 0)),
            _ => Ok((PatchNamespacedHorizontalPodAutoscalerResponse::Other, 0)),
        }
    }
}

// Generated from operation patchAutoscalingV1NamespacedHorizontalPodAutoscalerStatus

impl HorizontalPodAutoscaler {
    /// partially update status of the specified HorizontalPodAutoscaler
    ///
    /// Use [`PatchNamespacedHorizontalPodAutoscalerStatusResponse`](./enum.PatchNamespacedHorizontalPodAutoscalerStatusResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the HorizontalPodAutoscaler
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn patch_namespaced_horizontal_pod_autoscaler_status(
        name: &str,
        namespace: &str,
        body: &crate::v1_8::apimachinery::pkg::apis::meta::v1::Patch,
        optional: PatchNamespacedHorizontalPodAutoscalerStatusOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let PatchNamespacedHorizontalPodAutoscalerStatusOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/autoscaling/v1/namespaces/{namespace}/horizontalpodautoscalers/{name}/status?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::patch(__url);
        let __body = serde_json::to_vec(&body).map_err(crate::RequestError::Json)?;
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`HorizontalPodAutoscaler::patch_namespaced_horizontal_pod_autoscaler_status`](./struct.HorizontalPodAutoscaler.html#method.patch_namespaced_horizontal_pod_autoscaler_status)
#[derive(Clone, Copy, Debug, Default)]
pub struct PatchNamespacedHorizontalPodAutoscalerStatusOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`HorizontalPodAutoscaler::patch_namespaced_horizontal_pod_autoscaler_status`](./struct.HorizontalPodAutoscaler.html#method.patch_namespaced_horizontal_pod_autoscaler_status)
#[derive(Debug)]
pub enum PatchNamespacedHorizontalPodAutoscalerStatusResponse {
    Ok(crate::v1_8::api::autoscaling::v1::HorizontalPodAutoscaler),
    Unauthorized,
    Other,
}

impl crate::Response for PatchNamespacedHorizontalPodAutoscalerStatusResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchNamespacedHorizontalPodAutoscalerStatusResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((PatchNamespacedHorizontalPodAutoscalerStatusResponse::Unauthorized, 0)),
            _ => Ok((PatchNamespacedHorizontalPodAutoscalerStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation readAutoscalingV1NamespacedHorizontalPodAutoscaler

impl HorizontalPodAutoscaler {
    /// read the specified HorizontalPodAutoscaler
    ///
    /// Use [`ReadNamespacedHorizontalPodAutoscalerResponse`](./enum.ReadNamespacedHorizontalPodAutoscalerResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the HorizontalPodAutoscaler
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn read_namespaced_horizontal_pod_autoscaler(
        name: &str,
        namespace: &str,
        optional: ReadNamespacedHorizontalPodAutoscalerOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReadNamespacedHorizontalPodAutoscalerOptional {
            exact,
            export,
            pretty,
        } = optional;
        let __url = format!("/apis/autoscaling/v1/namespaces/{namespace}/horizontalpodautoscalers/{name}?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(exact) = exact {
            __query_pairs.append_pair("exact", &exact.to_string());
        }
        if let Some(export) = export {
            __query_pairs.append_pair("export", &export.to_string());
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`HorizontalPodAutoscaler::read_namespaced_horizontal_pod_autoscaler`](./struct.HorizontalPodAutoscaler.html#method.read_namespaced_horizontal_pod_autoscaler)
#[derive(Clone, Copy, Debug, Default)]
pub struct ReadNamespacedHorizontalPodAutoscalerOptional<'a> {
    /// Should the export be exact.  Exact export maintains cluster-specific fields like 'Namespace'.
    pub exact: Option<bool>,
    /// Should this value be exported.  Export strips fields that a user can not specify.
    pub export: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`HorizontalPodAutoscaler::read_namespaced_horizontal_pod_autoscaler`](./struct.HorizontalPodAutoscaler.html#method.read_namespaced_horizontal_pod_autoscaler)
#[derive(Debug)]
pub enum ReadNamespacedHorizontalPodAutoscalerResponse {
    Ok(crate::v1_8::api::autoscaling::v1::HorizontalPodAutoscaler),
    Unauthorized,
    Other,
}

impl crate::Response for ReadNamespacedHorizontalPodAutoscalerResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadNamespacedHorizontalPodAutoscalerResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReadNamespacedHorizontalPodAutoscalerResponse::Unauthorized, 0)),
            _ => Ok((ReadNamespacedHorizontalPodAutoscalerResponse::Other, 0)),
        }
    }
}

// Generated from operation readAutoscalingV1NamespacedHorizontalPodAutoscalerStatus

impl HorizontalPodAutoscaler {
    /// read status of the specified HorizontalPodAutoscaler
    ///
    /// Use [`ReadNamespacedHorizontalPodAutoscalerStatusResponse`](./enum.ReadNamespacedHorizontalPodAutoscalerStatusResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the HorizontalPodAutoscaler
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn read_namespaced_horizontal_pod_autoscaler_status(
        name: &str,
        namespace: &str,
        optional: ReadNamespacedHorizontalPodAutoscalerStatusOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReadNamespacedHorizontalPodAutoscalerStatusOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/autoscaling/v1/namespaces/{namespace}/horizontalpodautoscalers/{name}/status?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`HorizontalPodAutoscaler::read_namespaced_horizontal_pod_autoscaler_status`](./struct.HorizontalPodAutoscaler.html#method.read_namespaced_horizontal_pod_autoscaler_status)
#[derive(Clone, Copy, Debug, Default)]
pub struct ReadNamespacedHorizontalPodAutoscalerStatusOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`HorizontalPodAutoscaler::read_namespaced_horizontal_pod_autoscaler_status`](./struct.HorizontalPodAutoscaler.html#method.read_namespaced_horizontal_pod_autoscaler_status)
#[derive(Debug)]
pub enum ReadNamespacedHorizontalPodAutoscalerStatusResponse {
    Ok(crate::v1_8::api::autoscaling::v1::HorizontalPodAutoscaler),
    Unauthorized,
    Other,
}

impl crate::Response for ReadNamespacedHorizontalPodAutoscalerStatusResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadNamespacedHorizontalPodAutoscalerStatusResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReadNamespacedHorizontalPodAutoscalerStatusResponse::Unauthorized, 0)),
            _ => Ok((ReadNamespacedHorizontalPodAutoscalerStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceAutoscalingV1NamespacedHorizontalPodAutoscaler

impl HorizontalPodAutoscaler {
    /// replace the specified HorizontalPodAutoscaler
    ///
    /// Use [`ReplaceNamespacedHorizontalPodAutoscalerResponse`](./enum.ReplaceNamespacedHorizontalPodAutoscalerResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the HorizontalPodAutoscaler
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn replace_namespaced_horizontal_pod_autoscaler(
        name: &str,
        namespace: &str,
        body: &crate::v1_8::api::autoscaling::v1::HorizontalPodAutoscaler,
        optional: ReplaceNamespacedHorizontalPodAutoscalerOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReplaceNamespacedHorizontalPodAutoscalerOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/autoscaling/v1/namespaces/{namespace}/horizontalpodautoscalers/{name}?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::put(__url);
        let __body = serde_json::to_vec(&body).map_err(crate::RequestError::Json)?;
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`HorizontalPodAutoscaler::replace_namespaced_horizontal_pod_autoscaler`](./struct.HorizontalPodAutoscaler.html#method.replace_namespaced_horizontal_pod_autoscaler)
#[derive(Clone, Copy, Debug, Default)]
pub struct ReplaceNamespacedHorizontalPodAutoscalerOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`HorizontalPodAutoscaler::replace_namespaced_horizontal_pod_autoscaler`](./struct.HorizontalPodAutoscaler.html#method.replace_namespaced_horizontal_pod_autoscaler)
#[derive(Debug)]
pub enum ReplaceNamespacedHorizontalPodAutoscalerResponse {
    Ok(crate::v1_8::api::autoscaling::v1::HorizontalPodAutoscaler),
    Unauthorized,
    Other,
}

impl crate::Response for ReplaceNamespacedHorizontalPodAutoscalerResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceNamespacedHorizontalPodAutoscalerResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReplaceNamespacedHorizontalPodAutoscalerResponse::Unauthorized, 0)),
            _ => Ok((ReplaceNamespacedHorizontalPodAutoscalerResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceAutoscalingV1NamespacedHorizontalPodAutoscalerStatus

impl HorizontalPodAutoscaler {
    /// replace status of the specified HorizontalPodAutoscaler
    ///
    /// Use [`ReplaceNamespacedHorizontalPodAutoscalerStatusResponse`](./enum.ReplaceNamespacedHorizontalPodAutoscalerStatusResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the HorizontalPodAutoscaler
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn replace_namespaced_horizontal_pod_autoscaler_status(
        name: &str,
        namespace: &str,
        body: &crate::v1_8::api::autoscaling::v1::HorizontalPodAutoscaler,
        optional: ReplaceNamespacedHorizontalPodAutoscalerStatusOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReplaceNamespacedHorizontalPodAutoscalerStatusOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/autoscaling/v1/namespaces/{namespace}/horizontalpodautoscalers/{name}/status?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::put(__url);
        let __body = serde_json::to_vec(&body).map_err(crate::RequestError::Json)?;
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`HorizontalPodAutoscaler::replace_namespaced_horizontal_pod_autoscaler_status`](./struct.HorizontalPodAutoscaler.html#method.replace_namespaced_horizontal_pod_autoscaler_status)
#[derive(Clone, Copy, Debug, Default)]
pub struct ReplaceNamespacedHorizontalPodAutoscalerStatusOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`HorizontalPodAutoscaler::replace_namespaced_horizontal_pod_autoscaler_status`](./struct.HorizontalPodAutoscaler.html#method.replace_namespaced_horizontal_pod_autoscaler_status)
#[derive(Debug)]
pub enum ReplaceNamespacedHorizontalPodAutoscalerStatusResponse {
    Ok(crate::v1_8::api::autoscaling::v1::HorizontalPodAutoscaler),
    Unauthorized,
    Other,
}

impl crate::Response for ReplaceNamespacedHorizontalPodAutoscalerStatusResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceNamespacedHorizontalPodAutoscalerStatusResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReplaceNamespacedHorizontalPodAutoscalerStatusResponse::Unauthorized, 0)),
            _ => Ok((ReplaceNamespacedHorizontalPodAutoscalerStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation watchAutoscalingV1HorizontalPodAutoscalerListForAllNamespaces

impl HorizontalPodAutoscaler {
    /// watch individual changes to a list of HorizontalPodAutoscaler
    ///
    /// Use [`WatchHorizontalPodAutoscalerListForAllNamespacesResponse`](./enum.WatchHorizontalPodAutoscalerListForAllNamespacesResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn watch_horizontal_pod_autoscaler_list_for_all_namespaces(
        optional: WatchHorizontalPodAutoscalerListForAllNamespacesOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let WatchHorizontalPodAutoscalerListForAllNamespacesOptional {
            continue_,
            field_selector,
            include_uninitialized,
            label_selector,
            limit,
            pretty,
            resource_version,
            timeout_seconds,
            watch,
        } = optional;
        let __url = "/apis/autoscaling/v1/watch/horizontalpodautoscalers?".to_string();
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(continue_) = continue_ {
            __query_pairs.append_pair("continue", continue_);
        }
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", field_selector);
        }
        if let Some(include_uninitialized) = include_uninitialized {
            __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", label_selector);
        }
        if let Some(limit) = limit {
            __query_pairs.append_pair("limit", &limit.to_string());
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        if let Some(resource_version) = resource_version {
            __query_pairs.append_pair("resourceVersion", resource_version);
        }
        if let Some(timeout_seconds) = timeout_seconds {
            __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
        }
        if let Some(watch) = watch {
            __query_pairs.append_pair("watch", &watch.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`HorizontalPodAutoscaler::watch_horizontal_pod_autoscaler_list_for_all_namespaces`](./struct.HorizontalPodAutoscaler.html#method.watch_horizontal_pod_autoscaler_list_for_all_namespaces)
#[derive(Clone, Copy, Debug, Default)]
pub struct WatchHorizontalPodAutoscalerListForAllNamespacesOptional<'a> {
    /// The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server the server will respond with a 410 ResourceExpired error indicating the client must restart their list without the continue field. This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
    pub continue_: Option<&'a str>,
    /// A selector to restrict the list of returned objects by their fields. Defaults to everything.
    pub field_selector: Option<&'a str>,
    /// If true, partially initialized resources are included in the response.
    pub include_uninitialized: Option<bool>,
    /// A selector to restrict the list of returned objects by their labels. Defaults to everything.
    pub label_selector: Option<&'a str>,
    /// limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.
    ///
    /// The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
    pub limit: Option<i64>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
    pub resource_version: Option<&'a str>,
    /// Timeout for the list/watch call.
    pub timeout_seconds: Option<i64>,
    /// Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub watch: Option<bool>,
}

/// Parses the HTTP response of [`HorizontalPodAutoscaler::watch_horizontal_pod_autoscaler_list_for_all_namespaces`](./struct.HorizontalPodAutoscaler.html#method.watch_horizontal_pod_autoscaler_list_for_all_namespaces)
#[derive(Debug)]
pub enum WatchHorizontalPodAutoscalerListForAllNamespacesResponse {
    Ok(crate::v1_8::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl crate::Response for WatchHorizontalPodAutoscalerListForAllNamespacesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let mut deserializer = serde_json::Deserializer::from_slice(buf).into_iter();
                let (result, byte_offset) = match deserializer.next() {
                    Some(Ok(value)) => (value, deserializer.byte_offset()),
                    Some(Err(ref err)) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Some(Err(err)) => return Err(crate::ResponseError::Json(err)),
                    None => return Err(crate::ResponseError::NeedMoreData),
                };
                Ok((WatchHorizontalPodAutoscalerListForAllNamespacesResponse::Ok(result), byte_offset))
            },
            http::StatusCode::UNAUTHORIZED => Ok((WatchHorizontalPodAutoscalerListForAllNamespacesResponse::Unauthorized, 0)),
            _ => Ok((WatchHorizontalPodAutoscalerListForAllNamespacesResponse::Other, 0)),
        }
    }
}

// Generated from operation watchAutoscalingV1NamespacedHorizontalPodAutoscaler

impl HorizontalPodAutoscaler {
    /// watch changes to an object of kind HorizontalPodAutoscaler
    ///
    /// Use [`WatchNamespacedHorizontalPodAutoscalerResponse`](./enum.WatchNamespacedHorizontalPodAutoscalerResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the HorizontalPodAutoscaler
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn watch_namespaced_horizontal_pod_autoscaler(
        name: &str,
        namespace: &str,
        optional: WatchNamespacedHorizontalPodAutoscalerOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let WatchNamespacedHorizontalPodAutoscalerOptional {
            continue_,
            field_selector,
            include_uninitialized,
            label_selector,
            limit,
            pretty,
            resource_version,
            timeout_seconds,
            watch,
        } = optional;
        let __url = format!("/apis/autoscaling/v1/watch/namespaces/{namespace}/horizontalpodautoscalers/{name}?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(continue_) = continue_ {
            __query_pairs.append_pair("continue", continue_);
        }
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", field_selector);
        }
        if let Some(include_uninitialized) = include_uninitialized {
            __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", label_selector);
        }
        if let Some(limit) = limit {
            __query_pairs.append_pair("limit", &limit.to_string());
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        if let Some(resource_version) = resource_version {
            __query_pairs.append_pair("resourceVersion", resource_version);
        }
        if let Some(timeout_seconds) = timeout_seconds {
            __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
        }
        if let Some(watch) = watch {
            __query_pairs.append_pair("watch", &watch.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`HorizontalPodAutoscaler::watch_namespaced_horizontal_pod_autoscaler`](./struct.HorizontalPodAutoscaler.html#method.watch_namespaced_horizontal_pod_autoscaler)
#[derive(Clone, Copy, Debug, Default)]
pub struct WatchNamespacedHorizontalPodAutoscalerOptional<'a> {
    /// The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server the server will respond with a 410 ResourceExpired error indicating the client must restart their list without the continue field. This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
    pub continue_: Option<&'a str>,
    /// A selector to restrict the list of returned objects by their fields. Defaults to everything.
    pub field_selector: Option<&'a str>,
    /// If true, partially initialized resources are included in the response.
    pub include_uninitialized: Option<bool>,
    /// A selector to restrict the list of returned objects by their labels. Defaults to everything.
    pub label_selector: Option<&'a str>,
    /// limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.
    ///
    /// The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
    pub limit: Option<i64>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
    pub resource_version: Option<&'a str>,
    /// Timeout for the list/watch call.
    pub timeout_seconds: Option<i64>,
    /// Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub watch: Option<bool>,
}

/// Parses the HTTP response of [`HorizontalPodAutoscaler::watch_namespaced_horizontal_pod_autoscaler`](./struct.HorizontalPodAutoscaler.html#method.watch_namespaced_horizontal_pod_autoscaler)
#[derive(Debug)]
pub enum WatchNamespacedHorizontalPodAutoscalerResponse {
    Ok(crate::v1_8::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl crate::Response for WatchNamespacedHorizontalPodAutoscalerResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let mut deserializer = serde_json::Deserializer::from_slice(buf).into_iter();
                let (result, byte_offset) = match deserializer.next() {
                    Some(Ok(value)) => (value, deserializer.byte_offset()),
                    Some(Err(ref err)) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Some(Err(err)) => return Err(crate::ResponseError::Json(err)),
                    None => return Err(crate::ResponseError::NeedMoreData),
                };
                Ok((WatchNamespacedHorizontalPodAutoscalerResponse::Ok(result), byte_offset))
            },
            http::StatusCode::UNAUTHORIZED => Ok((WatchNamespacedHorizontalPodAutoscalerResponse::Unauthorized, 0)),
            _ => Ok((WatchNamespacedHorizontalPodAutoscalerResponse::Other, 0)),
        }
    }
}

// Generated from operation watchAutoscalingV1NamespacedHorizontalPodAutoscalerList

impl HorizontalPodAutoscaler {
    /// watch individual changes to a list of HorizontalPodAutoscaler
    ///
    /// Use [`WatchNamespacedHorizontalPodAutoscalerListResponse`](./enum.WatchNamespacedHorizontalPodAutoscalerListResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn watch_namespaced_horizontal_pod_autoscaler_list(
        namespace: &str,
        optional: WatchNamespacedHorizontalPodAutoscalerListOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let WatchNamespacedHorizontalPodAutoscalerListOptional {
            continue_,
            field_selector,
            include_uninitialized,
            label_selector,
            limit,
            pretty,
            resource_version,
            timeout_seconds,
            watch,
        } = optional;
        let __url = format!("/apis/autoscaling/v1/watch/namespaces/{namespace}/horizontalpodautoscalers?", namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(continue_) = continue_ {
            __query_pairs.append_pair("continue", continue_);
        }
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", field_selector);
        }
        if let Some(include_uninitialized) = include_uninitialized {
            __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", label_selector);
        }
        if let Some(limit) = limit {
            __query_pairs.append_pair("limit", &limit.to_string());
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        if let Some(resource_version) = resource_version {
            __query_pairs.append_pair("resourceVersion", resource_version);
        }
        if let Some(timeout_seconds) = timeout_seconds {
            __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
        }
        if let Some(watch) = watch {
            __query_pairs.append_pair("watch", &watch.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`HorizontalPodAutoscaler::watch_namespaced_horizontal_pod_autoscaler_list`](./struct.HorizontalPodAutoscaler.html#method.watch_namespaced_horizontal_pod_autoscaler_list)
#[derive(Clone, Copy, Debug, Default)]
pub struct WatchNamespacedHorizontalPodAutoscalerListOptional<'a> {
    /// The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server the server will respond with a 410 ResourceExpired error indicating the client must restart their list without the continue field. This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
    pub continue_: Option<&'a str>,
    /// A selector to restrict the list of returned objects by their fields. Defaults to everything.
    pub field_selector: Option<&'a str>,
    /// If true, partially initialized resources are included in the response.
    pub include_uninitialized: Option<bool>,
    /// A selector to restrict the list of returned objects by their labels. Defaults to everything.
    pub label_selector: Option<&'a str>,
    /// limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.
    ///
    /// The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
    pub limit: Option<i64>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
    pub resource_version: Option<&'a str>,
    /// Timeout for the list/watch call.
    pub timeout_seconds: Option<i64>,
    /// Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub watch: Option<bool>,
}

/// Parses the HTTP response of [`HorizontalPodAutoscaler::watch_namespaced_horizontal_pod_autoscaler_list`](./struct.HorizontalPodAutoscaler.html#method.watch_namespaced_horizontal_pod_autoscaler_list)
#[derive(Debug)]
pub enum WatchNamespacedHorizontalPodAutoscalerListResponse {
    Ok(crate::v1_8::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl crate::Response for WatchNamespacedHorizontalPodAutoscalerListResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let mut deserializer = serde_json::Deserializer::from_slice(buf).into_iter();
                let (result, byte_offset) = match deserializer.next() {
                    Some(Ok(value)) => (value, deserializer.byte_offset()),
                    Some(Err(ref err)) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Some(Err(err)) => return Err(crate::ResponseError::Json(err)),
                    None => return Err(crate::ResponseError::NeedMoreData),
                };
                Ok((WatchNamespacedHorizontalPodAutoscalerListResponse::Ok(result), byte_offset))
            },
            http::StatusCode::UNAUTHORIZED => Ok((WatchNamespacedHorizontalPodAutoscalerListResponse::Unauthorized, 0)),
            _ => Ok((WatchNamespacedHorizontalPodAutoscalerListResponse::Other, 0)),
        }
    }
}

// End autoscaling/v1/HorizontalPodAutoscaler

impl crate::Resource for HorizontalPodAutoscaler {
    fn api_version() -> &'static str {
        "autoscaling/v1"
    }

    fn group() -> &'static str {
        "autoscaling"
    }

    fn kind() -> &'static str {
        "HorizontalPodAutoscaler"
    }

    fn version() -> &'static str {
        "v1"
    }
}

impl crate::Metadata for HorizontalPodAutoscaler {
    type Ty = crate::v1_8::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> Option<&<Self as crate::Metadata>::Ty> {
        self.metadata.as_ref()
    }
}

impl<'de> serde::Deserialize<'de> for HorizontalPodAutoscaler {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_metadata,
            Key_spec,
            Key_status,
            Other,
        }

        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(f, "field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
                        Ok(match v {
                            "apiVersion" => Field::Key_api_version,
                            "kind" => Field::Key_kind,
                            "metadata" => Field::Key_metadata,
                            "spec" => Field::Key_spec,
                            "status" => Field::Key_status,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = HorizontalPodAutoscaler;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "struct HorizontalPodAutoscaler")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_metadata: Option<crate::v1_8::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<crate::v1_8::api::autoscaling::v1::HorizontalPodAutoscalerSpec> = None;
                let mut value_status: Option<crate::v1_8::api::autoscaling::v1::HorizontalPodAutoscalerStatus> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => {
                            let value_api_version: String = serde::de::MapAccess::next_value(&mut map)?;
                            if value_api_version != <Self::Value as crate::Resource>::api_version() {
                                return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(&value_api_version), &<Self::Value as crate::Resource>::api_version()));
                            }
                        },
                        Field::Key_kind => {
                            let value_kind: String = serde::de::MapAccess::next_value(&mut map)?;
                            if value_kind != <Self::Value as crate::Resource>::kind() {
                                return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(&value_kind), &<Self::Value as crate::Resource>::kind()));
                            }
                        },
                        Field::Key_metadata => value_metadata = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_spec => value_spec = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_status => value_status = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(HorizontalPodAutoscaler {
                    metadata: value_metadata,
                    spec: value_spec,
                    status: value_status,
                })
            }
        }

        deserializer.deserialize_struct(
            "HorizontalPodAutoscaler",
            &[
                "apiVersion",
                "kind",
                "metadata",
                "spec",
                "status",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for HorizontalPodAutoscaler {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "HorizontalPodAutoscaler",
            2 +
            self.metadata.as_ref().map_or(0, |_| 1) +
            self.spec.as_ref().map_or(0, |_| 1) +
            self.status.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as crate::Resource>::api_version())?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as crate::Resource>::kind())?;
        if let Some(value) = &self.metadata {
            serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", value)?;
        }
        if let Some(value) = &self.spec {
            serde::ser::SerializeStruct::serialize_field(&mut state, "spec", value)?;
        }
        if let Some(value) = &self.status {
            serde::ser::SerializeStruct::serialize_field(&mut state, "status", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
