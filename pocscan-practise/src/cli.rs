use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::time::Duration;
use std::time::Instant;
use anyhow::{Context,Result};
//真正的调用函数
use futures::stream;
use futures::StreamExt;
use reqwest::Client;
use crate::{modules,Error};
use crate::modules::HttpModule;

pub fn modules(){
    let http_modules = modules::all_http_modules();
    println!("HTTP modules");
    for module in http_modules{
        println!("    {}:{}",module.name(),module.description());
    }
}

pub fn scan(target:&str) ->Result<(),Error>{ //poc加载器
    log::info!("Scanning: {}",target);
    let scan_start = Instant::now();
    let http_timeout = Duration::from_secs(10);
    let http_client = Client::builder().timeout(http_timeout).build()?;
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Building tokio's runtime"); //tokio异步运行时
    
    //通过异步运行时进行异步
    runtime.block_on(async move{
        let mut targets:Vec<(Box<dyn HttpModule>,String)> = Vec::new(); //将targets设置成(HttpModule,target) 方便实现一个target对多个poc
        let http_modules = modules::all_http_modules(); //获取到所有poc
        for http_module in http_modules{
            let target = format!("{}",&target);
            targets.push((http_module,target));
        } //使每个target都对应所有的poc
        stream::iter(targets.into_iter())
            .for_each_concurrent(20, |(module,target)|{
                let http_client = http_client.clone();
                async move{
                    match module.scan(&http_client, &target).await {
                        Ok(Some(finding)) => println!("{:?}",&finding),
                        Ok(None) => {}
                        Err(err) => log::debug!("Error:{}",err),
                    }
                }
            }).await;
    });
    let scan_duration = scan_start.elapsed();
    log::info!("Scan completed in {:?}",scan_duration);

    Ok(())
}

pub fn scans(targets:&str) -> Result<(),Error>{
    let f = File::open(targets).with_context(||format!("could not read file {:?}",targets))?;
    let lines = BufReader::new(f).lines();
    for line in lines{
        let target = line.unwrap();
        scan(&target)?;
    }
    Ok(())
}
