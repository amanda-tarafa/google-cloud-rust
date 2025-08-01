// Copyright 2025 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// Code generated by sidekick. DO NOT EDIT.

pub mod data_policy_service {
    use crate::Result;

    /// A builder for [DataPolicyService][crate::client::DataPolicyService].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_bigquery_datapolicies_v2::*;
    /// # use builder::data_policy_service::ClientBuilder;
    /// # use client::DataPolicyService;
    /// let builder : ClientBuilder = DataPolicyService::builder();
    /// let client = builder
    ///     .with_endpoint("https://bigquerydatapolicy.googleapis.com")
    ///     .build().await?;
    /// # gax::client_builder::Result::<()>::Ok(()) });
    /// ```
    pub type ClientBuilder =
        gax::client_builder::ClientBuilder<client::Factory, gaxi::options::Credentials>;

    pub(crate) mod client {
        use super::super::super::client::DataPolicyService;
        pub struct Factory;
        impl gax::client_builder::internal::ClientFactory for Factory {
            type Client = DataPolicyService;
            type Credentials = gaxi::options::Credentials;
            async fn build(
                self,
                config: gaxi::options::ClientConfig,
            ) -> gax::client_builder::Result<Self::Client> {
                Self::Client::new(config).await
            }
        }
    }

    /// Common implementation for [crate::client::DataPolicyService] request builders.
    #[derive(Clone, Debug)]
    pub(crate) struct RequestBuilder<R: std::default::Default> {
        stub: std::sync::Arc<dyn super::super::stub::dynamic::DataPolicyService>,
        request: R,
        options: gax::options::RequestOptions,
    }

    impl<R> RequestBuilder<R>
    where
        R: std::default::Default,
    {
        pub(crate) fn new(
            stub: std::sync::Arc<dyn super::super::stub::dynamic::DataPolicyService>,
        ) -> Self {
            Self {
                stub,
                request: R::default(),
                options: gax::options::RequestOptions::default(),
            }
        }
    }

    /// The request builder for [DataPolicyService::create_data_policy][crate::client::DataPolicyService::create_data_policy] calls.
    ///
    /// # Example
    /// ```no_run
    /// # use google_cloud_bigquery_datapolicies_v2::builder;
    /// use builder::data_policy_service::CreateDataPolicy;
    /// # tokio_test::block_on(async {
    ///
    /// let builder = prepare_request_builder();
    /// let response = builder.send().await?;
    /// # gax::Result::<()>::Ok(()) });
    ///
    /// fn prepare_request_builder() -> CreateDataPolicy {
    ///   # panic!();
    ///   // ... details omitted ...
    /// }
    /// ```
    #[derive(Clone, Debug)]
    pub struct CreateDataPolicy(RequestBuilder<crate::model::CreateDataPolicyRequest>);

    impl CreateDataPolicy {
        pub(crate) fn new(
            stub: std::sync::Arc<dyn super::super::stub::dynamic::DataPolicyService>,
        ) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::CreateDataPolicyRequest>>(
            mut self,
            v: V,
        ) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::DataPolicy> {
            (*self.0.stub)
                .create_data_policy(self.0.request, self.0.options)
                .await
                .map(gax::response::Response::into_body)
        }

        /// Sets the value of [parent][crate::model::CreateDataPolicyRequest::parent].
        ///
        /// This is a **required** field for requests.
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [data_policy_id][crate::model::CreateDataPolicyRequest::data_policy_id].
        ///
        /// This is a **required** field for requests.
        pub fn set_data_policy_id<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.data_policy_id = v.into();
            self
        }

        /// Sets the value of [data_policy][crate::model::CreateDataPolicyRequest::data_policy].
        ///
        /// This is a **required** field for requests.
        pub fn set_data_policy<T>(mut self, v: T) -> Self
        where
            T: std::convert::Into<crate::model::DataPolicy>,
        {
            self.0.request.data_policy = std::option::Option::Some(v.into());
            self
        }

        /// Sets or clears the value of [data_policy][crate::model::CreateDataPolicyRequest::data_policy].
        ///
        /// This is a **required** field for requests.
        pub fn set_or_clear_data_policy<T>(mut self, v: std::option::Option<T>) -> Self
        where
            T: std::convert::Into<crate::model::DataPolicy>,
        {
            self.0.request.data_policy = v.map(|x| x.into());
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for CreateDataPolicy {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [DataPolicyService::add_grantees][crate::client::DataPolicyService::add_grantees] calls.
    ///
    /// # Example
    /// ```no_run
    /// # use google_cloud_bigquery_datapolicies_v2::builder;
    /// use builder::data_policy_service::AddGrantees;
    /// # tokio_test::block_on(async {
    ///
    /// let builder = prepare_request_builder();
    /// let response = builder.send().await?;
    /// # gax::Result::<()>::Ok(()) });
    ///
    /// fn prepare_request_builder() -> AddGrantees {
    ///   # panic!();
    ///   // ... details omitted ...
    /// }
    /// ```
    #[derive(Clone, Debug)]
    pub struct AddGrantees(RequestBuilder<crate::model::AddGranteesRequest>);

    impl AddGrantees {
        pub(crate) fn new(
            stub: std::sync::Arc<dyn super::super::stub::dynamic::DataPolicyService>,
        ) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::AddGranteesRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::DataPolicy> {
            (*self.0.stub)
                .add_grantees(self.0.request, self.0.options)
                .await
                .map(gax::response::Response::into_body)
        }

        /// Sets the value of [data_policy][crate::model::AddGranteesRequest::data_policy].
        ///
        /// This is a **required** field for requests.
        pub fn set_data_policy<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.data_policy = v.into();
            self
        }

        /// Sets the value of [grantees][crate::model::AddGranteesRequest::grantees].
        ///
        /// This is a **required** field for requests.
        pub fn set_grantees<T, V>(mut self, v: T) -> Self
        where
            T: std::iter::IntoIterator<Item = V>,
            V: std::convert::Into<std::string::String>,
        {
            use std::iter::Iterator;
            self.0.request.grantees = v.into_iter().map(|i| i.into()).collect();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for AddGrantees {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [DataPolicyService::remove_grantees][crate::client::DataPolicyService::remove_grantees] calls.
    ///
    /// # Example
    /// ```no_run
    /// # use google_cloud_bigquery_datapolicies_v2::builder;
    /// use builder::data_policy_service::RemoveGrantees;
    /// # tokio_test::block_on(async {
    ///
    /// let builder = prepare_request_builder();
    /// let response = builder.send().await?;
    /// # gax::Result::<()>::Ok(()) });
    ///
    /// fn prepare_request_builder() -> RemoveGrantees {
    ///   # panic!();
    ///   // ... details omitted ...
    /// }
    /// ```
    #[derive(Clone, Debug)]
    pub struct RemoveGrantees(RequestBuilder<crate::model::RemoveGranteesRequest>);

    impl RemoveGrantees {
        pub(crate) fn new(
            stub: std::sync::Arc<dyn super::super::stub::dynamic::DataPolicyService>,
        ) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::RemoveGranteesRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::DataPolicy> {
            (*self.0.stub)
                .remove_grantees(self.0.request, self.0.options)
                .await
                .map(gax::response::Response::into_body)
        }

        /// Sets the value of [data_policy][crate::model::RemoveGranteesRequest::data_policy].
        ///
        /// This is a **required** field for requests.
        pub fn set_data_policy<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.data_policy = v.into();
            self
        }

        /// Sets the value of [grantees][crate::model::RemoveGranteesRequest::grantees].
        ///
        /// This is a **required** field for requests.
        pub fn set_grantees<T, V>(mut self, v: T) -> Self
        where
            T: std::iter::IntoIterator<Item = V>,
            V: std::convert::Into<std::string::String>,
        {
            use std::iter::Iterator;
            self.0.request.grantees = v.into_iter().map(|i| i.into()).collect();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for RemoveGrantees {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [DataPolicyService::update_data_policy][crate::client::DataPolicyService::update_data_policy] calls.
    ///
    /// # Example
    /// ```no_run
    /// # use google_cloud_bigquery_datapolicies_v2::builder;
    /// use builder::data_policy_service::UpdateDataPolicy;
    /// # tokio_test::block_on(async {
    ///
    /// let builder = prepare_request_builder();
    /// let response = builder.send().await?;
    /// # gax::Result::<()>::Ok(()) });
    ///
    /// fn prepare_request_builder() -> UpdateDataPolicy {
    ///   # panic!();
    ///   // ... details omitted ...
    /// }
    /// ```
    #[derive(Clone, Debug)]
    pub struct UpdateDataPolicy(RequestBuilder<crate::model::UpdateDataPolicyRequest>);

    impl UpdateDataPolicy {
        pub(crate) fn new(
            stub: std::sync::Arc<dyn super::super::stub::dynamic::DataPolicyService>,
        ) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::UpdateDataPolicyRequest>>(
            mut self,
            v: V,
        ) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::DataPolicy> {
            (*self.0.stub)
                .update_data_policy(self.0.request, self.0.options)
                .await
                .map(gax::response::Response::into_body)
        }

        /// Sets the value of [data_policy][crate::model::UpdateDataPolicyRequest::data_policy].
        ///
        /// This is a **required** field for requests.
        pub fn set_data_policy<T>(mut self, v: T) -> Self
        where
            T: std::convert::Into<crate::model::DataPolicy>,
        {
            self.0.request.data_policy = std::option::Option::Some(v.into());
            self
        }

        /// Sets or clears the value of [data_policy][crate::model::UpdateDataPolicyRequest::data_policy].
        ///
        /// This is a **required** field for requests.
        pub fn set_or_clear_data_policy<T>(mut self, v: std::option::Option<T>) -> Self
        where
            T: std::convert::Into<crate::model::DataPolicy>,
        {
            self.0.request.data_policy = v.map(|x| x.into());
            self
        }

        /// Sets the value of [update_mask][crate::model::UpdateDataPolicyRequest::update_mask].
        pub fn set_update_mask<T>(mut self, v: T) -> Self
        where
            T: std::convert::Into<wkt::FieldMask>,
        {
            self.0.request.update_mask = std::option::Option::Some(v.into());
            self
        }

        /// Sets or clears the value of [update_mask][crate::model::UpdateDataPolicyRequest::update_mask].
        pub fn set_or_clear_update_mask<T>(mut self, v: std::option::Option<T>) -> Self
        where
            T: std::convert::Into<wkt::FieldMask>,
        {
            self.0.request.update_mask = v.map(|x| x.into());
            self
        }

        /// Sets the value of [allow_missing][crate::model::UpdateDataPolicyRequest::allow_missing].
        pub fn set_allow_missing<T: Into<bool>>(mut self, v: T) -> Self {
            self.0.request.allow_missing = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for UpdateDataPolicy {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [DataPolicyService::delete_data_policy][crate::client::DataPolicyService::delete_data_policy] calls.
    ///
    /// # Example
    /// ```no_run
    /// # use google_cloud_bigquery_datapolicies_v2::builder;
    /// use builder::data_policy_service::DeleteDataPolicy;
    /// # tokio_test::block_on(async {
    ///
    /// let builder = prepare_request_builder();
    /// let response = builder.send().await?;
    /// # gax::Result::<()>::Ok(()) });
    ///
    /// fn prepare_request_builder() -> DeleteDataPolicy {
    ///   # panic!();
    ///   // ... details omitted ...
    /// }
    /// ```
    #[derive(Clone, Debug)]
    pub struct DeleteDataPolicy(RequestBuilder<crate::model::DeleteDataPolicyRequest>);

    impl DeleteDataPolicy {
        pub(crate) fn new(
            stub: std::sync::Arc<dyn super::super::stub::dynamic::DataPolicyService>,
        ) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::DeleteDataPolicyRequest>>(
            mut self,
            v: V,
        ) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<()> {
            (*self.0.stub)
                .delete_data_policy(self.0.request, self.0.options)
                .await
                .map(gax::response::Response::into_body)
        }

        /// Sets the value of [name][crate::model::DeleteDataPolicyRequest::name].
        ///
        /// This is a **required** field for requests.
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for DeleteDataPolicy {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [DataPolicyService::get_data_policy][crate::client::DataPolicyService::get_data_policy] calls.
    ///
    /// # Example
    /// ```no_run
    /// # use google_cloud_bigquery_datapolicies_v2::builder;
    /// use builder::data_policy_service::GetDataPolicy;
    /// # tokio_test::block_on(async {
    ///
    /// let builder = prepare_request_builder();
    /// let response = builder.send().await?;
    /// # gax::Result::<()>::Ok(()) });
    ///
    /// fn prepare_request_builder() -> GetDataPolicy {
    ///   # panic!();
    ///   // ... details omitted ...
    /// }
    /// ```
    #[derive(Clone, Debug)]
    pub struct GetDataPolicy(RequestBuilder<crate::model::GetDataPolicyRequest>);

    impl GetDataPolicy {
        pub(crate) fn new(
            stub: std::sync::Arc<dyn super::super::stub::dynamic::DataPolicyService>,
        ) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GetDataPolicyRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::DataPolicy> {
            (*self.0.stub)
                .get_data_policy(self.0.request, self.0.options)
                .await
                .map(gax::response::Response::into_body)
        }

        /// Sets the value of [name][crate::model::GetDataPolicyRequest::name].
        ///
        /// This is a **required** field for requests.
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for GetDataPolicy {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [DataPolicyService::list_data_policies][crate::client::DataPolicyService::list_data_policies] calls.
    ///
    /// # Example
    /// ```no_run
    /// # use google_cloud_bigquery_datapolicies_v2::builder;
    /// use builder::data_policy_service::ListDataPolicies;
    /// # tokio_test::block_on(async {
    /// use gax::paginator::ItemPaginator;
    ///
    /// let builder = prepare_request_builder();
    /// let mut items = builder.by_item();
    /// while let Some(result) = items.next().await {
    ///   let item = result?;
    /// }
    /// # gax::Result::<()>::Ok(()) });
    ///
    /// fn prepare_request_builder() -> ListDataPolicies {
    ///   # panic!();
    ///   // ... details omitted ...
    /// }
    /// ```
    #[derive(Clone, Debug)]
    pub struct ListDataPolicies(RequestBuilder<crate::model::ListDataPoliciesRequest>);

    impl ListDataPolicies {
        pub(crate) fn new(
            stub: std::sync::Arc<dyn super::super::stub::dynamic::DataPolicyService>,
        ) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ListDataPoliciesRequest>>(
            mut self,
            v: V,
        ) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::ListDataPoliciesResponse> {
            (*self.0.stub)
                .list_data_policies(self.0.request, self.0.options)
                .await
                .map(gax::response::Response::into_body)
        }

        /// Streams each page in the collection.
        pub fn by_page(
            self,
        ) -> impl gax::paginator::Paginator<crate::model::ListDataPoliciesResponse, gax::error::Error>
        {
            use std::clone::Clone;
            let token = self.0.request.page_token.clone();
            let execute = move |token: String| {
                let mut builder = self.clone();
                builder.0.request = builder.0.request.set_page_token(token);
                builder.send()
            };
            gax::paginator::internal::new_paginator(token, execute)
        }

        /// Streams each item in the collection.
        pub fn by_item(
            self,
        ) -> impl gax::paginator::ItemPaginator<crate::model::ListDataPoliciesResponse, gax::error::Error>
        {
            use gax::paginator::Paginator;
            self.by_page().items()
        }

        /// Sets the value of [parent][crate::model::ListDataPoliciesRequest::parent].
        ///
        /// This is a **required** field for requests.
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [page_size][crate::model::ListDataPoliciesRequest::page_size].
        pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
            self.0.request.page_size = v.into();
            self
        }

        /// Sets the value of [page_token][crate::model::ListDataPoliciesRequest::page_token].
        pub fn set_page_token<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.page_token = v.into();
            self
        }

        /// Sets the value of [filter][crate::model::ListDataPoliciesRequest::filter].
        pub fn set_filter<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.filter = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for ListDataPolicies {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [DataPolicyService::get_iam_policy][crate::client::DataPolicyService::get_iam_policy] calls.
    ///
    /// # Example
    /// ```no_run
    /// # use google_cloud_bigquery_datapolicies_v2::builder;
    /// use builder::data_policy_service::GetIamPolicy;
    /// # tokio_test::block_on(async {
    ///
    /// let builder = prepare_request_builder();
    /// let response = builder.send().await?;
    /// # gax::Result::<()>::Ok(()) });
    ///
    /// fn prepare_request_builder() -> GetIamPolicy {
    ///   # panic!();
    ///   // ... details omitted ...
    /// }
    /// ```
    #[derive(Clone, Debug)]
    pub struct GetIamPolicy(RequestBuilder<iam_v1::model::GetIamPolicyRequest>);

    impl GetIamPolicy {
        pub(crate) fn new(
            stub: std::sync::Arc<dyn super::super::stub::dynamic::DataPolicyService>,
        ) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<iam_v1::model::GetIamPolicyRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<iam_v1::model::Policy> {
            (*self.0.stub)
                .get_iam_policy(self.0.request, self.0.options)
                .await
                .map(gax::response::Response::into_body)
        }

        /// Sets the value of [resource][iam_v1::model::GetIamPolicyRequest::resource].
        ///
        /// This is a **required** field for requests.
        pub fn set_resource<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.resource = v.into();
            self
        }

        /// Sets the value of [options][iam_v1::model::GetIamPolicyRequest::options].
        pub fn set_options<T>(mut self, v: T) -> Self
        where
            T: std::convert::Into<iam_v1::model::GetPolicyOptions>,
        {
            self.0.request.options = std::option::Option::Some(v.into());
            self
        }

        /// Sets or clears the value of [options][iam_v1::model::GetIamPolicyRequest::options].
        pub fn set_or_clear_options<T>(mut self, v: std::option::Option<T>) -> Self
        where
            T: std::convert::Into<iam_v1::model::GetPolicyOptions>,
        {
            self.0.request.options = v.map(|x| x.into());
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for GetIamPolicy {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [DataPolicyService::set_iam_policy][crate::client::DataPolicyService::set_iam_policy] calls.
    ///
    /// # Example
    /// ```no_run
    /// # use google_cloud_bigquery_datapolicies_v2::builder;
    /// use builder::data_policy_service::SetIamPolicy;
    /// # tokio_test::block_on(async {
    ///
    /// let builder = prepare_request_builder();
    /// let response = builder.send().await?;
    /// # gax::Result::<()>::Ok(()) });
    ///
    /// fn prepare_request_builder() -> SetIamPolicy {
    ///   # panic!();
    ///   // ... details omitted ...
    /// }
    /// ```
    #[derive(Clone, Debug)]
    pub struct SetIamPolicy(RequestBuilder<iam_v1::model::SetIamPolicyRequest>);

    impl SetIamPolicy {
        pub(crate) fn new(
            stub: std::sync::Arc<dyn super::super::stub::dynamic::DataPolicyService>,
        ) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<iam_v1::model::SetIamPolicyRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<iam_v1::model::Policy> {
            (*self.0.stub)
                .set_iam_policy(self.0.request, self.0.options)
                .await
                .map(gax::response::Response::into_body)
        }

        /// Sets the value of [resource][iam_v1::model::SetIamPolicyRequest::resource].
        ///
        /// This is a **required** field for requests.
        pub fn set_resource<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.resource = v.into();
            self
        }

        /// Sets the value of [policy][iam_v1::model::SetIamPolicyRequest::policy].
        ///
        /// This is a **required** field for requests.
        pub fn set_policy<T>(mut self, v: T) -> Self
        where
            T: std::convert::Into<iam_v1::model::Policy>,
        {
            self.0.request.policy = std::option::Option::Some(v.into());
            self
        }

        /// Sets or clears the value of [policy][iam_v1::model::SetIamPolicyRequest::policy].
        ///
        /// This is a **required** field for requests.
        pub fn set_or_clear_policy<T>(mut self, v: std::option::Option<T>) -> Self
        where
            T: std::convert::Into<iam_v1::model::Policy>,
        {
            self.0.request.policy = v.map(|x| x.into());
            self
        }

        /// Sets the value of [update_mask][iam_v1::model::SetIamPolicyRequest::update_mask].
        pub fn set_update_mask<T>(mut self, v: T) -> Self
        where
            T: std::convert::Into<wkt::FieldMask>,
        {
            self.0.request.update_mask = std::option::Option::Some(v.into());
            self
        }

        /// Sets or clears the value of [update_mask][iam_v1::model::SetIamPolicyRequest::update_mask].
        pub fn set_or_clear_update_mask<T>(mut self, v: std::option::Option<T>) -> Self
        where
            T: std::convert::Into<wkt::FieldMask>,
        {
            self.0.request.update_mask = v.map(|x| x.into());
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for SetIamPolicy {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [DataPolicyService::test_iam_permissions][crate::client::DataPolicyService::test_iam_permissions] calls.
    ///
    /// # Example
    /// ```no_run
    /// # use google_cloud_bigquery_datapolicies_v2::builder;
    /// use builder::data_policy_service::TestIamPermissions;
    /// # tokio_test::block_on(async {
    ///
    /// let builder = prepare_request_builder();
    /// let response = builder.send().await?;
    /// # gax::Result::<()>::Ok(()) });
    ///
    /// fn prepare_request_builder() -> TestIamPermissions {
    ///   # panic!();
    ///   // ... details omitted ...
    /// }
    /// ```
    #[derive(Clone, Debug)]
    pub struct TestIamPermissions(RequestBuilder<iam_v1::model::TestIamPermissionsRequest>);

    impl TestIamPermissions {
        pub(crate) fn new(
            stub: std::sync::Arc<dyn super::super::stub::dynamic::DataPolicyService>,
        ) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<iam_v1::model::TestIamPermissionsRequest>>(
            mut self,
            v: V,
        ) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<iam_v1::model::TestIamPermissionsResponse> {
            (*self.0.stub)
                .test_iam_permissions(self.0.request, self.0.options)
                .await
                .map(gax::response::Response::into_body)
        }

        /// Sets the value of [resource][iam_v1::model::TestIamPermissionsRequest::resource].
        ///
        /// This is a **required** field for requests.
        pub fn set_resource<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.resource = v.into();
            self
        }

        /// Sets the value of [permissions][iam_v1::model::TestIamPermissionsRequest::permissions].
        ///
        /// This is a **required** field for requests.
        pub fn set_permissions<T, V>(mut self, v: T) -> Self
        where
            T: std::iter::IntoIterator<Item = V>,
            V: std::convert::Into<std::string::String>,
        {
            use std::iter::Iterator;
            self.0.request.permissions = v.into_iter().map(|i| i.into()).collect();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for TestIamPermissions {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }
}
