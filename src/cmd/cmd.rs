use inquire::Select;
use models::message::{NewTask, Task, UpdateTask};
use std::env;

#[tokio::main]
async fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        args.remove(0);
        create_task(args.join(" ")).await.expect("WTF!!!");
        return;
    }

    // просмотр тасок и вкл выкл
    let tasks = get_tasks().await.expect("wtf");
    let ans = Select::new("?", tasks).with_page_size(16).prompt();

    match ans {
        Ok(choice) => {
            toggle_task(choice).await.expect("wtf");
            println!("Ok")
        }
        Err(_) => println!("There was an error, please try again"),
    }
}

const TARGET: &str = "http://192.168.1.42:3456";
async fn get_tasks() -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    let resp = reqwest::get(TARGET).await?.json::<Vec<Task>>().await?;
    Ok(resp)
}

async fn create_task(t: String) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    client
        .post(TARGET)
        .json(&NewTask { name: t })
        .send()
        .await
        .unwrap()
        .error_for_status()
        .map_err(|err| println!("WTF!!! {}", err))
        .unwrap()
        .json::<Task>()
        .await
        .unwrap();

    Ok(())
}

async fn toggle_task(t: Task) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let toggle = match t.status.as_str() {
        "init" => format!("{}/task/done", TARGET),
        "done" => format!("{}/task/undone", TARGET),
        _ => "wtf".to_string(),
    };

    client
        .post(toggle)
        .json(&UpdateTask { id: t.id })
        .send()
        .await
        .unwrap()
        .error_for_status()
        .map_err(|err| println!("WTF!!! {}", err))
        .unwrap()
        .json::<Task>()
        .await
        .unwrap();

    Ok(())
}
