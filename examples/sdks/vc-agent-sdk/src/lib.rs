use wasm_bindgen::prelude::*;
use web_sys::WebSocket;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct VoiceCommAPI;

#[wasm_bindgen]
impl VoiceCommAPI {
    #[wasm_bindgen(static_method_of = VoiceCommAPI, js_name = setMode)]
    pub fn set_mode(mode: &str) {
        log(&format!("Set mode to: {}", mode));
    }

    #[wasm_bindgen(static_method_of = VoiceCommAPI, js_name = setConfigureServer)]
    pub fn set_configure_server(config: &JsValue) {
        log(&format!("Set config: {:?}", config));
    }

    #[wasm_bindgen(static_method_of = VoiceCommAPI, js_name = createAgentControl)]
    pub fn create_agent_control() -> AgentControl {
        AgentControl::new()
    }

    #[wasm_bindgen(static_method_of = VoiceCommAPI, js_name = createCallControl)]
    pub fn create_call_control() -> CallControl {
        CallControl::new()
    }
}

#[wasm_bindgen]
pub struct AgentControl {
    ws: Option<WebSocket>,
    login_success_handler: Option<js_sys::Function>,
    login_failed_handler: Option<js_sys::Function>,
    logout_handler: Option<js_sys::Function>,
    status_change_handler: Option<js_sys::Function>,
    members_count_handler: Option<js_sys::Function>,
    monitor_action_handler: Option<js_sys::Function>,
    error_handler: Option<js_sys::Function>,
    queue_list_handler: Option<js_sys::Function>,
    queue_agent_map_handler: Option<js_sys::Function>,
    tier_on_result_handler: Option<js_sys::Function>,
    tier_off_result_handler: Option<js_sys::Function>,
}

#[wasm_bindgen]
impl AgentControl {
    #[wasm_bindgen(constructor)]
    pub fn new() -> AgentControl {
        AgentControl {
            ws: None,
            login_success_handler: None,
            login_failed_handler: None,
            logout_handler: None,
            status_change_handler: None,
            members_count_handler: None,
            monitor_action_handler: None,
            error_handler: None,
            queue_list_handler: None,
            queue_agent_map_handler: None,
            tier_on_result_handler: None,
            tier_off_result_handler: None,
        }
    }

    #[wasm_bindgen(js_name = login)]
    pub fn login(
        &mut self,
        agent_id: &str,
        device: &str,
        password: &str,
        belong_queues: &JsValue,
        peer_audio: Option<web_sys::HtmlAudioElement>,
        local_audio: Option<web_sys::HtmlAudioElement>,
    ) -> bool {
        log(&format!("Login: agent_id={}, device={}", agent_id, device));
        true
    }

    #[wasm_bindgen(js_name = logout)]
    pub fn logout(&mut self) -> bool {
        log("Logout called");
        true
    }

    #[wasm_bindgen(js_name = setStatus)]
    pub fn set_status(&mut self, status: &str) -> bool {
        log(&format!("Set status to: {}", status));
        true
    }

    #[wasm_bindgen(js_name = forceBusy)]
    pub fn force_busy(&mut self, agent_device: &str) -> bool {
        log(&format!("Force busy: {}", agent_device));
        true
    }

    #[wasm_bindgen(js_name = forceFree)]
    pub fn force_free(&mut self, agent_device: &str) -> bool {
        log(&format!("Force free: {}", agent_device));
        true
    }

    #[wasm_bindgen(js_name = forceLogout)]
    pub fn force_logout(&mut self, agent_device: &str) -> bool {
        log(&format!("Force logout: {}", agent_device));
        true
    }

    #[wasm_bindgen(js_name = forceListen)]
    pub fn force_listen(&mut self, agent_device: &str) -> bool {
        log(&format!("Force listen: {}", agent_device));
        true
    }

    #[wasm_bindgen(js_name = forceConference)]
    pub fn force_conference(&mut self, agent_device: &str) -> bool {
        log(&format!("Force conference: {}", agent_device));
        true
    }

    #[wasm_bindgen(js_name = forceWhisper)]
    pub fn force_whisper(&mut self, agent_device: &str) -> bool {
        log(&format!("Force whisper: {}", agent_device));
        true
    }

    #[wasm_bindgen(js_name = forceHangup)]
    pub fn force_hangup(&mut self, agent_device: &str) -> bool {
        log(&format!("Force hangup: {}", agent_device));
        true
    }

    #[wasm_bindgen(js_name = forceIntercept)]
    pub fn force_intercept(&mut self, agent_device: &str) -> bool {
        log(&format!("Force intercept: {}", agent_device));
        true
    }

    #[wasm_bindgen(js_name = tierOn)]
    pub fn tier_on(&mut self, peer_agent_id: &str, queue: &str) -> bool {
        log(&format!("Tier on: agent={}, queue={}", peer_agent_id, queue));
        true
    }

    #[wasm_bindgen(js_name = tierOff)]
    pub fn tier_off(&mut self, peer_agent_id: &str, queue: &str) -> bool {
        log(&format!("Tier off: agent={}, queue={}", peer_agent_id, queue));
        true
    }

    #[wasm_bindgen(js_name = queueDetail)]
    pub fn queue_detail(&mut self, peer_agent_id: &str, queue: &str) -> bool {
        log(&format!("Queue detail: agent={}, queue={}", peer_agent_id, queue));
        true
    }

    #[wasm_bindgen(js_name = setAgentLoginSuccessEventListener)]
    pub fn set_agent_login_success_event_listener(&mut self, handler: js_sys::Function) {
        self.login_success_handler = Some(handler);
    }

    #[wasm_bindgen(js_name = setAgentLoginFailedEventListener)]
    pub fn set_agent_login_failed_event_listener(&mut self, handler: js_sys::Function) {
        self.login_failed_handler = Some(handler);
    }

    #[wasm_bindgen(js_name = setAgentLogoutEventListener)]
    pub fn set_agent_logout_event_listener(&mut self, handler: js_sys::Function) {
        self.logout_handler = Some(handler);
    }

    #[wasm_bindgen(js_name = setAgentStatusChangeEventListener)]
    pub fn set_agent_status_change_event_listener(&mut self, handler: js_sys::Function) {
        self.status_change_handler = Some(handler);
    }

    #[wasm_bindgen(js_name = setMembersCountEventListener)]
    pub fn set_members_count_event_listener(&mut self, handler: js_sys::Function) {
        self.members_count_handler = Some(handler);
    }

    #[wasm_bindgen(js_name = setMonitorForceActionAnsweredEventListener)]
    pub fn set_monitor_force_action_answered_event_listener(&mut self, handler: js_sys::Function) {
        self.monitor_action_handler = Some(handler);
    }

    #[wasm_bindgen(js_name = setAgentErrorEventListener)]
    pub fn set_agent_error_event_listener(&mut self, handler: js_sys::Function) {
        self.error_handler = Some(handler);
    }

    #[wasm_bindgen(js_name = setAllQueueListEventListener)]
    pub fn set_all_queue_list_event_listener(&mut self, handler: js_sys::Function) {
        self.queue_list_handler = Some(handler);
    }

    #[wasm_bindgen(js_name = setAllQueueAgentMapEventListener)]
    pub fn set_all_queue_agent_map_event_listener(&mut self, handler: js_sys::Function) {
        self.queue_agent_map_handler = Some(handler);
    }

    #[wasm_bindgen(js_name = setTierOnResultEventListener)]
    pub fn set_tier_on_result_event_listener(&mut self, handler: js_sys::Function) {
        self.tier_on_result_handler = Some(handler);
    }

    #[wasm_bindgen(js_name = setTierOffResultEventListener)]
    pub fn set_tier_off_result_event_listener(&mut self, handler: js_sys::Function) {
        self.tier_off_result_handler = Some(handler);
    }
}

#[wasm_bindgen]
pub struct CallControl {
    ws: Option<WebSocket>,
    ringing_handler: Option<js_sys::Function>,
    ring_back_handler: Option<js_sys::Function>,
    this_party_answered_handler: Option<js_sys::Function>,
    other_party_answered_handler: Option<js_sys::Function>,
    destroy_handler: Option<js_sys::Function>,
    hold_handler: Option<js_sys::Function>,
    unhold_handler: Option<js_sys::Function>,
    consult_handler: Option<js_sys::Function>,
    other_party_canceled_handler: Option<js_sys::Function>,
    dtmf_handler: Option<js_sys::Function>,
    agent_on_call_handler: Option<js_sys::Function>,
    mute_handler: Option<js_sys::Function>,
    unmute_handler: Option<js_sys::Function>,
    video_stream_error_handler: Option<js_sys::Function>,
    detect_speech_handler: Option<js_sys::Function>,
    publish_stream_prepare_handler: Option<js_sys::Function>,
    start_video_push_handler: Option<js_sys::Function>,
    stop_video_push_handler: Option<js_sys::Function>,
    voice_change_handler: Option<js_sys::Function>,
}

#[wasm_bindgen]
impl CallControl {
    #[wasm_bindgen(constructor)]
    pub fn new() -> CallControl {
        CallControl {
            ws: None,
            ringing_handler: None,
            ring_back_handler: None,
            this_party_answered_handler: None,
            other_party_answered_handler: None,
            destroy_handler: None,
            hold_handler: None,
            unhold_handler: None,
            consult_handler: None,
            other_party_canceled_handler: None,
            dtmf_handler: None,
            agent_on_call_handler: None,
            mute_handler: None,
            unmute_handler: None,
            video_stream_error_handler: None,
            detect_speech_handler: None,
            publish_stream_prepare_handler: None,
            start_video_push_handler: None,
            stop_video_push_handler: None,
            voice_change_handler: None,
        }
    }

    #[wasm_bindgen(js_name = makeCall)]
    pub fn make_call(
        &mut self,
        target: &str,
        trunk_number: Option<String>,
        gateway: Option<String>,
        outno: Option<String>,
    ) -> bool {
        log(&format!("Make call to: {}", target));
        true
    }

    #[wasm_bindgen(js_name = hangup)]
    pub fn hangup(&mut self) -> bool {
        log("Hangup called");
        true
    }

    #[wasm_bindgen(js_name = hold)]
    pub fn hold(&mut self) -> bool {
        log("Hold called");
        true
    }

    #[wasm_bindgen(js_name = unHold)]
    pub fn un_hold(&mut self) -> bool {
        log("Unhold called");
        true
    }

    #[wasm_bindgen(js_name = consult)]
    pub fn consult(
        &mut self,
        target: &str,
        call_data: Option<String>,
        trunk_number: Option<String>,
        gateway: Option<String>,
        outno: Option<String>,
    ) -> bool {
        log(&format!("Consult to: {}", target));
        true
    }

    #[wasm_bindgen(js_name = retrieve)]
    pub fn retrieve(&mut self) -> bool {
        log("Retrieve called");
        true
    }

    #[wasm_bindgen(js_name = completeTransfer)]
    pub fn complete_transfer(&mut self) -> bool {
        log("Complete transfer called");
        true
    }

    #[wasm_bindgen(js_name = completeThreeParties)]
    pub fn complete_three_parties(&mut self) -> bool {
        log("Complete three parties called");
        true
    }

    #[wasm_bindgen(js_name = singleStepTransfer)]
    pub fn single_step_transfer(&mut self, target: &str, call_data: Option<String>) -> bool {
        log(&format!("Single step transfer to: {}", target));
        true
    }

    #[wasm_bindgen(js_name = singleStepTransferPhone)]
    pub fn single_step_transfer_phone(
        &mut self,
        target: &str,
        call_data: Option<String>,
        trunk_number: Option<String>,
        gateway: Option<String>,
        outno: Option<String>,
    ) -> bool {
        log(&format!("Single step transfer to phone: {}", target));
        true
    }

    #[wasm_bindgen(js_name = transferAgentGroup)]
    pub fn transfer_agent_group(&mut self, target: &str, call_data: Option<String>) -> bool {
        log(&format!("Transfer to agent group: {}", target));
        true
    }

    #[wasm_bindgen(js_name = agentEvaluate)]
    pub fn agent_evaluate(&mut self, call_data: Option<String>) -> bool {
        log("Agent evaluate called");
        true
    }

    #[wasm_bindgen(js_name = mute)]
    pub fn mute(&mut self) -> bool {
        log("Mute called");
        true
    }

    #[wasm_bindgen(js_name = unMute)]
    pub fn un_mute(&mut self) -> bool {
        log("Unmute called");
        true
    }

    #[wasm_bindgen(js_name = proxyAnswer)]
    pub fn proxy_answer(&mut self, target: &str) -> bool {
        log(&format!("Proxy answer to: {}", target));
        true
    }

    #[wasm_bindgen(js_name = sendDTMF)]
    pub fn send_dtmf(&mut self, target: &str) -> bool {
        log(&format!("Send DTMF: {}", target));
        true
    }

    #[wasm_bindgen(js_name = showCustomerViewer)]
    pub fn show_customer_viewer(&mut self, element_id: &str, href: &str) -> bool {
        log(&format!("Show customer viewer: {} - {}", element_id, href));
        true
    }

    #[wasm_bindgen(js_name = closeCustomerViewer)]
    pub fn close_customer_viewer(&mut self, element_id: &str) -> bool {
        log(&format!("Close customer viewer: {}", element_id));
        true
    }

    #[wasm_bindgen(js_name = publishLocalStream)]
    pub fn publish_local_stream(&mut self, element_id: &str) -> bool {
        log(&format!("Publish local stream: {}", element_id));
        true
    }

    #[wasm_bindgen(js_name = startPushVideoStream)]
    pub fn start_push_video_stream(&mut self, stream_path: &str) -> bool {
        log(&format!("Start push video stream: {}", stream_path));
        true
    }

    #[wasm_bindgen(js_name = closeLocalStream)]
    pub fn close_local_stream(&mut self, element_id: &str) -> bool {
        log(&format!("Close local stream: {}", element_id));
        true
    }

    #[wasm_bindgen(js_name = requestVideoMode)]
    pub fn request_video_mode(&mut self) -> bool {
        log("Request video mode");
        true
    }

    #[wasm_bindgen(js_name = requestAudioMode)]
    pub fn request_audio_mode(&mut self) -> bool {
        log("Request audio mode");
        true
    }

    #[wasm_bindgen(js_name = startPushMediaFile)]
    pub fn start_push_media_file(&mut self, local_path: &str) -> bool {
        log(&format!("Start push media file: {}", local_path));
        true
    }

    #[wasm_bindgen(js_name = stopPushMedia)]
    pub fn stop_push_media(&mut self) -> bool {
        log("Stop push media");
        true
    }

    #[wasm_bindgen(js_name = voiceChangeThick)]
    pub fn voice_change_thick(
        &mut self,
        frequency_key: &str,
        dest_type: &str,
        value: Option<String>,
        is_change_record: Option<String>,
    ) -> bool {
        log(&format!("Voice change thick: {}", frequency_key));
        true
    }

    #[wasm_bindgen(js_name = voiceChangeThin)]
    pub fn voice_change_thin(
        &mut self,
        frequency_key: &str,
        dest_type: &str,
        value: Option<String>,
        is_change_record: Option<String>,
    ) -> bool {
        log(&format!("Voice change thin: {}", frequency_key));
        true
    }

    #[wasm_bindgen(js_name = answer)]
    pub fn answer(&mut self) -> bool {
        log("Answer called");
        true
    }

    #[wasm_bindgen(js_name = checkAnswerTool)]
    pub fn check_answer_tool(&mut self) -> bool {
        log("Check answer tool");
        true
    }

    #[wasm_bindgen(js_name = singleStepTransferErgtoc)]
    pub fn single_step_transfer_ergtoc(
        &mut self,
        target: &str,
        call_data: Option<String>,
        trunk_number: Option<String>,
        gateway: Option<String>,
        outno: Option<String>,
    ) -> bool {
        log(&format!("Single step transfer ergtoc: {}", target));
        true
    }

    #[wasm_bindgen(js_name = makeCallRegGWExtNo)]
    pub fn make_call_reg_gw_ext_no(
        &mut self,
        target: &str,
        trunk_number: Option<String>,
        gateway: Option<String>,
        outno: Option<String>,
    ) -> bool {
        log(&format!("Make call reg GW ext no: {}", target));
        true
    }

    #[wasm_bindgen(js_name = setChannelRingingEventListener)]
    pub fn set_channel_ringing_event_listener(&mut self, handler: js_sys::Function) {
        self.ringing_handler = Some(handler);
    }

    #[wasm_bindgen(js_name = setChannelRingBackEventListener)]
    pub fn set_channel_ring_back_event_listener(&mut self, handler: js_sys::Function) {
        self.ring_back_handler = Some(handler);
    }

    #[wasm_bindgen(js_name = setChannelThisPartyAnsweredEventListener)]
    pub fn set_channel_this_party_answered_event_listener(&mut self, handler: js_sys::Function) {
        self.this_party_answered_handler = Some(handler);
    }

    #[wasm_bindgen(js_name = setChannelOtherPartyAnsweredEventListener)]
    pub fn set_channel_other_party_answered_event_listener(&mut self, handler: js_sys::Function) {
        self.other_party_answered_handler = Some(handler);
    }

    #[wasm_bindgen(js_name = setChannelDestroyEventListener)]
    pub fn set_channel_destroy_event_listener(&mut self, handler: js_sys::Function) {
        self.destroy_handler = Some(handler);
    }

    #[wasm_bindgen(js_name = setChannelHoldEventListener)]
    pub fn set_channel_hold_event_listener(&mut self, handler: js_sys::Function) {
        self.hold_handler = Some(handler);
    }

    #[wasm_bindgen(js_name = setChannelUnHoldEventListener)]
    pub fn set_channel_un_hold_event_listener(&mut self, handler: js_sys::Function) {
        self.unhold_handler = Some(handler);
    }

    #[wasm_bindgen(js_name = setConsultEventListener)]
    pub fn set_consult_event_listener(&mut self, handler: js_sys::Function) {
        self.consult_handler = Some(handler);
    }

    #[wasm_bindgen(js_name = setOtherPartyCanceledEventListener)]
    pub fn set_other_party_canceled_event_listener(&mut self, handler: js_sys::Function) {
        self.other_party_canceled_handler = Some(handler);
    }

    #[wasm_bindgen(js_name = setRecvDtmfEventListener)]
    pub fn set_recv_dtmf_event_listener(&mut self, handler: js_sys::Function) {
        self.dtmf_handler = Some(handler);
    }

    #[wasm_bindgen(js_name = setAgentOnCallEventListener)]
    pub fn set_agent_on_call_event_listener(&mut self, handler: js_sys::Function) {
        self.agent_on_call_handler = Some(handler);
    }

    #[wasm_bindgen(js_name = setChannelMuteEventListener)]
    pub fn set_channel_mute_event_listener(&mut self, handler: js_sys::Function) {
        self.mute_handler = Some(handler);
    }

    #[wasm_bindgen(js_name = setChannelUnMuteEventListener)]
    pub fn set_channel_un_mute_event_listener(&mut self, handler: js_sys::Function) {
        self.unmute_handler = Some(handler);
    }

    #[wasm_bindgen(js_name = setVideoStreamErrorListener)]
    pub fn set_video_stream_error_listener(&mut self, handler: js_sys::Function) {
        self.video_stream_error_handler = Some(handler);
    }

    #[wasm_bindgen(js_name = setDetectSpeechEventListener)]
    pub fn set_detect_speech_event_listener(&mut self, handler: js_sys::Function) {
        self.detect_speech_handler = Some(handler);
    }

    #[wasm_bindgen(js_name = setPublishStreamPrepareEventListener)]
    pub fn set_publish_stream_prepare_event_listener(&mut self, handler: js_sys::Function) {
        self.publish_stream_prepare_handler = Some(handler);
    }

    #[wasm_bindgen(js_name = setStartVideoPushEventListener)]
    pub fn set_start_video_push_event_listener(&mut self, handler: js_sys::Function) {
        self.start_video_push_handler = Some(handler);
    }

    #[wasm_bindgen(js_name = setStopVideoPushEventListener)]
    pub fn set_stop_video_push_event_listener(&mut self, handler: js_sys::Function) {
        self.stop_video_push_handler = Some(handler);
    }

    #[wasm_bindgen(js_name = setVoiceChangeEventListener)]
    pub fn set_voice_change_event_listener(&mut self, handler: js_sys::Function) {
        self.voice_change_handler = Some(handler);
    }
}
