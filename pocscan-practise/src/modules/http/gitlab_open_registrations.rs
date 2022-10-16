use crate::{
    modules::{HttpFinding,HttpModule,Module},
    Error
};
use async_trait::async_trait;
use reqwest::Client;
//创建一个GitlabOpenRegistrations
pub struct GitlabOpenRegistrations{}
impl GitlabOpenRegistrations{
    pub fn new() -> Self{
        GitlabOpenRegistrations{}
    }
}

//GitlabOpenRegistrations 的名字及介绍
impl Module for GitlabOpenRegistrations{
    fn name(&self) -> String {
        String::from("http/gitlab_open_registration")
    }
    fn description(&self) -> String {
        String::from("Check if the GitLab instance is open to registrations")
    }
}

//poc scan
#[async_trait]
impl HttpModule for GitlabOpenRegistrations{
    async fn scan(
        &self,
        http_client:&Client,
        endpoint:&str,   
    ) -> Result<Option<HttpFinding>,Error>{
        let url = format!("{}",&endpoint);
        let res = http_client.get(&url).send().await?;  //获取response

        if !res.status().is_success(){  //如果response 返回不是200 就返回Ok(None)
            return Ok(None);
        }
        let body = res.text().await?;  //获取body 的内容
        if body.contains("This is a self-managed instance of GitLab")&&body.contains("Register"){ //body中存在This is a self-managed instance of GitLab和Register表示漏洞存在
            return Ok(Some(HttpFinding::GitlabOpenRegistrations(url)));
        }


        Ok(None)
    }
}