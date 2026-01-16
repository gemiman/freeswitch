import * as VoiceCommAPI from 'vc-agent-sdk';

console.log('测试座席SDK初始化');

try {
    // 设置模式
    VoiceCommAPI.VoiceCommAPI.set_mode('development');

    // 配置服务器
    VoiceCommAPI.VoiceCommAPI.set_configure_server({
        websocketHost: '127.0.0.1',
        websocketPort: 9058,
        websocketSSL: false,
        webSocketUrl: 'ws://127.0.0.1:9058',
        domain: '172.16.20.181',
        trunkNumber: '+862160568356',
        gateway: 'dfdea3e4-ba6c-4398-8b9d-781cc71dbb98',
        outno: '98',
        isAfterCall: false,
        traceMsgDebug: true,
        isDetectSpeech: true
    });

    // 创建控制实例
    const agent = VoiceCommAPI.VoiceCommAPI.create_agent_control();
    const call = VoiceCommAPI.VoiceCommAPI.create_call_control();

    console.log('座席SDK初始化成功');

    // 测试设置事件监听器
    agent.set_agent_login_success_event_listener((event) => {
        console.log('登录成功:', event);
    });

    agent.set_agent_login_failed_event_listener((event) => {
        console.log('登录失败:', event);
    });

    agent.set_agent_logout_event_listener((event) => {
        console.log('登出:', event);
    });

    call.set_channel_ringing_event_listener((event) => {
        console.log('呼入振铃:', event);
    });

    call.set_channel_ring_back_event_listener((event) => {
        console.log('呼出振铃:', event);
    });

    console.log('事件监听器设置成功');
} catch (error) {
    console.error('座席SDK测试失败:', error);
}
