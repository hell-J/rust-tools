use crate::{
    modules::{HttpFinding,HttpModule,Module},
    Error
};
use async_trait::async_trait;
use reqwest::Client;
//创建一个GitlabOpenRegistrations
pub struct DsStoreDisclosure{}
impl DsStoreDisclosure{
    pub fn new() -> Self{
        DsStoreDisclosure{}
    }
    pub fn is_ds_store_file(&self, content: &[u8]) -> bool {
        if content.len() < 8 {
            return false;
        }

        let signature = [0x0, 0x0, 0x0, 0x1, 0x42, 0x75, 0x64, 0x31];  //.DS_Store文件头

        return content[0..8] == signature;
    }
}

//GitlabOpenRegistrations 的名字及介绍
impl Module for DsStoreDisclosure{
    fn name(&self) -> String {
        String::from("http/ds_store")
    }
    fn description(&self) -> String {
        String::from("Check if a .DS_Store file disclosure")
    }
}

//poc scan
#[async_trait]
impl HttpModule for DsStoreDisclosure{
    async fn scan(
        &self,
        http_client:&Client,
        endpoint:&str,   
    ) -> Result<Option<HttpFinding>,Error>{
        let url = format!("{}/.DS_Store",&endpoint);
        let res = http_client.get(&url).send().await?;  //获取response

        if !res.status().is_success(){  //如果response 返回不是200 就返回Ok(None)
            return Ok(None);
        }
        let body = res.bytes().await?;
        if self.is_ds_store_file(&body.as_ref()) {
            return Ok(Some(HttpFinding::DsStoreDisclosure(url)));
        }


        Ok(None)
    }
}