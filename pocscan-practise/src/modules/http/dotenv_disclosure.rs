use crate::{
    modules::{HttpFinding,HttpModule,Module},
    Error
};
use async_trait::async_trait;
use reqwest::Client;
//创建一个GitlabOpenRegistrations
pub struct DotEnvDisclosure{}
impl DotEnvDisclosure{
    pub fn new() -> Self{
        DotEnvDisclosure{}
    }
}

//GitlabOpenRegistrations 的名字及介绍
impl Module for DotEnvDisclosure{
    fn name(&self) -> String {
        String::from("http/dotenv")
    }
    fn description(&self) -> String {
        String::from("Check if a .env file disclosure")
    }
}

//poc scan
#[async_trait]
impl HttpModule for DotEnvDisclosure{
    async fn scan(
        &self,
        http_client:&Client,
        endpoint:&str,   
    ) -> Result<Option<HttpFinding>,Error>{
        let url = format!("{}/.env",&endpoint);
        let res = http_client.get(&url).send().await?;  //获取response

        if res.status().is_success(){  //如果response 返回不是200 就返回Ok(None)
            return Ok(Some(HttpFinding::DotEnvDisclosure(url)));
        }


        Ok(None)
    }
}