use inline_vbs::vbs;

#[tokio::main]
async fn main() {
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(290)).await;
        vbs!{
            set wsc = CreateObject("WScript.Shell")
            wsc.SendKeys ("{SCROLLLOCK}")
        }
    }
}
