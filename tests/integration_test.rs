// importing common module.
mod common;
extern crate junit_report;

use std::{str};

use lambda::{Blob, Client, Config, Region, SdkError, Credentials};

use smithy_http::endpoint::Endpoint;
use http::Uri;

use junit_report::{ReportBuilder, TestCase, TestCaseBuilder, TestSuite, TestSuiteBuilder, Duration, TimeZone, Utc};
use std::fs::File;

macro_rules! aw {
  ($e:expr) => {
      tokio_test::block_on($e)
  };
}

#[test]
fn test_add() {
    let region = Region::new("us-west-2");
    let config = Config::builder()
        .region(region)
        .credentials_provider(Credentials::from_keys("FOO", "BAR", Some("TOKEN".to_string())))
        .endpoint_resolver(Endpoint::immutable(Uri::from_static("http://127.0.0.1:3001/")))
        .build();
    let client = Client::from_conf(config);
    let text = "{\"path\":\"/foo/bar\"}";
    let blob = Blob::new(text.as_bytes());
    match aw!(client.invoke().function_name("HelloWorldFunction").payload(blob).send()) {
        Ok(resp) => {
            if let Some(blob) = resp.payload {
                let s = str::from_utf8(blob.as_ref()).expect("invalid utf-8");
                println!("Response: {:?}", s);
                let test_success = TestCaseBuilder::success("good test", Duration::seconds(15))
                    .set_classname("MyClass")
                    .build();

                let timestamp = Utc.ymd(1970, 1, 1).and_hms(0, 1, 1);
                let ts2 = TestSuiteBuilder::new("ts2")
                    .set_timestamp(timestamp)
                    .add_testcase(test_success)
                    .build();
                let r = ReportBuilder::new()
                    .add_testsuite(ts2)
                    .build();
                let mut file = File::create("my-junit.xml").unwrap();
                r.write_xml(&mut file).unwrap();
            }
        }
        Err(SdkError::ServiceError { err, .. }) if err.is_resource_not_found_exception() => {
            println!("This lambda function does not exist: {}", err);
            assert_eq!(true, false);
        }
        Err(err) => {
            println!("Got an error invoking the function: {}", err);
            assert_eq!(true, false);
        }
    };
}