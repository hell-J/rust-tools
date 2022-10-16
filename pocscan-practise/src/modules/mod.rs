use async_trait::async_trait;
use reqwest::Client;
use crate::Error;
mod http;

pub trait Module{ //此trait 是设置漏洞的具体信息 包括漏洞名称及详情
    fn name(&self) -> String;
    fn description(&self) -> String;
}
#[derive(Debug)]
pub enum HttpFinding{ //此枚举表明漏洞类型类型及地址
    GitlabOpenRegistrations(String),
    GitHeadDisclosure(String),
    DotEnvDisclosure(String),
    DsStoreDisclosure(String),
    EtcdUnauthenticatedAccess(String),
    KibanaUnauthenticatedAccess(String),
    DirectoryListingDisclosure(String),
}

#[async_trait]
pub trait HttpModule:Module { //此trait实现了漏洞的发现 返回HttpFinding类型
    async fn scan(
        &self,
        http_client:&Client,
        endpoint:&str,   
    ) -> Result<Option<HttpFinding>,Error>;
}

pub fn all_http_modules() -> Vec<Box<dyn HttpModule>>{ //此函数方便获取到所有的漏洞类型
    return vec![
        Box::new(http::GitlabOpenRegistrations::new()),
        Box::new(http::GitHeadDisclosure::new()),
        Box::new(http::DotEnvDisclosure::new()),
        Box::new(http::DsStoreDisclosure::new()),
        Box::new(http::EtcdUnauthenticatedAccess::new()),
        Box::new(http::KibanaUnauthenticatedAccess::new()),
        Box::new(http::DirectoryListingDisclosure::new()),
    ];
}