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
pub struct VcMonitorApi {
    ws: Option<WebSocket>,
    agent_status_change_handler: Option<js_sys::Function>,
    agent_list_handler: Option<js_sys::Function>,
    members_count_handler: Option<js_sys::Function>,
    agent_on_call_handler: Option<js_sys::Function>,
    gateway_warning_handler: Option<js_sys::Function>,
    logout_handler: Option<js_sys::Function>,
    detected_speech_handler: Option<js_sys::Function>,
}

#[wasm_bindgen]
impl VcMonitorApi {
    #[wasm_bindgen(constructor)]
    pub fn new() -> VcMonitorApi {
        VcMonitorApi {
            ws: None,
            agent_status_change_handler: None,
            agent_list_handler: None,
            members_count_handler: None,
            agent_on_call_handler: None,
            gateway_warning_handler: None,
            logout_handler: None,
            detected_speech_handler: None,
        }
    }

    #[wasm_bindgen(static_method_of = VcMonitorApi)]
    pub fn create_agent_control(host: &str, port: u32, ssl: &str) -> VcMonitorApi {
        let mut api = VcMonitorApi::new();
        let protocol = if ssl == "wss" { "wss" } else { "ws" };
        let url = format!("{}://{}:{}", protocol, host, port);

        match WebSocket::new(&url) {
            Ok(ws) => {
                api.ws = Some(ws);
                log("WebSocket connection created successfully");
            }
            Err(e) => {
                log(&format!("WebSocket connection failed: {:?}", e));
            }
        }

        api
    }

    #[wasm_bindgen]
    pub fn admin_login(&mut self, device: &str, queue: &str) -> bool {
        match &self.ws {
            Some(ws) => {
                let msg = json!({
                    "cmd": "admin_login",
                    "device": device,
                    "queue": queue
                });
                if let Err(e) = ws.send_with_str(&serde_json::to_string(&msg).unwrap()) {
                    log(&format!("Send message failed: {:?}", e));
                    return false;
                }
                true
            }
            None => {
                log("WebSocket not connected");
                false
            }
        }
    }

    #[wasm_bindgen]
    pub fn logout(&mut self) -> bool {
        match &self.ws {
            Some(ws) => {
                let msg = json!({ "cmd": "logout" });
                if let Err(e) = ws.send_with_str(&serde_json::to_string(&msg).unwrap()) {
                    log(&format!("Send message failed: {:?}", e));
                    return false;
                }
                true
            }
            None => {
                log("WebSocket not connected");
                false
            }
        }
    }

    #[wasm_bindgen]
    pub fn force_free(&mut self, agent_device: &str) -> bool {
        match &self.ws {
            Some(ws) => {
                let msg = json!({
                    "cmd": "force_free",
                    "AgentDevice": agent_device
                });
                if let Err(e) = ws.send_with_str(&serde_json::to_string(&msg).unwrap()) {
                    log(&format!("Send message failed: {:?}", e));
                    return false;
                }
                true
            }
            None => {
                log("WebSocket not connected");
                false
            }
        }
    }

    #[wasm_bindgen]
    pub fn force_busy(&mut self, agent_device: &str) -> bool {
        match &self.ws {
            Some(ws) => {
                let msg = json!({
                    "cmd": "force_busy",
                    "AgentDevice": agent_device
                });
                if let Err(e) = ws.send_with_str(&serde_json::to_string(&msg).unwrap()) {
                    log(&format!("Send message failed: {:?}", e));
                    return false;
                }
                true
            }
            None => {
                log("WebSocket not connected");
                false
            }
        }
    }

    #[wasm_bindgen]
    pub fn force_logout(&mut self, agent_device: &str) -> bool {
        match &self.ws {
            Some(ws) => {
                let msg = json!({
                    "cmd": "force_logout",
                    "AgentDevice": agent_device
                });
                if let Err(e) = ws.send_with_str(&serde_json::to_string(&msg).unwrap()) {
                    log(&format!("Send message failed: {:?}", e));
                    return false;
                }
                true
            }
            None => {
                log("WebSocket not connected");
                false
            }
        }
    }

    #[wasm_bindgen]
    pub fn force_listen(&mut self, agent_device: &str) -> bool {
        match &self.ws {
            Some(ws) => {
                let msg = json!({
                    "cmd": "force_listen",
                    "AgentDevice": agent_device
                });
                if let Err(e) = ws.send_with_str(&serde_json::to_string(&msg).unwrap()) {
                    log(&format!("Send message failed: {:?}", e));
                    return false;
                }
                true
            }
            None => {
                log("WebSocket not connected");
                false
            }
        }
    }

    #[wasm_bindgen]
    pub fn force_conference(&mut self, agent_device: &str) -> bool {
        match &self.ws {
            Some(ws) => {
                let msg = json!({
                    "cmd": "force_conference",
                    "AgentDevice": agent_device
                });
                if let Err(e) = ws.send_with_str(&serde_json::to_string(&msg).unwrap()) {
                    log(&format!("Send message failed: {:?}", e));
                    return false;
                }
                true
            }
            None => {
                log("WebSocket not connected");
                false
            }
        }
    }

    #[wasm_bindgen]
    pub fn force_whisper(&mut self, agent_device: &str) -> bool {
        match &self.ws {
            Some(ws) => {
                let msg = json!({
                    "cmd": "force_whisper",
                    "AgentDevice": agent_device
                });
                if let Err(e) = ws.send_with_str(&serde_json::to_string(&msg).unwrap()) {
                    log(&format!("Send message failed: {:?}", e));
                    return false;
                }
                true
            }
            None => {
                log("WebSocket not connected");
                false
            }
        }
    }

    #[wasm_bindgen]
    pub fn force_hangup(&mut self, agent_device: &str) -> bool {
        match &self.ws {
            Some(ws) => {
                let msg = json!({
                    "cmd": "force_hangup",
                    "AgentDevice": agent_device
                });
                if let Err(e) = ws.send_with_str(&serde_json::to_string(&msg).unwrap()) {
                    log(&format!("Send message failed: {:?}", e));
                    return false;
                }
                true
            }
            None => {
                log("WebSocket not connected");
                false
            }
        }
    }

    #[wasm_bindgen]
    pub fn force_intercept(&mut self, agent_device: &str) -> bool {
        match &self.ws {
            Some(ws) => {
                let msg = json!({
                    "cmd": "force_intercept",
                    "AgentDevice": agent_device
                });
                if let Err(e) = ws.send_with_str(&serde_json::to_string(&msg).unwrap()) {
                    log(&format!("Send message failed: {:?}", e));
                    return false;
                }
                true
            }
            None => {
                log("WebSocket not connected");
                false
            }
        }
    }

    #[wasm_bindgen]
    pub fn set_agent_status_change_event_listener(&mut self, handler: js_sys::Function) {
        self.agent_status_change_handler = Some(handler);
    }

    #[wasm_bindgen]
    pub fn set_agent_list_event_listener(&mut self, handler: js_sys::Function) {
        self.agent_list_handler = Some(handler);
    }

    #[wasm_bindgen]
    pub fn set_members_count_event(&mut self, handler: js_sys::Function) {
        self.members_count_handler = Some(handler);
    }

    #[wasm_bindgen]
    pub fn set_agent_on_call_event_listener(&mut self, handler: js_sys::Function) {
        self.agent_on_call_handler = Some(handler);
    }

    #[wasm_bindgen]
    pub fn set_gateway_warning_event_listener(&mut self, handler: js_sys::Function) {
        self.gateway_warning_handler = Some(handler);
    }

    #[wasm_bindgen]
    pub fn set_logout_event_listener(&mut self, handler: js_sys::Function) {
        self.logout_handler = Some(handler);
    }

    #[wasm_bindgen]
    pub fn set_detected_speech_event_listener(&mut self, handler: js_sys::Function) {
        self.detected_speech_handler = Some(handler);
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "EvtName")]
enum Event {
    AgentStatusChangeEvent {
        Data: AgentStatusData,
    },
    AgentListEvent {
        List: Vec<AgentStatusData>,
    },
    MembersCountEvent {
        Data: MembersCountData,
    },
    AgentOnCallEvent {
        Called: String,
        State: String,
    },
    GatewayWarningEvent {
        Gateway: String,
        Profile: String,
        Status: String,
    },
    LogoutEvent {
        Code: String,
        Cause: String,
    },
    DetectedSpeechEvent {
        Data: DetectedSpeechData,
    },
}

#[derive(Debug, Deserialize)]
struct AgentStatusData {
    AgentId: String,
    Device: String,
    Status: String,
    Queue: String,
    DateTime: String,
}

#[derive(Debug, Deserialize)]
struct MembersCountData {
    Queue: String,
    Count: String,
    DateTime: String,
}

#[derive(Debug, Deserialize)]
struct DetectedSpeechData {
    ConnId: String,
    Speaker: String,
    SpeakerUUID: String,
    Content: String,
    DateTime: String,
}
