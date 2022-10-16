use crate::{
    modules::{HttpFinding,HttpModule,Module},
    Error
};
use async_trait::async_trait;
use reqwest::Client;
//创建一个GitlabOpenRegistrations
pub struct GitHeadDisclosure{}
impl GitHeadDisclosure{
    pub fn new() -> Self{
        GitHeadDisclosure{}
    }
    fn is_head_file(&self,content:&str) -> bool{
        return Some(0) == content.to_lowercase().trim().find("ref:"); //ref在content中第一次出现的位置 HEAD文件 ref: refs/heads/master
    }
}

//GitlabOpenRegistrations 的名字及介绍
impl Module for GitHeadDisclosure{
    fn name(&self) -> String {
        String::from("http/git_head_disclosure")
    }
    fn description(&self) -> String {
        String::from("Check for .git/HEAD file disclosure")
    }
}

//poc scan
#[async_trait]
impl HttpModule for GitHeadDisclosure{
    async fn scan(
        &self,
        http_client:&Client,
        endpoint:&str,   
    ) -> Result<Option<HttpFinding>,Error>{
        let url = format!("{}/.git/HEAD",&endpoint);
        let res = http_client.get(&url).send().await?;  //获取response

        if !res.status().is_success(){  //如果response 返回不是200 就返回Ok(None)
            return Ok(None);
        }
        let body = res.text().await?;  //获取body 的内容
        if self.is_head_file(&body){ //body中存在This is a self-managed instance of GitLab和Register表示漏洞存在
            return Ok(Some(HttpFinding::GitHeadDisclosure(url)));
        }


        Ok(None)
    }
}