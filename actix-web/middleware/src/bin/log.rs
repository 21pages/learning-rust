use actix_web::middleware::Logger;
use env_logger::Env;

/*
%% 百分号
%a 远程 IP 地址（若使用了反向代理，则为代理地址）
%t 请求开始处理的时间
%P 请求的子服务进程 ID
%r 请求的第一行
%s 响应状态码
%b 包含 HTTP 消息标头的响应字节大小
%T 请求服务所用时间, 单位为秒，格式为浮点分数（.06f）
%D 请求服务所用时间, 单位为毫秒
%{FOO}i request.headers[‘FOO’]
%{FOO}o response.headers[‘FOO’]
%{FOO}e os.environ[‘FOO’]
*/

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
