use crate::{
    modules::{HttpFinding,HttpModule,Module},
    Error
};
use async_trait::async_trait;
use reqwest::Client;
use regex::Regex; //正则表达式
//创建一个GitlabOpenRegistrations
pub struct DirectoryListingDisclosure{
    dir_listing_regex:Regex, 
}
impl DirectoryListingDisclosure{
    pub fn new() -> Self{
        DirectoryListingDisclosure{
            dir_listing_regex: Regex::new(r"<title>Index of .*</title>").expect("compling http/directory_listing regexp"),
        }
    }
    async fn is_director_listing(&self,body:String) -> Result<bool,Error>{
        let dir_listing_regex = self.dir_listing_regex.clone();
        let res = tokio::task::spawn_blocking(move ||{
            dir_listing_regex.is_match(&body) //判断正则表达式是否在body里面
        }).await?; //错误类型指向的是tokio::task::JoinError
        Ok(res)
    }

}

//GitlabOpenRegistrations 的名字及介绍
impl Module for DirectoryListingDisclosure{
    fn name(&self) -> String {
        String::from("http/directory_listing")
    }
    fn description(&self) -> String {
        String::from("Check for enabled directory listing, which often leak information")
    }
}

//poc scan
#[async_trait]
impl HttpModule for DirectoryListingDisclosure{
    async fn scan(
        &self,
        http_client:&Client,
        endpoint:&str,   
    ) -> Result<Option<HttpFinding>,Error>{
        let url = format!("{}/",&endpoint);
        let res = http_client.get(&url).send().await?;  //获取response

        if !res.status().is_success(){  //如果response 返回不是200 就返回Ok(None)
            return Ok(None);
        }
        let body = res.text().await?;  //获取body 的内容
        if self.is_director_listing(body).await?{ //body中存在This is a self-managed instance of GitLab和Register表示漏洞存在
            return Ok(Some(HttpFinding::DirectoryListingDisclosure(url)));
        }


        Ok(None)
    }
}