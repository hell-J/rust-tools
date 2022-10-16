use crate::{
    modules::{HttpFinding,HttpModule,Module},
    Error
};
use async_trait::async_trait;
use reqwest::Client;
//创建一个EtcdUnauthenticatedAccess
pub struct EtcdUnauthenticatedAccess{}
impl EtcdUnauthenticatedAccess{
    pub fn new() -> Self{
        EtcdUnauthenticatedAccess{}
    }
}

//EtcdUnauthenticatedAccess 的名字及介绍
impl Module for EtcdUnauthenticatedAccess{
    fn name(&self) -> String {
        String::from("http/etc_unauthenticated_access")
    }
    fn description(&self) -> String {
        String::from("Check for CoreOS' etcd Unauthenticated Access")
    }
}

//poc scan
#[async_trait]
impl HttpModule for EtcdUnauthenticatedAccess{
    async fn scan(
        &self,
        http_client:&Client,
        endpoint:&str,   
    ) -> Result<Option<HttpFinding>,Error>{
        let url = format!("{}/version",&endpoint);
        let res = http_client.get(&url).send().await?;  //获取response

        if !res.status().is_success(){  //如果response 返回不是200 就返回Ok(None)
            return Ok(None);
        }
        let body = res.text().await?;  //获取body 的内容
        //println!("{:?}",body.chars().count());
        if body.contains(r#""etcdserver""#)
            &&body.contains(r#""etcdcluster""#)
            &&body.chars().count()<200{ //body中存在"etcdserver"
            return Ok(Some(HttpFinding::EtcdUnauthenticatedAccess(url)));
        }


        Ok(None)
    }
}