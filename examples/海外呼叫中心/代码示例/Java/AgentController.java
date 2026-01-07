package com.example.callcenter.service;

import org.freeswitch.esl.client.inbound.Client;
import org.freeswitch.esl.client.inbound.InboundConnectionFailure;
import org.freeswitch.esl.client.transport.message.EslMessage;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.stereotype.Service;

/**
 * 坐席呼叫控制器 (基于 ESL)
 * 依赖库: org.freeswitch.esl.client:org.freeswitch.esl.client:0.9.2
 */
@Service
public class AgentController {

    private static final Logger logger = LoggerFactory.getLogger(AgentController.class);
    private final Client eslClient = new Client();

    public AgentController() {
        try {
            // 连接到 FreeSWITCH ESL 端口 (默认 8021)
            eslClient.connect("127.0.0.1", 8021, "ClueCon", 10);
        } catch (InboundConnectionFailure e) {
            logger.error("连接 FreeSWITCH 失败", e);
        }
    }

    /**
     * 发起点击拨号 (Click-to-Dial)
     * 逻辑: 先呼叫坐席(WebRTC)，坐席接通后，再呼叫客户(Aliyun)
     *
     * @param agentId 坐席分机号 (如 1001)
     * @param customerNumber 客户号码 (如 00852...)
     */
    public void clickToDial(String agentId, String customerNumber) {
        // 构造 originate 命令
        // 格式: originate {origination_caller_id_number=...}user/1001 &bridge(sofia/gateway/aliyun/...)
        
        String command = String.format(
            "originate {origination_caller_id_name='System',origination_caller_id_number='%s'}user/%s &bridge(sofia/gateway/aliyun-intl/%s)",
            customerNumber, agentId, customerNumber
        );

        logger.info("执行外呼: {}", command);

        EslMessage response = eslClient.sendSyncApiCommand(command, null);
        
        if (response.getBodyLines().stream().anyMatch(line -> line.contains("+OK"))) {
            logger.info("外呼指令发送成功，UUID: {}", response.getHeaderValue("Job-UUID"));
        } else {
            logger.error("外呼失败: {}", response.getBodyLines());
        }
    }
    
    /**
     * 监听坐席状态 (简易版)
     */
    public void subscribeEvents() {
        // 订阅通道状态和心跳
        eslClient.setEventSubscriptions("plain", "CHANNEL_CREATE CHANNEL_HANGUP HEARTBEAT");
        
        // 在实际应用中，需要通过 addEventListener 注册回调处理逻辑
    }
}
