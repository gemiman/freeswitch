use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::error::Error;

/// Rust ESL 客户端示例
/// 演示如何连接 FreeSWITCH 并发送外呼指令
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 1. 连接 FreeSWITCH ESL 端口
    let mut stream = TcpStream::connect("127.0.0.1:8021").await?;
    println!("已连接到 FreeSWITCH");

    // 2. 读取欢迎消息 (Content-Type: auth/request)
    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).await?;
    println!("收到: {}", String::from_utf8_lossy(&buffer[..n]));

    // 3. 发送认证指令
    let auth_cmd = "auth ClueCon\n\n";
    stream.write_all(auth_cmd.as_bytes()).await?;

    // 4. 读取认证结果 (+OK accepted)
    let n = stream.read(&mut buffer).await?;
    let response = String::from_utf8_lossy(&buffer[..n]);
    if !response.contains("+OK accepted") {
        panic!("认证失败");
    }
    println!("认证成功");

    // 5. 发送点击拨号指令 (bgapi originate)
    // 场景: 坐席 1001 呼叫香港号码 0085298765432
    let agent_id = "1001";
    let customer_number = "0085298765432";
    
    let originate_cmd = format!(
        "bgapi originate {{origination_caller_id_number={}}}user/{} &bridge(sofia/gateway/aliyun-intl/{})\n\n",
        customer_number, agent_id, customer_number
    );

    println!("发送指令: {}", originate_cmd);
    stream.write_all(originate_cmd.as_bytes()).await?;

    // 6. 读取指令响应 (Job-UUID)
    let n = stream.read(&mut buffer).await?;
    println!("指令响应: {}", String::from_utf8_lossy(&buffer[..n]));

    Ok(())
}
