package com.example.cc.ivr;

import org.freeswitch.esl.client.inbound.Client;
import org.freeswitch.esl.client.transport.event.EslEvent;
import org.freeswitch.esl.client.transport.message.EslMessage;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

/**
 * 简单的自助 IVR 流程
 * 流程: 欢迎语 -> 按键检测 -> 转人工
 */
public class IvrController {
    
    private static final Logger logger = LoggerFactory.getLogger(IvrController.class);
    private final Client eslClient;

    public IvrController() {
        eslClient = new Client();
        try {
            // 连接到 Outbound Socket 模式的 FS
            // 注意: 这里假设是 Inbound 连接控制，实际 Outbound 模式需要 netty server 监听
            eslClient.connect("127.0.0.1", 8021, "ClueCon", 10);
            
            // 订阅 DTMF 按键事件
            eslClient.setEventSubscriptions("plain", "DTMF CHANNEL_EXECUTE_COMPLETE");
            
            // 注册事件监听器
            eslClient.addEventListener(event -> handleEvent(event));
            
        } catch (Exception e) {
            logger.error("连接失败", e);
        }
    }

    private void handleEvent(EslEvent event) {
        String eventName = event.getEventName();
        String uuid = event.getEventHeaders().get("Unique-ID");

        if ("DTMF".equals(eventName)) {
            String digit = event.getEventHeaders().get("DTMF-Digit");
            logger.info("收到按键: {} (UUID: {})", digit, uuid);
            
            if ("1".equals(digit)) {
                transferToAgent(uuid);
            } else if ("0".equals(digit)) {
                playGoodbye(uuid);
            }
        }
    }

    // 转接人工坐席
    private void transferToAgent(String uuid) {
        // execute(uuid, app, args)
        eslClient.sendAsyncApiCommand("uuid_break", uuid + " all"); // 打断当前的播放
        eslClient.sendAsyncApiCommand("uuid_transfer", uuid + " 1001 XML default");
        logger.info("已转接坐席 1001");
    }

    // 播放结束语并挂断
    private void playGoodbye(String uuid) {
        eslClient.sendAsyncApiCommand("uuid_broadcast", uuid + " playback::ivr/goodbye.wav");
        eslClient.sendAsyncApiCommand("uuid_kill", uuid);
    }
}
