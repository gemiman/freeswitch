const fs = require('fs');
const path = require('path');

// 读取d.ts文件
const monitorDtsContent = fs.readFileSync('vc-monitor-sdk/pkg/vc_monitor_sdk.d.ts', 'utf8');
const agentDtsContent = fs.readFileSync('vc-agent-sdk/pkg/vc_agent_sdk.d.ts', 'utf8');

console.log('=== 监控SDK事件监听器 ===');
const monitorListenerMatches = monitorDtsContent.match(/set_\w+_event_listener\(/g);
if (monitorListenerMatches) {
    const uniqueListeners = [...new Set(monitorListenerMatches.map(m => m.replace('(', '').trim()))];
    uniqueListeners.sort();
    uniqueListeners.forEach(listener => {
        console.log('-', listener);
    });
}

console.log('\n=== 座席SDK事件监听器 ===');
console.log('AgentControl:');
const agentControlListenerMatches = agentDtsContent.match(/set_agent_\w+_event_listener\(/g);
if (agentControlListenerMatches) {
    const uniqueListeners = [...new Set(agentControlListenerMatches.map(m => m.replace('(', '').trim()))];
    uniqueListeners.sort();
    uniqueListeners.forEach(listener => {
        console.log('-', listener);
    });
}

console.log('\nCallControl:');
const callControlListenerMatches = agentDtsContent.match(/set_channel_\w+_event_listener\(|set_\w+_event_listener\(/g);
if (callControlListenerMatches) {
    const uniqueListeners = [...new Set(callControlListenerMatches.map(m => m.replace('(', '').trim()))];
    const filteredListeners = uniqueListeners.filter(listener => !listener.includes('set_agent_'));
    filteredListeners.sort();
    filteredListeners.forEach(listener => {
        if (!['set_mode', 'set_configure_server', 'constructor', 'free', '[Symbol.dispose]', 'complete_three_parties', 'complete_transfer', 'single_step_transfer', 'single_step_transfer_ergtoc', 'single_step_transfer_phone', 'transfer_agent_group'].includes(listener)) {
            console.log('-', listener);
        }
    });
}
