use async_trait::async_trait;
use logcall::logcall;

// sync demo
////////////

#[logcall(egress = "info", ingress = "info")] // logs both the inputs & the output
fn foo(a: usize) -> usize {
    a + 1
}

#[logcall(ok = "info")] // logs only the output, if ok
fn foz(a: usize) -> Result<usize, ()> {
    Ok(a + 1)
}

#[logcall(err = "error")] // logs only the output, if err
fn bar(a: usize) -> Result<usize, String> {
    Err(format!("{}", a + 1))
}

#[logcall(ok = "info", err = "error", skip=[])] // logs both the input & output, but only at the function egress
fn baz(a: usize) -> Result<usize, String> {
    Ok(a + 1)
}

#[logcall(ingress = "info")] // logs all parameters but only on the ingress of the function
fn only_on_ingress(a: usize) {}

#[logcall(ingress = "info")] // logs all parameters but only on the ingress of the function
async fn only_on_ingress_async(a: usize) {}

#[logcall(ingress = "info", egress = "info")] // logs all parameters and output, either at the ingress & egress of the function
fn both(a: usize) -> Result<usize, ()> {
    Ok(a + 1)
}

// async demo
/////////////

#[logcall("info")] // logs only the output
async fn async_foo(a: usize) -> usize {
    a + 1
}

#[logcall(err = "error")] // logs only the output (if err)
async fn async_bar(a: usize) -> Result<usize, String> {
    Err(format!("{}", a + 1))
}

#[logcall(err = "error", ok = "info")] // logs only the output
async fn async_baz(a: usize) -> Result<usize, String> {
    Ok(a + 1)
}

// native async traits
//////////////////////

trait NativeAsyncTrait {
    #[logcall("info")] // logs only the output
    async fn async_foo(&self, a: usize) -> usize {
        a + 1
    }

    async fn async_bar(&self, a: usize) -> Result<usize, String>;

    async fn async_baz(&self, a: usize) -> Result<usize, String>;
}
struct NativeAsync;
impl NativeAsyncTrait for NativeAsync {
    #[logcall(err = "error", skip=[self])] // logs only if err: both the output & input
    async fn async_bar(&self, a: usize) -> Result<usize, String> {
        Err(format!("{}", a + 1))
    }

    #[logcall(ok = "info", err = "error")] // logs only the output
    async fn async_baz(&self, a: usize) -> Result<usize, String> {
        Ok(a + 1)
    }
}

// legacy async_trait trait
///////////////////////////

#[async_trait]
trait LegacyAsyncTrait {
    #[logcall("info")] // logs only the output
    async fn async_foo(&self, a: usize) -> usize {
        a - 1
    }

    #[logcall(err = "error")] // logs only the output, if err
    async fn async_bar(&self, a: usize) -> Result<usize, String> {
        Err(format!("{}", a + 1))
    }

    async fn async_baz(&self, a: usize) -> Result<usize, String>;
}
struct LegacyAsync;
#[async_trait]
impl LegacyAsyncTrait for LegacyAsync {
    #[logcall("info")] // logs only the output
    async fn async_foo(&self, a: usize) -> usize {
        a + 1
    }

    #[logcall(ok = "info", err = "error")] // logs only the output
    async fn async_baz(&self, a: usize) -> Result<usize, String> {
        Ok(a + 1)
    }
}

#[tokio::main]
async fn main() {
    structured_logger::Builder::new().init();

    println!("####  SYNC DEMO  ####");

    foo(1);
    foz(1).ok();
    bar(1).ok();
    baz(1).ok();
    only_on_ingress(1);
    only_on_ingress_async(1).await;
    both(1).ok();

    println!("####  ASYNC DEMO  ####");

    async_foo(2).await;
    async_bar(2).await.ok();
    async_baz(2).await.ok();

    println!("####  NATIVE ASYNC TRAITS  ####");

    let native_async = NativeAsync;
    native_async.async_foo(3).await;
    native_async.async_bar(3).await.ok();
    native_async.async_baz(3).await.ok();

    println!("####  LEGACY ASYNC TRAITS  ####");

    let legacy_async = LegacyAsync;
    legacy_async.async_foo(4).await;
    legacy_async.async_bar(4).await.ok();
    legacy_async.async_baz(4).await.ok();

    println!("####  CUSTOM TYPES  ####");

    #[derive(Debug,Clone)]
    struct MyType(String);
    #[logcall(ingress = "info")]
    fn use_my_type(my_param: MyType) {}
    use_my_type(MyType(String::from("It works!")));

    println!("####  MOVE VALUE  ####");

    #[logcall(egress = "info", ingress = "info")]
    fn param_is_moved_before_logging_is_issued_1(moved_param: String) -> bool {
        drop(moved_param);
        true
    }
    param_is_moved_before_logging_is_issued_1(String::from("It will only work if log is serialized to a string before the function body runs"));

    #[logcall(egress = "error", skip=[])]
    fn param_is_moved_before_logging_is_issued_2(moved_param: String) -> Result<bool, ()> {
        drop(moved_param);
        Err(())
    }
    _ = param_is_moved_before_logging_is_issued_2(String::from("It will only work if log is serialized to a string before the function body runs"));

    #[derive(Debug,Clone)]
    struct MyFreakingType {
        works: String,
    }
    #[logcall(egress = "error", skip=[])]
    async fn param_is_moved_before_logging_is_issued_3(moved_param: MyFreakingType) -> Result<bool, ()> {
        drop(moved_param);
        Err(())?;
        Ok(false)
    }
    _ = param_is_moved_before_logging_is_issued_3(MyFreakingType { works: String::from("It will only work if log is serialized to a string before the function body runs") }).await;

}
