import * as VcMonitorApi from 'vc-monitor-sdk';

console.log('测试监控SDK初始化');

try {
    const api = VcMonitorApi.VcMonitorApi.create_agent_control('127.0.0.1', 9059, 'ws');
    console.log('监控SDK初始化成功');

    // 测试设置事件监听器
    api.set_agent_status_change_event_listener((event) => {
        console.log('座席状态变化:', event);
    });

    api.set_agent_list_event_listener((event) => {
        console.log('座席列表:', event);
    });

    api.set_members_count_event((event) => {
        console.log('未接来电排队数量:', event);
    });

    console.log('事件监听器设置成功');
} catch (error) {
    console.error('监控SDK测试失败:', error);
}
