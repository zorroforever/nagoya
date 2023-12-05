# nagoya system 名古屋システム
## RUST ウェブ練習
1. 数据库选择的是mysql
2. web框架是actix-web
3. orm用的是sea-orm

**2023-12-05**  
增加了注册接口，用来练习插入数据库的逻辑。  
这里在接受html提交上来的字段时候如果直接使用  
数据库model会造成缺少某些字段的错误，  
比如id，所有需要专门定义一个结构体来接受来自  
html的数据，然后把数据转给数据库的model就好了。

感谢：  
actix-files = "0.6.2"  
actix-http = "3.4.0"  
actix-rt = "2.9"  
actix-service = "2.0.2"  
actix-web = "4.4.0"  
tera = "1.19.0"  
dotenvy = "0.15"  
listenfd = "1.0.1"  
serde = "1.0.188"  
tracing-subscriber = { version = "0.3.17", features = ["env-filter"] }  
sea-orm  = "0.12.2"  
sea-orm-migration = { version = "0.12.2" ,features = ["runtime-actix-native-tls","sqlx-mysql"]}  
tokio = { version = "1.20.0", features = ["macros", "rt"] }  
