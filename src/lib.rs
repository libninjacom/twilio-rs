//! [`TwilioClient`](struct.TwilioClient.html) is the main entry point for this library.
//!
//! Library created with [`libninja`](https://www.libninja.com).
#![allow(non_camel_case_types)]
#![allow(unused)]
pub mod model;
pub mod request;
use crate::model::*;
pub struct TwilioClient {
    pub client: httpclient::Client,
    authentication: TwilioAuthentication,
}
impl TwilioClient {
    pub fn from_env() -> Self {
        Self {
            client: httpclient::Client::new().base_url("https://api.twilio.com"),
            authentication: TwilioAuthentication::from_env(),
        }
    }
}
impl TwilioClient {
    pub fn new(url: &str, authentication: TwilioAuthentication) -> Self {
        let client = httpclient::Client::new().base_url(url);
        Self { client, authentication }
    }
    pub fn with_authentication(mut self, authentication: TwilioAuthentication) -> Self {
        self.authentication = authentication;
        self
    }
    pub(crate) fn authenticate<'a>(
        &self,
        mut r: httpclient::RequestBuilder<'a>,
    ) -> httpclient::RequestBuilder<'a> {
        match &self.authentication {
            TwilioAuthentication::BasicAuth { basic_auth } => {
                r = r.basic_auth(basic_auth);
            }
        }
        r
    }
    pub fn with_middleware<M: httpclient::Middleware + 'static>(
        mut self,
        middleware: M,
    ) -> Self {
        self.client = self.client.with_middleware(middleware);
        self
    }
    ///Retrieves a collection of Accounts belonging to the account used to make the request
    pub fn list_account(&self) -> request::ListAccountRequest {
        request::ListAccountRequest {
            http_client: &self,
            friendly_name: None,
            page: None,
            page_size: None,
            page_token: None,
            status: None,
        }
    }
    ///Create a new Twilio Subaccount from the account making the request
    pub fn create_account(&self) -> request::CreateAccountRequest {
        request::CreateAccountRequest {
            http_client: &self,
        }
    }
    ///Fetch the account specified by the provided Account Sid
    pub fn fetch_account(&self, sid: &str) -> request::FetchAccountRequest {
        request::FetchAccountRequest {
            http_client: &self,
            sid: sid.to_owned(),
        }
    }
    ///Modify the properties of a given Account
    pub fn update_account(&self, sid: &str) -> request::UpdateAccountRequest {
        request::UpdateAccountRequest {
            http_client: &self,
            sid: sid.to_owned(),
        }
    }
    pub fn list_address(&self, account_sid: &str) -> request::ListAddressRequest {
        request::ListAddressRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            customer_name: None,
            friendly_name: None,
            iso_country: None,
            page: None,
            page_size: None,
            page_token: None,
        }
    }
    pub fn create_address(&self, account_sid: &str) -> request::CreateAddressRequest {
        request::CreateAddressRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
        }
    }
    pub fn fetch_address(
        &self,
        account_sid: &str,
        sid: &str,
    ) -> request::FetchAddressRequest {
        request::FetchAddressRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    pub fn update_address(
        &self,
        account_sid: &str,
        sid: &str,
    ) -> request::UpdateAddressRequest {
        request::UpdateAddressRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    pub fn delete_address(
        &self,
        account_sid: &str,
        sid: &str,
    ) -> request::DeleteAddressRequest {
        request::DeleteAddressRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Retrieve a list of applications representing an application within the requesting account
    pub fn list_application(
        &self,
        account_sid: &str,
    ) -> request::ListApplicationRequest {
        request::ListApplicationRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            friendly_name: None,
            page: None,
            page_size: None,
            page_token: None,
        }
    }
    ///Create a new application within your account
    pub fn create_application(
        &self,
        account_sid: &str,
    ) -> request::CreateApplicationRequest {
        request::CreateApplicationRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
        }
    }
    ///Fetch the application specified by the provided sid
    pub fn fetch_application(
        &self,
        account_sid: &str,
        sid: &str,
    ) -> request::FetchApplicationRequest {
        request::FetchApplicationRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Updates the application's properties
    pub fn update_application(
        &self,
        account_sid: &str,
        sid: &str,
    ) -> request::UpdateApplicationRequest {
        request::UpdateApplicationRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Delete the application by the specified application sid
    pub fn delete_application(
        &self,
        account_sid: &str,
        sid: &str,
    ) -> request::DeleteApplicationRequest {
        request::DeleteApplicationRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Fetch an instance of an authorized-connect-app
    pub fn fetch_authorized_connect_app(
        &self,
        account_sid: &str,
        connect_app_sid: &str,
    ) -> request::FetchAuthorizedConnectAppRequest {
        request::FetchAuthorizedConnectAppRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            connect_app_sid: connect_app_sid.to_owned(),
        }
    }
    ///Retrieve a list of authorized-connect-apps belonging to the account used to make the request
    pub fn list_authorized_connect_app(
        &self,
        account_sid: &str,
    ) -> request::ListAuthorizedConnectAppRequest {
        request::ListAuthorizedConnectAppRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            page: None,
            page_size: None,
            page_token: None,
        }
    }
    pub fn list_available_phone_number_country(
        &self,
        account_sid: &str,
    ) -> request::ListAvailablePhoneNumberCountryRequest {
        request::ListAvailablePhoneNumberCountryRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            page: None,
            page_size: None,
            page_token: None,
        }
    }
    pub fn fetch_available_phone_number_country(
        &self,
        account_sid: &str,
        country_code: &str,
    ) -> request::FetchAvailablePhoneNumberCountryRequest {
        request::FetchAvailablePhoneNumberCountryRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            country_code: country_code.to_owned(),
        }
    }
    pub fn list_available_phone_number_local(
        &self,
        account_sid: &str,
        country_code: &str,
    ) -> request::ListAvailablePhoneNumberLocalRequest {
        request::ListAvailablePhoneNumberLocalRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            area_code: None,
            beta: None,
            contains: None,
            country_code: country_code.to_owned(),
            distance: None,
            exclude_all_address_required: None,
            exclude_foreign_address_required: None,
            exclude_local_address_required: None,
            fax_enabled: None,
            in_lata: None,
            in_locality: None,
            in_postal_code: None,
            in_rate_center: None,
            in_region: None,
            mms_enabled: None,
            near_lat_long: None,
            near_number: None,
            page: None,
            page_size: None,
            page_token: None,
            sms_enabled: None,
            voice_enabled: None,
        }
    }
    pub fn list_available_phone_number_machine_to_machine(
        &self,
        account_sid: &str,
        country_code: &str,
    ) -> request::ListAvailablePhoneNumberMachineToMachineRequest {
        request::ListAvailablePhoneNumberMachineToMachineRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            area_code: None,
            beta: None,
            contains: None,
            country_code: country_code.to_owned(),
            distance: None,
            exclude_all_address_required: None,
            exclude_foreign_address_required: None,
            exclude_local_address_required: None,
            fax_enabled: None,
            in_lata: None,
            in_locality: None,
            in_postal_code: None,
            in_rate_center: None,
            in_region: None,
            mms_enabled: None,
            near_lat_long: None,
            near_number: None,
            page: None,
            page_size: None,
            page_token: None,
            sms_enabled: None,
            voice_enabled: None,
        }
    }
    pub fn list_available_phone_number_mobile(
        &self,
        account_sid: &str,
        country_code: &str,
    ) -> request::ListAvailablePhoneNumberMobileRequest {
        request::ListAvailablePhoneNumberMobileRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            area_code: None,
            beta: None,
            contains: None,
            country_code: country_code.to_owned(),
            distance: None,
            exclude_all_address_required: None,
            exclude_foreign_address_required: None,
            exclude_local_address_required: None,
            fax_enabled: None,
            in_lata: None,
            in_locality: None,
            in_postal_code: None,
            in_rate_center: None,
            in_region: None,
            mms_enabled: None,
            near_lat_long: None,
            near_number: None,
            page: None,
            page_size: None,
            page_token: None,
            sms_enabled: None,
            voice_enabled: None,
        }
    }
    pub fn list_available_phone_number_national(
        &self,
        account_sid: &str,
        country_code: &str,
    ) -> request::ListAvailablePhoneNumberNationalRequest {
        request::ListAvailablePhoneNumberNationalRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            area_code: None,
            beta: None,
            contains: None,
            country_code: country_code.to_owned(),
            distance: None,
            exclude_all_address_required: None,
            exclude_foreign_address_required: None,
            exclude_local_address_required: None,
            fax_enabled: None,
            in_lata: None,
            in_locality: None,
            in_postal_code: None,
            in_rate_center: None,
            in_region: None,
            mms_enabled: None,
            near_lat_long: None,
            near_number: None,
            page: None,
            page_size: None,
            page_token: None,
            sms_enabled: None,
            voice_enabled: None,
        }
    }
    pub fn list_available_phone_number_shared_cost(
        &self,
        account_sid: &str,
        country_code: &str,
    ) -> request::ListAvailablePhoneNumberSharedCostRequest {
        request::ListAvailablePhoneNumberSharedCostRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            area_code: None,
            beta: None,
            contains: None,
            country_code: country_code.to_owned(),
            distance: None,
            exclude_all_address_required: None,
            exclude_foreign_address_required: None,
            exclude_local_address_required: None,
            fax_enabled: None,
            in_lata: None,
            in_locality: None,
            in_postal_code: None,
            in_rate_center: None,
            in_region: None,
            mms_enabled: None,
            near_lat_long: None,
            near_number: None,
            page: None,
            page_size: None,
            page_token: None,
            sms_enabled: None,
            voice_enabled: None,
        }
    }
    pub fn list_available_phone_number_toll_free(
        &self,
        account_sid: &str,
        country_code: &str,
    ) -> request::ListAvailablePhoneNumberTollFreeRequest {
        request::ListAvailablePhoneNumberTollFreeRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            area_code: None,
            beta: None,
            contains: None,
            country_code: country_code.to_owned(),
            distance: None,
            exclude_all_address_required: None,
            exclude_foreign_address_required: None,
            exclude_local_address_required: None,
            fax_enabled: None,
            in_lata: None,
            in_locality: None,
            in_postal_code: None,
            in_rate_center: None,
            in_region: None,
            mms_enabled: None,
            near_lat_long: None,
            near_number: None,
            page: None,
            page_size: None,
            page_token: None,
            sms_enabled: None,
            voice_enabled: None,
        }
    }
    pub fn list_available_phone_number_voip(
        &self,
        account_sid: &str,
        country_code: &str,
    ) -> request::ListAvailablePhoneNumberVoipRequest {
        request::ListAvailablePhoneNumberVoipRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            area_code: None,
            beta: None,
            contains: None,
            country_code: country_code.to_owned(),
            distance: None,
            exclude_all_address_required: None,
            exclude_foreign_address_required: None,
            exclude_local_address_required: None,
            fax_enabled: None,
            in_lata: None,
            in_locality: None,
            in_postal_code: None,
            in_rate_center: None,
            in_region: None,
            mms_enabled: None,
            near_lat_long: None,
            near_number: None,
            page: None,
            page_size: None,
            page_token: None,
            sms_enabled: None,
            voice_enabled: None,
        }
    }
    ///Fetch the balance for an Account based on Account Sid. Balance changes may not be reflected immediately. Child accounts do not contain balance information
    pub fn fetch_balance(&self, account_sid: &str) -> request::FetchBalanceRequest {
        request::FetchBalanceRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
        }
    }
    ///Retrieves a collection of calls made to and from your account
    pub fn list_call(&self, account_sid: &str) -> request::ListCallRequest {
        request::ListCallRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            end_time: None,
            end_time_gt: None,
            end_time_lt: None,
            from: None,
            page: None,
            page_size: None,
            page_token: None,
            parent_call_sid: None,
            start_time: None,
            start_time_gt: None,
            start_time_lt: None,
            status: None,
            to: None,
        }
    }
    ///Create a new outgoing call to phones, SIP-enabled endpoints or Twilio Client connections
    pub fn create_call(&self, account_sid: &str) -> request::CreateCallRequest {
        request::CreateCallRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
        }
    }
    ///Fetch the call specified by the provided Call SID
    pub fn fetch_call(&self, account_sid: &str, sid: &str) -> request::FetchCallRequest {
        request::FetchCallRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Initiates a call redirect or terminates a call
    pub fn update_call(
        &self,
        account_sid: &str,
        sid: &str,
    ) -> request::UpdateCallRequest {
        request::UpdateCallRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Delete a Call record from your account. Once the record is deleted, it will no longer appear in the API and Account Portal logs.
    pub fn delete_call(
        &self,
        account_sid: &str,
        sid: &str,
    ) -> request::DeleteCallRequest {
        request::DeleteCallRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Retrieve a list of all events for a call.
    pub fn list_call_event(
        &self,
        account_sid: &str,
        call_sid: &str,
    ) -> request::ListCallEventRequest {
        request::ListCallEventRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            call_sid: call_sid.to_owned(),
            page: None,
            page_size: None,
            page_token: None,
        }
    }
    ///Fetch a Feedback resource from a call
    pub fn fetch_call_feedback(
        &self,
        account_sid: &str,
        call_sid: &str,
    ) -> request::FetchCallFeedbackRequest {
        request::FetchCallFeedbackRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            call_sid: call_sid.to_owned(),
        }
    }
    ///Update a Feedback resource for a call
    pub fn update_call_feedback(
        &self,
        account_sid: &str,
        call_sid: &str,
    ) -> request::UpdateCallFeedbackRequest {
        request::UpdateCallFeedbackRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            call_sid: call_sid.to_owned(),
        }
    }
    ///Create a FeedbackSummary resource for a call
    pub fn create_call_feedback_summary(
        &self,
        account_sid: &str,
    ) -> request::CreateCallFeedbackSummaryRequest {
        request::CreateCallFeedbackSummaryRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
        }
    }
    ///Fetch a FeedbackSummary resource from a call
    pub fn fetch_call_feedback_summary(
        &self,
        account_sid: &str,
        sid: &str,
    ) -> request::FetchCallFeedbackSummaryRequest {
        request::FetchCallFeedbackSummaryRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Delete a FeedbackSummary resource from a call
    pub fn delete_call_feedback_summary(
        &self,
        account_sid: &str,
        sid: &str,
    ) -> request::DeleteCallFeedbackSummaryRequest {
        request::DeleteCallFeedbackSummaryRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    pub fn fetch_call_notification(
        &self,
        account_sid: &str,
        call_sid: &str,
        sid: &str,
    ) -> request::FetchCallNotificationRequest {
        request::FetchCallNotificationRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            call_sid: call_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    pub fn list_call_notification(
        &self,
        account_sid: &str,
        call_sid: &str,
    ) -> request::ListCallNotificationRequest {
        request::ListCallNotificationRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            call_sid: call_sid.to_owned(),
            log: None,
            message_date: None,
            message_date_gt: None,
            message_date_lt: None,
            page: None,
            page_size: None,
            page_token: None,
        }
    }
    ///Retrieve a list of recordings belonging to the call used to make the request
    pub fn list_call_recording(
        &self,
        account_sid: &str,
        call_sid: &str,
    ) -> request::ListCallRecordingRequest {
        request::ListCallRecordingRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            call_sid: call_sid.to_owned(),
            date_created: None,
            date_created_gt: None,
            date_created_lt: None,
            page: None,
            page_size: None,
            page_token: None,
        }
    }
    ///Create a recording for the call
    pub fn create_call_recording(
        &self,
        account_sid: &str,
        call_sid: &str,
    ) -> request::CreateCallRecordingRequest {
        request::CreateCallRecordingRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            call_sid: call_sid.to_owned(),
        }
    }
    ///Fetch an instance of a recording for a call
    pub fn fetch_call_recording(
        &self,
        account_sid: &str,
        call_sid: &str,
        sid: &str,
    ) -> request::FetchCallRecordingRequest {
        request::FetchCallRecordingRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            call_sid: call_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Changes the status of the recording to paused, stopped, or in-progress. Note: Pass `Twilio.CURRENT` instead of recording sid to reference current active recording.
    pub fn update_call_recording(
        &self,
        account_sid: &str,
        call_sid: &str,
        sid: &str,
    ) -> request::UpdateCallRecordingRequest {
        request::UpdateCallRecordingRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            call_sid: call_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Delete a recording from your account
    pub fn delete_call_recording(
        &self,
        account_sid: &str,
        call_sid: &str,
        sid: &str,
    ) -> request::DeleteCallRecordingRequest {
        request::DeleteCallRecordingRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            call_sid: call_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Fetch an instance of a conference
    pub fn fetch_conference(
        &self,
        account_sid: &str,
        sid: &str,
    ) -> request::FetchConferenceRequest {
        request::FetchConferenceRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    pub fn update_conference(
        &self,
        account_sid: &str,
        sid: &str,
    ) -> request::UpdateConferenceRequest {
        request::UpdateConferenceRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Retrieve a list of conferences belonging to the account used to make the request
    pub fn list_conference(&self, account_sid: &str) -> request::ListConferenceRequest {
        request::ListConferenceRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            date_created: None,
            date_created_gt: None,
            date_created_lt: None,
            date_updated: None,
            date_updated_gt: None,
            date_updated_lt: None,
            friendly_name: None,
            page: None,
            page_size: None,
            page_token: None,
            status: None,
        }
    }
    ///Fetch an instance of a recording for a call
    pub fn fetch_conference_recording(
        &self,
        account_sid: &str,
        conference_sid: &str,
        sid: &str,
    ) -> request::FetchConferenceRecordingRequest {
        request::FetchConferenceRecordingRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            conference_sid: conference_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Changes the status of the recording to paused, stopped, or in-progress. Note: To use `Twilio.CURRENT`, pass it as recording sid.
    pub fn update_conference_recording(
        &self,
        account_sid: &str,
        conference_sid: &str,
        sid: &str,
    ) -> request::UpdateConferenceRecordingRequest {
        request::UpdateConferenceRecordingRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            conference_sid: conference_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Delete a recording from your account
    pub fn delete_conference_recording(
        &self,
        account_sid: &str,
        conference_sid: &str,
        sid: &str,
    ) -> request::DeleteConferenceRecordingRequest {
        request::DeleteConferenceRecordingRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            conference_sid: conference_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Retrieve a list of recordings belonging to the call used to make the request
    pub fn list_conference_recording(
        &self,
        account_sid: &str,
        conference_sid: &str,
    ) -> request::ListConferenceRecordingRequest {
        request::ListConferenceRecordingRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            conference_sid: conference_sid.to_owned(),
            date_created: None,
            date_created_gt: None,
            date_created_lt: None,
            page: None,
            page_size: None,
            page_token: None,
        }
    }
    ///Fetch an instance of a connect-app
    pub fn fetch_connect_app(
        &self,
        account_sid: &str,
        sid: &str,
    ) -> request::FetchConnectAppRequest {
        request::FetchConnectAppRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Update a connect-app with the specified parameters
    pub fn update_connect_app(
        &self,
        account_sid: &str,
        sid: &str,
    ) -> request::UpdateConnectAppRequest {
        request::UpdateConnectAppRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Delete an instance of a connect-app
    pub fn delete_connect_app(
        &self,
        account_sid: &str,
        sid: &str,
    ) -> request::DeleteConnectAppRequest {
        request::DeleteConnectAppRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Retrieve a list of connect-apps belonging to the account used to make the request
    pub fn list_connect_app(&self, account_sid: &str) -> request::ListConnectAppRequest {
        request::ListConnectAppRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            page: None,
            page_size: None,
            page_token: None,
        }
    }
    pub fn list_dependent_phone_number(
        &self,
        account_sid: &str,
        address_sid: &str,
    ) -> request::ListDependentPhoneNumberRequest {
        request::ListDependentPhoneNumberRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            address_sid: address_sid.to_owned(),
            page: None,
            page_size: None,
            page_token: None,
        }
    }
    ///Fetch an incoming-phone-number belonging to the account used to make the request.
    pub fn fetch_incoming_phone_number(
        &self,
        account_sid: &str,
        sid: &str,
    ) -> request::FetchIncomingPhoneNumberRequest {
        request::FetchIncomingPhoneNumberRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Update an incoming-phone-number instance.
    pub fn update_incoming_phone_number(
        &self,
        account_sid: &str,
        sid: &str,
    ) -> request::UpdateIncomingPhoneNumberRequest {
        request::UpdateIncomingPhoneNumberRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Delete a phone-numbers belonging to the account used to make the request.
    pub fn delete_incoming_phone_number(
        &self,
        account_sid: &str,
        sid: &str,
    ) -> request::DeleteIncomingPhoneNumberRequest {
        request::DeleteIncomingPhoneNumberRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Retrieve a list of incoming-phone-numbers belonging to the account used to make the request.
    pub fn list_incoming_phone_number(
        &self,
        account_sid: &str,
    ) -> request::ListIncomingPhoneNumberRequest {
        request::ListIncomingPhoneNumberRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            beta: None,
            friendly_name: None,
            origin: None,
            page: None,
            page_size: None,
            page_token: None,
            phone_number: None,
        }
    }
    ///Purchase a phone-number for the account.
    pub fn create_incoming_phone_number(
        &self,
        account_sid: &str,
    ) -> request::CreateIncomingPhoneNumberRequest {
        request::CreateIncomingPhoneNumberRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
        }
    }
    ///Fetch an instance of an Add-on installation currently assigned to this Number.
    pub fn fetch_incoming_phone_number_assigned_add_on(
        &self,
        account_sid: &str,
        resource_sid: &str,
        sid: &str,
    ) -> request::FetchIncomingPhoneNumberAssignedAddOnRequest {
        request::FetchIncomingPhoneNumberAssignedAddOnRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            resource_sid: resource_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Remove the assignment of an Add-on installation from the Number specified.
    pub fn delete_incoming_phone_number_assigned_add_on(
        &self,
        account_sid: &str,
        resource_sid: &str,
        sid: &str,
    ) -> request::DeleteIncomingPhoneNumberAssignedAddOnRequest {
        request::DeleteIncomingPhoneNumberAssignedAddOnRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            resource_sid: resource_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Retrieve a list of Add-on installations currently assigned to this Number.
    pub fn list_incoming_phone_number_assigned_add_on(
        &self,
        account_sid: &str,
        resource_sid: &str,
    ) -> request::ListIncomingPhoneNumberAssignedAddOnRequest {
        request::ListIncomingPhoneNumberAssignedAddOnRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            page: None,
            page_size: None,
            page_token: None,
            resource_sid: resource_sid.to_owned(),
        }
    }
    ///Assign an Add-on installation to the Number specified.
    pub fn create_incoming_phone_number_assigned_add_on(
        &self,
        account_sid: &str,
        resource_sid: &str,
    ) -> request::CreateIncomingPhoneNumberAssignedAddOnRequest {
        request::CreateIncomingPhoneNumberAssignedAddOnRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            resource_sid: resource_sid.to_owned(),
        }
    }
    ///Fetch an instance of an Extension for the Assigned Add-on.
    pub fn fetch_incoming_phone_number_assigned_add_on_extension(
        &self,
        args: request::FetchIncomingPhoneNumberAssignedAddOnExtensionRequired,
    ) -> request::FetchIncomingPhoneNumberAssignedAddOnExtensionRequest {
        request::FetchIncomingPhoneNumberAssignedAddOnExtensionRequest {
            http_client: &self,
            account_sid: args.account_sid.to_owned(),
            assigned_add_on_sid: args.assigned_add_on_sid.to_owned(),
            resource_sid: args.resource_sid.to_owned(),
            sid: args.sid.to_owned(),
        }
    }
    ///Retrieve a list of Extensions for the Assigned Add-on.
    pub fn list_incoming_phone_number_assigned_add_on_extension(
        &self,
        account_sid: &str,
        assigned_add_on_sid: &str,
        resource_sid: &str,
    ) -> request::ListIncomingPhoneNumberAssignedAddOnExtensionRequest {
        request::ListIncomingPhoneNumberAssignedAddOnExtensionRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            assigned_add_on_sid: assigned_add_on_sid.to_owned(),
            page: None,
            page_size: None,
            page_token: None,
            resource_sid: resource_sid.to_owned(),
        }
    }
    pub fn list_incoming_phone_number_local(
        &self,
        account_sid: &str,
    ) -> request::ListIncomingPhoneNumberLocalRequest {
        request::ListIncomingPhoneNumberLocalRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            beta: None,
            friendly_name: None,
            origin: None,
            page: None,
            page_size: None,
            page_token: None,
            phone_number: None,
        }
    }
    pub fn create_incoming_phone_number_local(
        &self,
        account_sid: &str,
    ) -> request::CreateIncomingPhoneNumberLocalRequest {
        request::CreateIncomingPhoneNumberLocalRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
        }
    }
    pub fn list_incoming_phone_number_mobile(
        &self,
        account_sid: &str,
    ) -> request::ListIncomingPhoneNumberMobileRequest {
        request::ListIncomingPhoneNumberMobileRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            beta: None,
            friendly_name: None,
            origin: None,
            page: None,
            page_size: None,
            page_token: None,
            phone_number: None,
        }
    }
    pub fn create_incoming_phone_number_mobile(
        &self,
        account_sid: &str,
    ) -> request::CreateIncomingPhoneNumberMobileRequest {
        request::CreateIncomingPhoneNumberMobileRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
        }
    }
    pub fn list_incoming_phone_number_toll_free(
        &self,
        account_sid: &str,
    ) -> request::ListIncomingPhoneNumberTollFreeRequest {
        request::ListIncomingPhoneNumberTollFreeRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            beta: None,
            friendly_name: None,
            origin: None,
            page: None,
            page_size: None,
            page_token: None,
            phone_number: None,
        }
    }
    pub fn create_incoming_phone_number_toll_free(
        &self,
        account_sid: &str,
    ) -> request::CreateIncomingPhoneNumberTollFreeRequest {
        request::CreateIncomingPhoneNumberTollFreeRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
        }
    }
    pub fn fetch_key(&self, account_sid: &str, sid: &str) -> request::FetchKeyRequest {
        request::FetchKeyRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    pub fn update_key(&self, account_sid: &str, sid: &str) -> request::UpdateKeyRequest {
        request::UpdateKeyRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    pub fn delete_key(&self, account_sid: &str, sid: &str) -> request::DeleteKeyRequest {
        request::DeleteKeyRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    pub fn list_key(&self, account_sid: &str) -> request::ListKeyRequest {
        request::ListKeyRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            page: None,
            page_size: None,
            page_token: None,
        }
    }
    pub fn create_new_key(&self, account_sid: &str) -> request::CreateNewKeyRequest {
        request::CreateNewKeyRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
        }
    }
    ///Fetch a single media instance belonging to the account used to make the request
    pub fn fetch_media(
        &self,
        account_sid: &str,
        message_sid: &str,
        sid: &str,
    ) -> request::FetchMediaRequest {
        request::FetchMediaRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            message_sid: message_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Delete media from your account. Once delete, you will no longer be billed
    pub fn delete_media(
        &self,
        account_sid: &str,
        message_sid: &str,
        sid: &str,
    ) -> request::DeleteMediaRequest {
        request::DeleteMediaRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            message_sid: message_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Retrieve a list of Media resources belonging to the account used to make the request
    pub fn list_media(
        &self,
        account_sid: &str,
        message_sid: &str,
    ) -> request::ListMediaRequest {
        request::ListMediaRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            date_created: None,
            date_created_gt: None,
            date_created_lt: None,
            message_sid: message_sid.to_owned(),
            page: None,
            page_size: None,
            page_token: None,
        }
    }
    ///Fetch a specific member from the queue
    pub fn fetch_member(
        &self,
        account_sid: &str,
        call_sid: &str,
        queue_sid: &str,
    ) -> request::FetchMemberRequest {
        request::FetchMemberRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            call_sid: call_sid.to_owned(),
            queue_sid: queue_sid.to_owned(),
        }
    }
    ///Dequeue a member from a queue and have the member's call begin executing the TwiML document at that URL
    pub fn update_member(
        &self,
        account_sid: &str,
        call_sid: &str,
        queue_sid: &str,
    ) -> request::UpdateMemberRequest {
        request::UpdateMemberRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            call_sid: call_sid.to_owned(),
            queue_sid: queue_sid.to_owned(),
        }
    }
    ///Retrieve the members of the queue
    pub fn list_member(
        &self,
        account_sid: &str,
        queue_sid: &str,
    ) -> request::ListMemberRequest {
        request::ListMemberRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            page: None,
            page_size: None,
            page_token: None,
            queue_sid: queue_sid.to_owned(),
        }
    }
    ///Retrieve a list of messages belonging to the account used to make the request
    pub fn list_message(&self, account_sid: &str) -> request::ListMessageRequest {
        request::ListMessageRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            date_sent: None,
            date_sent_gt: None,
            date_sent_lt: None,
            from: None,
            page: None,
            page_size: None,
            page_token: None,
            to: None,
        }
    }
    ///Send a message from the account used to make the request
    pub fn create_message(&self, account_sid: &str) -> request::CreateMessageRequest {
        request::CreateMessageRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
        }
    }
    ///Fetch a message belonging to the account used to make the request
    pub fn fetch_message(
        &self,
        account_sid: &str,
        sid: &str,
    ) -> request::FetchMessageRequest {
        request::FetchMessageRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///To redact a message-body from a post-flight message record, post to the message instance resource with an empty body
    pub fn update_message(
        &self,
        account_sid: &str,
        sid: &str,
    ) -> request::UpdateMessageRequest {
        request::UpdateMessageRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Deletes a message record from your account
    pub fn delete_message(
        &self,
        account_sid: &str,
        sid: &str,
    ) -> request::DeleteMessageRequest {
        request::DeleteMessageRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    pub fn create_message_feedback(
        &self,
        account_sid: &str,
        message_sid: &str,
    ) -> request::CreateMessageFeedbackRequest {
        request::CreateMessageFeedbackRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            message_sid: message_sid.to_owned(),
        }
    }
    pub fn list_signing_key(&self, account_sid: &str) -> request::ListSigningKeyRequest {
        request::ListSigningKeyRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            page: None,
            page_size: None,
            page_token: None,
        }
    }
    ///Create a new Signing Key for the account making the request.
    pub fn create_new_signing_key(
        &self,
        account_sid: &str,
    ) -> request::CreateNewSigningKeyRequest {
        request::CreateNewSigningKeyRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
        }
    }
    ///Fetch a notification belonging to the account used to make the request
    pub fn fetch_notification(
        &self,
        account_sid: &str,
        sid: &str,
    ) -> request::FetchNotificationRequest {
        request::FetchNotificationRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Retrieve a list of notifications belonging to the account used to make the request
    pub fn list_notification(
        &self,
        account_sid: &str,
    ) -> request::ListNotificationRequest {
        request::ListNotificationRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            log: None,
            message_date: None,
            message_date_gt: None,
            message_date_lt: None,
            page: None,
            page_size: None,
            page_token: None,
        }
    }
    ///Fetch an outgoing-caller-id belonging to the account used to make the request
    pub fn fetch_outgoing_caller_id(
        &self,
        account_sid: &str,
        sid: &str,
    ) -> request::FetchOutgoingCallerIdRequest {
        request::FetchOutgoingCallerIdRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Updates the caller-id
    pub fn update_outgoing_caller_id(
        &self,
        account_sid: &str,
        sid: &str,
    ) -> request::UpdateOutgoingCallerIdRequest {
        request::UpdateOutgoingCallerIdRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Delete the caller-id specified from the account
    pub fn delete_outgoing_caller_id(
        &self,
        account_sid: &str,
        sid: &str,
    ) -> request::DeleteOutgoingCallerIdRequest {
        request::DeleteOutgoingCallerIdRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Retrieve a list of outgoing-caller-ids belonging to the account used to make the request
    pub fn list_outgoing_caller_id(
        &self,
        account_sid: &str,
    ) -> request::ListOutgoingCallerIdRequest {
        request::ListOutgoingCallerIdRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            friendly_name: None,
            page: None,
            page_size: None,
            page_token: None,
            phone_number: None,
        }
    }
    pub fn create_validation_request(
        &self,
        account_sid: &str,
    ) -> request::CreateValidationRequestRequest {
        request::CreateValidationRequestRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
        }
    }
    ///Fetch an instance of a participant
    pub fn fetch_participant(
        &self,
        account_sid: &str,
        call_sid: &str,
        conference_sid: &str,
    ) -> request::FetchParticipantRequest {
        request::FetchParticipantRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            call_sid: call_sid.to_owned(),
            conference_sid: conference_sid.to_owned(),
        }
    }
    ///Update the properties of the participant
    pub fn update_participant(
        &self,
        account_sid: &str,
        call_sid: &str,
        conference_sid: &str,
    ) -> request::UpdateParticipantRequest {
        request::UpdateParticipantRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            call_sid: call_sid.to_owned(),
            conference_sid: conference_sid.to_owned(),
        }
    }
    ///Kick a participant from a given conference
    pub fn delete_participant(
        &self,
        account_sid: &str,
        call_sid: &str,
        conference_sid: &str,
    ) -> request::DeleteParticipantRequest {
        request::DeleteParticipantRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            call_sid: call_sid.to_owned(),
            conference_sid: conference_sid.to_owned(),
        }
    }
    ///Retrieve a list of participants belonging to the account used to make the request
    pub fn list_participant(
        &self,
        account_sid: &str,
        conference_sid: &str,
    ) -> request::ListParticipantRequest {
        request::ListParticipantRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            coaching: None,
            conference_sid: conference_sid.to_owned(),
            hold: None,
            muted: None,
            page: None,
            page_size: None,
            page_token: None,
        }
    }
    pub fn create_participant(
        &self,
        account_sid: &str,
        conference_sid: &str,
    ) -> request::CreateParticipantRequest {
        request::CreateParticipantRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            conference_sid: conference_sid.to_owned(),
        }
    }
    ///create an instance of payments. This will start a new payments session
    pub fn create_payments(
        &self,
        account_sid: &str,
        call_sid: &str,
    ) -> request::CreatePaymentsRequest {
        request::CreatePaymentsRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            call_sid: call_sid.to_owned(),
        }
    }
    ///update an instance of payments with different phases of payment flows.
    pub fn update_payments(
        &self,
        account_sid: &str,
        call_sid: &str,
        sid: &str,
    ) -> request::UpdatePaymentsRequest {
        request::UpdatePaymentsRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            call_sid: call_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Fetch an instance of a queue identified by the QueueSid
    pub fn fetch_queue(
        &self,
        account_sid: &str,
        sid: &str,
    ) -> request::FetchQueueRequest {
        request::FetchQueueRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Update the queue with the new parameters
    pub fn update_queue(
        &self,
        account_sid: &str,
        sid: &str,
    ) -> request::UpdateQueueRequest {
        request::UpdateQueueRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Remove an empty queue
    pub fn delete_queue(
        &self,
        account_sid: &str,
        sid: &str,
    ) -> request::DeleteQueueRequest {
        request::DeleteQueueRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Retrieve a list of queues belonging to the account used to make the request
    pub fn list_queue(&self, account_sid: &str) -> request::ListQueueRequest {
        request::ListQueueRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            page: None,
            page_size: None,
            page_token: None,
        }
    }
    ///Create a queue
    pub fn create_queue(&self, account_sid: &str) -> request::CreateQueueRequest {
        request::CreateQueueRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
        }
    }
    ///Fetch an instance of a recording
    pub fn fetch_recording(
        &self,
        account_sid: &str,
        sid: &str,
    ) -> request::FetchRecordingRequest {
        request::FetchRecordingRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            include_soft_deleted: None,
            sid: sid.to_owned(),
        }
    }
    ///Delete a recording from your account
    pub fn delete_recording(
        &self,
        account_sid: &str,
        sid: &str,
    ) -> request::DeleteRecordingRequest {
        request::DeleteRecordingRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Retrieve a list of recordings belonging to the account used to make the request
    pub fn list_recording(&self, account_sid: &str) -> request::ListRecordingRequest {
        request::ListRecordingRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            call_sid: None,
            conference_sid: None,
            date_created: None,
            date_created_gt: None,
            date_created_lt: None,
            include_soft_deleted: None,
            page: None,
            page_size: None,
            page_token: None,
        }
    }
    ///Fetch an instance of an AddOnResult
    pub fn fetch_recording_add_on_result(
        &self,
        account_sid: &str,
        reference_sid: &str,
        sid: &str,
    ) -> request::FetchRecordingAddOnResultRequest {
        request::FetchRecordingAddOnResultRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            reference_sid: reference_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Delete a result and purge all associated Payloads
    pub fn delete_recording_add_on_result(
        &self,
        account_sid: &str,
        reference_sid: &str,
        sid: &str,
    ) -> request::DeleteRecordingAddOnResultRequest {
        request::DeleteRecordingAddOnResultRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            reference_sid: reference_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Retrieve a list of results belonging to the recording
    pub fn list_recording_add_on_result(
        &self,
        account_sid: &str,
        reference_sid: &str,
    ) -> request::ListRecordingAddOnResultRequest {
        request::ListRecordingAddOnResultRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            page: None,
            page_size: None,
            page_token: None,
            reference_sid: reference_sid.to_owned(),
        }
    }
    ///Fetch an instance of a result payload
    pub fn fetch_recording_add_on_result_payload(
        &self,
        args: request::FetchRecordingAddOnResultPayloadRequired,
    ) -> request::FetchRecordingAddOnResultPayloadRequest {
        request::FetchRecordingAddOnResultPayloadRequest {
            http_client: &self,
            account_sid: args.account_sid.to_owned(),
            add_on_result_sid: args.add_on_result_sid.to_owned(),
            reference_sid: args.reference_sid.to_owned(),
            sid: args.sid.to_owned(),
        }
    }
    ///Delete a payload from the result along with all associated Data
    pub fn delete_recording_add_on_result_payload(
        &self,
        args: request::DeleteRecordingAddOnResultPayloadRequired,
    ) -> request::DeleteRecordingAddOnResultPayloadRequest {
        request::DeleteRecordingAddOnResultPayloadRequest {
            http_client: &self,
            account_sid: args.account_sid.to_owned(),
            add_on_result_sid: args.add_on_result_sid.to_owned(),
            reference_sid: args.reference_sid.to_owned(),
            sid: args.sid.to_owned(),
        }
    }
    ///Retrieve a list of payloads belonging to the AddOnResult
    pub fn list_recording_add_on_result_payload(
        &self,
        account_sid: &str,
        add_on_result_sid: &str,
        reference_sid: &str,
    ) -> request::ListRecordingAddOnResultPayloadRequest {
        request::ListRecordingAddOnResultPayloadRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            add_on_result_sid: add_on_result_sid.to_owned(),
            page: None,
            page_size: None,
            page_token: None,
            reference_sid: reference_sid.to_owned(),
        }
    }
    pub fn fetch_recording_transcription(
        &self,
        account_sid: &str,
        recording_sid: &str,
        sid: &str,
    ) -> request::FetchRecordingTranscriptionRequest {
        request::FetchRecordingTranscriptionRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            recording_sid: recording_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    pub fn delete_recording_transcription(
        &self,
        account_sid: &str,
        recording_sid: &str,
        sid: &str,
    ) -> request::DeleteRecordingTranscriptionRequest {
        request::DeleteRecordingTranscriptionRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            recording_sid: recording_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    pub fn list_recording_transcription(
        &self,
        account_sid: &str,
        recording_sid: &str,
    ) -> request::ListRecordingTranscriptionRequest {
        request::ListRecordingTranscriptionRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            page: None,
            page_size: None,
            page_token: None,
            recording_sid: recording_sid.to_owned(),
        }
    }
    ///Fetch an instance of a short code
    pub fn fetch_short_code(
        &self,
        account_sid: &str,
        sid: &str,
    ) -> request::FetchShortCodeRequest {
        request::FetchShortCodeRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Update a short code with the following parameters
    pub fn update_short_code(
        &self,
        account_sid: &str,
        sid: &str,
    ) -> request::UpdateShortCodeRequest {
        request::UpdateShortCodeRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Retrieve a list of short-codes belonging to the account used to make the request
    pub fn list_short_code(&self, account_sid: &str) -> request::ListShortCodeRequest {
        request::ListShortCodeRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            friendly_name: None,
            page: None,
            page_size: None,
            page_token: None,
            short_code: None,
        }
    }
    pub fn fetch_signing_key(
        &self,
        account_sid: &str,
        sid: &str,
    ) -> request::FetchSigningKeyRequest {
        request::FetchSigningKeyRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    pub fn update_signing_key(
        &self,
        account_sid: &str,
        sid: &str,
    ) -> request::UpdateSigningKeyRequest {
        request::UpdateSigningKeyRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    pub fn delete_signing_key(
        &self,
        account_sid: &str,
        sid: &str,
    ) -> request::DeleteSigningKeyRequest {
        request::DeleteSigningKeyRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Retrieve a list of credential list mappings belonging to the domain used in the request
    pub fn list_sip_auth_calls_credential_list_mapping(
        &self,
        account_sid: &str,
        domain_sid: &str,
    ) -> request::ListSipAuthCallsCredentialListMappingRequest {
        request::ListSipAuthCallsCredentialListMappingRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            domain_sid: domain_sid.to_owned(),
            page: None,
            page_size: None,
            page_token: None,
        }
    }
    ///Create a new credential list mapping resource
    pub fn create_sip_auth_calls_credential_list_mapping(
        &self,
        account_sid: &str,
        domain_sid: &str,
    ) -> request::CreateSipAuthCallsCredentialListMappingRequest {
        request::CreateSipAuthCallsCredentialListMappingRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            domain_sid: domain_sid.to_owned(),
        }
    }
    ///Fetch a specific instance of a credential list mapping
    pub fn fetch_sip_auth_calls_credential_list_mapping(
        &self,
        account_sid: &str,
        domain_sid: &str,
        sid: &str,
    ) -> request::FetchSipAuthCallsCredentialListMappingRequest {
        request::FetchSipAuthCallsCredentialListMappingRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            domain_sid: domain_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Delete a credential list mapping from the requested domain
    pub fn delete_sip_auth_calls_credential_list_mapping(
        &self,
        account_sid: &str,
        domain_sid: &str,
        sid: &str,
    ) -> request::DeleteSipAuthCallsCredentialListMappingRequest {
        request::DeleteSipAuthCallsCredentialListMappingRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            domain_sid: domain_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Retrieve a list of IP Access Control List mappings belonging to the domain used in the request
    pub fn list_sip_auth_calls_ip_access_control_list_mapping(
        &self,
        account_sid: &str,
        domain_sid: &str,
    ) -> request::ListSipAuthCallsIpAccessControlListMappingRequest {
        request::ListSipAuthCallsIpAccessControlListMappingRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            domain_sid: domain_sid.to_owned(),
            page: None,
            page_size: None,
            page_token: None,
        }
    }
    ///Create a new IP Access Control List mapping
    pub fn create_sip_auth_calls_ip_access_control_list_mapping(
        &self,
        account_sid: &str,
        domain_sid: &str,
    ) -> request::CreateSipAuthCallsIpAccessControlListMappingRequest {
        request::CreateSipAuthCallsIpAccessControlListMappingRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            domain_sid: domain_sid.to_owned(),
        }
    }
    ///Fetch a specific instance of an IP Access Control List mapping
    pub fn fetch_sip_auth_calls_ip_access_control_list_mapping(
        &self,
        account_sid: &str,
        domain_sid: &str,
        sid: &str,
    ) -> request::FetchSipAuthCallsIpAccessControlListMappingRequest {
        request::FetchSipAuthCallsIpAccessControlListMappingRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            domain_sid: domain_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Delete an IP Access Control List mapping from the requested domain
    pub fn delete_sip_auth_calls_ip_access_control_list_mapping(
        &self,
        account_sid: &str,
        domain_sid: &str,
        sid: &str,
    ) -> request::DeleteSipAuthCallsIpAccessControlListMappingRequest {
        request::DeleteSipAuthCallsIpAccessControlListMappingRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            domain_sid: domain_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Retrieve a list of credential list mappings belonging to the domain used in the request
    pub fn list_sip_auth_registrations_credential_list_mapping(
        &self,
        account_sid: &str,
        domain_sid: &str,
    ) -> request::ListSipAuthRegistrationsCredentialListMappingRequest {
        request::ListSipAuthRegistrationsCredentialListMappingRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            domain_sid: domain_sid.to_owned(),
            page: None,
            page_size: None,
            page_token: None,
        }
    }
    ///Create a new credential list mapping resource
    pub fn create_sip_auth_registrations_credential_list_mapping(
        &self,
        account_sid: &str,
        domain_sid: &str,
    ) -> request::CreateSipAuthRegistrationsCredentialListMappingRequest {
        request::CreateSipAuthRegistrationsCredentialListMappingRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            domain_sid: domain_sid.to_owned(),
        }
    }
    ///Fetch a specific instance of a credential list mapping
    pub fn fetch_sip_auth_registrations_credential_list_mapping(
        &self,
        account_sid: &str,
        domain_sid: &str,
        sid: &str,
    ) -> request::FetchSipAuthRegistrationsCredentialListMappingRequest {
        request::FetchSipAuthRegistrationsCredentialListMappingRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            domain_sid: domain_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Delete a credential list mapping from the requested domain
    pub fn delete_sip_auth_registrations_credential_list_mapping(
        &self,
        account_sid: &str,
        domain_sid: &str,
        sid: &str,
    ) -> request::DeleteSipAuthRegistrationsCredentialListMappingRequest {
        request::DeleteSipAuthRegistrationsCredentialListMappingRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            domain_sid: domain_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Retrieve a list of credentials.
    pub fn list_sip_credential(
        &self,
        account_sid: &str,
        credential_list_sid: &str,
    ) -> request::ListSipCredentialRequest {
        request::ListSipCredentialRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            credential_list_sid: credential_list_sid.to_owned(),
            page: None,
            page_size: None,
            page_token: None,
        }
    }
    ///Create a new credential resource.
    pub fn create_sip_credential(
        &self,
        account_sid: &str,
        credential_list_sid: &str,
    ) -> request::CreateSipCredentialRequest {
        request::CreateSipCredentialRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            credential_list_sid: credential_list_sid.to_owned(),
        }
    }
    ///Fetch a single credential.
    pub fn fetch_sip_credential(
        &self,
        account_sid: &str,
        credential_list_sid: &str,
        sid: &str,
    ) -> request::FetchSipCredentialRequest {
        request::FetchSipCredentialRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            credential_list_sid: credential_list_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Update a credential resource.
    pub fn update_sip_credential(
        &self,
        account_sid: &str,
        credential_list_sid: &str,
        sid: &str,
    ) -> request::UpdateSipCredentialRequest {
        request::UpdateSipCredentialRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            credential_list_sid: credential_list_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Delete a credential resource.
    pub fn delete_sip_credential(
        &self,
        account_sid: &str,
        credential_list_sid: &str,
        sid: &str,
    ) -> request::DeleteSipCredentialRequest {
        request::DeleteSipCredentialRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            credential_list_sid: credential_list_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Get All Credential Lists
    pub fn list_sip_credential_list(
        &self,
        account_sid: &str,
    ) -> request::ListSipCredentialListRequest {
        request::ListSipCredentialListRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            page: None,
            page_size: None,
            page_token: None,
        }
    }
    ///Create a Credential List
    pub fn create_sip_credential_list(
        &self,
        account_sid: &str,
    ) -> request::CreateSipCredentialListRequest {
        request::CreateSipCredentialListRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
        }
    }
    ///Get a Credential List
    pub fn fetch_sip_credential_list(
        &self,
        account_sid: &str,
        sid: &str,
    ) -> request::FetchSipCredentialListRequest {
        request::FetchSipCredentialListRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Update a Credential List
    pub fn update_sip_credential_list(
        &self,
        account_sid: &str,
        sid: &str,
    ) -> request::UpdateSipCredentialListRequest {
        request::UpdateSipCredentialListRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Delete a Credential List
    pub fn delete_sip_credential_list(
        &self,
        account_sid: &str,
        sid: &str,
    ) -> request::DeleteSipCredentialListRequest {
        request::DeleteSipCredentialListRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Read multiple CredentialListMapping resources from an account.
    pub fn list_sip_credential_list_mapping(
        &self,
        account_sid: &str,
        domain_sid: &str,
    ) -> request::ListSipCredentialListMappingRequest {
        request::ListSipCredentialListMappingRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            domain_sid: domain_sid.to_owned(),
            page: None,
            page_size: None,
            page_token: None,
        }
    }
    ///Create a CredentialListMapping resource for an account.
    pub fn create_sip_credential_list_mapping(
        &self,
        account_sid: &str,
        domain_sid: &str,
    ) -> request::CreateSipCredentialListMappingRequest {
        request::CreateSipCredentialListMappingRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            domain_sid: domain_sid.to_owned(),
        }
    }
    ///Fetch a single CredentialListMapping resource from an account.
    pub fn fetch_sip_credential_list_mapping(
        &self,
        account_sid: &str,
        domain_sid: &str,
        sid: &str,
    ) -> request::FetchSipCredentialListMappingRequest {
        request::FetchSipCredentialListMappingRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            domain_sid: domain_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Delete a CredentialListMapping resource from an account.
    pub fn delete_sip_credential_list_mapping(
        &self,
        account_sid: &str,
        domain_sid: &str,
        sid: &str,
    ) -> request::DeleteSipCredentialListMappingRequest {
        request::DeleteSipCredentialListMappingRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            domain_sid: domain_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Retrieve a list of domains belonging to the account used to make the request
    pub fn list_sip_domain(&self, account_sid: &str) -> request::ListSipDomainRequest {
        request::ListSipDomainRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            page: None,
            page_size: None,
            page_token: None,
        }
    }
    ///Create a new Domain
    pub fn create_sip_domain(
        &self,
        account_sid: &str,
    ) -> request::CreateSipDomainRequest {
        request::CreateSipDomainRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
        }
    }
    ///Fetch an instance of a Domain
    pub fn fetch_sip_domain(
        &self,
        account_sid: &str,
        sid: &str,
    ) -> request::FetchSipDomainRequest {
        request::FetchSipDomainRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Update the attributes of a domain
    pub fn update_sip_domain(
        &self,
        account_sid: &str,
        sid: &str,
    ) -> request::UpdateSipDomainRequest {
        request::UpdateSipDomainRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Delete an instance of a Domain
    pub fn delete_sip_domain(
        &self,
        account_sid: &str,
        sid: &str,
    ) -> request::DeleteSipDomainRequest {
        request::DeleteSipDomainRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Retrieve a list of IpAccessControlLists that belong to the account used to make the request
    pub fn list_sip_ip_access_control_list(
        &self,
        account_sid: &str,
    ) -> request::ListSipIpAccessControlListRequest {
        request::ListSipIpAccessControlListRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            page: None,
            page_size: None,
            page_token: None,
        }
    }
    ///Create a new IpAccessControlList resource
    pub fn create_sip_ip_access_control_list(
        &self,
        account_sid: &str,
    ) -> request::CreateSipIpAccessControlListRequest {
        request::CreateSipIpAccessControlListRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
        }
    }
    ///Fetch a specific instance of an IpAccessControlList
    pub fn fetch_sip_ip_access_control_list(
        &self,
        account_sid: &str,
        sid: &str,
    ) -> request::FetchSipIpAccessControlListRequest {
        request::FetchSipIpAccessControlListRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Rename an IpAccessControlList
    pub fn update_sip_ip_access_control_list(
        &self,
        account_sid: &str,
        sid: &str,
    ) -> request::UpdateSipIpAccessControlListRequest {
        request::UpdateSipIpAccessControlListRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Delete an IpAccessControlList from the requested account
    pub fn delete_sip_ip_access_control_list(
        &self,
        account_sid: &str,
        sid: &str,
    ) -> request::DeleteSipIpAccessControlListRequest {
        request::DeleteSipIpAccessControlListRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Fetch an IpAccessControlListMapping resource.
    pub fn fetch_sip_ip_access_control_list_mapping(
        &self,
        account_sid: &str,
        domain_sid: &str,
        sid: &str,
    ) -> request::FetchSipIpAccessControlListMappingRequest {
        request::FetchSipIpAccessControlListMappingRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            domain_sid: domain_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Delete an IpAccessControlListMapping resource.
    pub fn delete_sip_ip_access_control_list_mapping(
        &self,
        account_sid: &str,
        domain_sid: &str,
        sid: &str,
    ) -> request::DeleteSipIpAccessControlListMappingRequest {
        request::DeleteSipIpAccessControlListMappingRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            domain_sid: domain_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Retrieve a list of IpAccessControlListMapping resources.
    pub fn list_sip_ip_access_control_list_mapping(
        &self,
        account_sid: &str,
        domain_sid: &str,
    ) -> request::ListSipIpAccessControlListMappingRequest {
        request::ListSipIpAccessControlListMappingRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            domain_sid: domain_sid.to_owned(),
            page: None,
            page_size: None,
            page_token: None,
        }
    }
    ///Create a new IpAccessControlListMapping resource.
    pub fn create_sip_ip_access_control_list_mapping(
        &self,
        account_sid: &str,
        domain_sid: &str,
    ) -> request::CreateSipIpAccessControlListMappingRequest {
        request::CreateSipIpAccessControlListMappingRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            domain_sid: domain_sid.to_owned(),
        }
    }
    ///Read multiple IpAddress resources.
    pub fn list_sip_ip_address(
        &self,
        account_sid: &str,
        ip_access_control_list_sid: &str,
    ) -> request::ListSipIpAddressRequest {
        request::ListSipIpAddressRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            ip_access_control_list_sid: ip_access_control_list_sid.to_owned(),
            page: None,
            page_size: None,
            page_token: None,
        }
    }
    ///Create a new IpAddress resource.
    pub fn create_sip_ip_address(
        &self,
        account_sid: &str,
        ip_access_control_list_sid: &str,
    ) -> request::CreateSipIpAddressRequest {
        request::CreateSipIpAddressRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            ip_access_control_list_sid: ip_access_control_list_sid.to_owned(),
        }
    }
    ///Read one IpAddress resource.
    pub fn fetch_sip_ip_address(
        &self,
        account_sid: &str,
        ip_access_control_list_sid: &str,
        sid: &str,
    ) -> request::FetchSipIpAddressRequest {
        request::FetchSipIpAddressRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            ip_access_control_list_sid: ip_access_control_list_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Update an IpAddress resource.
    pub fn update_sip_ip_address(
        &self,
        account_sid: &str,
        ip_access_control_list_sid: &str,
        sid: &str,
    ) -> request::UpdateSipIpAddressRequest {
        request::UpdateSipIpAddressRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            ip_access_control_list_sid: ip_access_control_list_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Delete an IpAddress resource.
    pub fn delete_sip_ip_address(
        &self,
        account_sid: &str,
        ip_access_control_list_sid: &str,
        sid: &str,
    ) -> request::DeleteSipIpAddressRequest {
        request::DeleteSipIpAddressRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            ip_access_control_list_sid: ip_access_control_list_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Create a Siprec
    pub fn create_siprec(
        &self,
        account_sid: &str,
        call_sid: &str,
    ) -> request::CreateSiprecRequest {
        request::CreateSiprecRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            call_sid: call_sid.to_owned(),
        }
    }
    ///Stop a Siprec using either the SID of the Siprec resource or the `name` used when creating the resource
    pub fn update_siprec(
        &self,
        account_sid: &str,
        call_sid: &str,
        sid: &str,
    ) -> request::UpdateSiprecRequest {
        request::UpdateSiprecRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            call_sid: call_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Create a Stream
    pub fn create_stream(
        &self,
        account_sid: &str,
        call_sid: &str,
    ) -> request::CreateStreamRequest {
        request::CreateStreamRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            call_sid: call_sid.to_owned(),
        }
    }
    ///Stop a Stream using either the SID of the Stream resource or the `name` used when creating the resource
    pub fn update_stream(
        &self,
        account_sid: &str,
        call_sid: &str,
        sid: &str,
    ) -> request::UpdateStreamRequest {
        request::UpdateStreamRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            call_sid: call_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Create a new token for ICE servers
    pub fn create_token(&self, account_sid: &str) -> request::CreateTokenRequest {
        request::CreateTokenRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
        }
    }
    ///Fetch an instance of a Transcription
    pub fn fetch_transcription(
        &self,
        account_sid: &str,
        sid: &str,
    ) -> request::FetchTranscriptionRequest {
        request::FetchTranscriptionRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Delete a transcription from the account used to make the request
    pub fn delete_transcription(
        &self,
        account_sid: &str,
        sid: &str,
    ) -> request::DeleteTranscriptionRequest {
        request::DeleteTranscriptionRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Retrieve a list of transcriptions belonging to the account used to make the request
    pub fn list_transcription(
        &self,
        account_sid: &str,
    ) -> request::ListTranscriptionRequest {
        request::ListTranscriptionRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            page: None,
            page_size: None,
            page_token: None,
        }
    }
    ///Retrieve a list of usage-records belonging to the account used to make the request
    pub fn list_usage_record(
        &self,
        account_sid: &str,
    ) -> request::ListUsageRecordRequest {
        request::ListUsageRecordRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            category: None,
            end_date: None,
            include_subaccounts: None,
            page: None,
            page_size: None,
            page_token: None,
            start_date: None,
        }
    }
    pub fn list_usage_record_all_time(
        &self,
        account_sid: &str,
    ) -> request::ListUsageRecordAllTimeRequest {
        request::ListUsageRecordAllTimeRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            category: None,
            end_date: None,
            include_subaccounts: None,
            page: None,
            page_size: None,
            page_token: None,
            start_date: None,
        }
    }
    pub fn list_usage_record_daily(
        &self,
        account_sid: &str,
    ) -> request::ListUsageRecordDailyRequest {
        request::ListUsageRecordDailyRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            category: None,
            end_date: None,
            include_subaccounts: None,
            page: None,
            page_size: None,
            page_token: None,
            start_date: None,
        }
    }
    pub fn list_usage_record_last_month(
        &self,
        account_sid: &str,
    ) -> request::ListUsageRecordLastMonthRequest {
        request::ListUsageRecordLastMonthRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            category: None,
            end_date: None,
            include_subaccounts: None,
            page: None,
            page_size: None,
            page_token: None,
            start_date: None,
        }
    }
    pub fn list_usage_record_monthly(
        &self,
        account_sid: &str,
    ) -> request::ListUsageRecordMonthlyRequest {
        request::ListUsageRecordMonthlyRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            category: None,
            end_date: None,
            include_subaccounts: None,
            page: None,
            page_size: None,
            page_token: None,
            start_date: None,
        }
    }
    pub fn list_usage_record_this_month(
        &self,
        account_sid: &str,
    ) -> request::ListUsageRecordThisMonthRequest {
        request::ListUsageRecordThisMonthRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            category: None,
            end_date: None,
            include_subaccounts: None,
            page: None,
            page_size: None,
            page_token: None,
            start_date: None,
        }
    }
    pub fn list_usage_record_today(
        &self,
        account_sid: &str,
    ) -> request::ListUsageRecordTodayRequest {
        request::ListUsageRecordTodayRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            category: None,
            end_date: None,
            include_subaccounts: None,
            page: None,
            page_size: None,
            page_token: None,
            start_date: None,
        }
    }
    pub fn list_usage_record_yearly(
        &self,
        account_sid: &str,
    ) -> request::ListUsageRecordYearlyRequest {
        request::ListUsageRecordYearlyRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            category: None,
            end_date: None,
            include_subaccounts: None,
            page: None,
            page_size: None,
            page_token: None,
            start_date: None,
        }
    }
    pub fn list_usage_record_yesterday(
        &self,
        account_sid: &str,
    ) -> request::ListUsageRecordYesterdayRequest {
        request::ListUsageRecordYesterdayRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            category: None,
            end_date: None,
            include_subaccounts: None,
            page: None,
            page_size: None,
            page_token: None,
            start_date: None,
        }
    }
    ///Fetch and instance of a usage-trigger
    pub fn fetch_usage_trigger(
        &self,
        account_sid: &str,
        sid: &str,
    ) -> request::FetchUsageTriggerRequest {
        request::FetchUsageTriggerRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Update an instance of a usage trigger
    pub fn update_usage_trigger(
        &self,
        account_sid: &str,
        sid: &str,
    ) -> request::UpdateUsageTriggerRequest {
        request::UpdateUsageTriggerRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    pub fn delete_usage_trigger(
        &self,
        account_sid: &str,
        sid: &str,
    ) -> request::DeleteUsageTriggerRequest {
        request::DeleteUsageTriggerRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
    ///Retrieve a list of usage-triggers belonging to the account used to make the request
    pub fn list_usage_trigger(
        &self,
        account_sid: &str,
    ) -> request::ListUsageTriggerRequest {
        request::ListUsageTriggerRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            page: None,
            page_size: None,
            page_token: None,
            recurring: None,
            trigger_by: None,
            usage_category: None,
        }
    }
    ///Create a new UsageTrigger
    pub fn create_usage_trigger(
        &self,
        account_sid: &str,
    ) -> request::CreateUsageTriggerRequest {
        request::CreateUsageTriggerRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
        }
    }
    ///Create a new User Defined Message for the given Call SID.
    pub fn create_user_defined_message(
        &self,
        account_sid: &str,
        call_sid: &str,
    ) -> request::CreateUserDefinedMessageRequest {
        request::CreateUserDefinedMessageRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            call_sid: call_sid.to_owned(),
        }
    }
    ///Subscribe to User Defined Messages for a given Call SID.
    pub fn create_user_defined_message_subscription(
        &self,
        account_sid: &str,
        call_sid: &str,
    ) -> request::CreateUserDefinedMessageSubscriptionRequest {
        request::CreateUserDefinedMessageSubscriptionRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            call_sid: call_sid.to_owned(),
        }
    }
    ///Delete a specific User Defined Message Subscription.
    pub fn delete_user_defined_message_subscription(
        &self,
        account_sid: &str,
        call_sid: &str,
        sid: &str,
    ) -> request::DeleteUserDefinedMessageSubscriptionRequest {
        request::DeleteUserDefinedMessageSubscriptionRequest {
            http_client: &self,
            account_sid: account_sid.to_owned(),
            call_sid: call_sid.to_owned(),
            sid: sid.to_owned(),
        }
    }
}
pub enum TwilioAuthentication {
    BasicAuth { basic_auth: String },
}
impl TwilioAuthentication {
    pub fn from_env() -> Self {
        Self::BasicAuth {
            basic_auth: std::env::var("TWILIO_BASIC_AUTH")
                .expect("Environment variable TWILIO_BASIC_AUTH is not set."),
        }
    }
}