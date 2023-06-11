use crate::todo_service::todo_service_server::{TodoService, TodoServiceServer};
use crate::todo_service::{CreateTodoRequest, CreateTodoResponse, Empty, ListTodoResponse, Todo};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;
use futures::TryFutureExt;
use std::sync::{Arc, Mutex};
use todo::{create_task, establish_connection};
use tonic::{transport::Server, Request, Response, Status};

pub mod todo_service {
    tonic::include_proto!("service");
}

pub struct MyTodoService {
    conn: Pool<ConnectionManager<SqliteConnection>>,
}

impl MyTodoService {
    pub fn new(conn: Pool<ConnectionManager<SqliteConnection>>) -> Self {
        MyTodoService { conn }
    }
}

#[tonic::async_trait]
impl TodoService for MyTodoService {
    async fn create_todo(
        &self,
        request: Request<CreateTodoRequest>,
    ) -> Result<Response<CreateTodoResponse>, Status> {
        let r = request.into_inner();
        let resp = create_task(
            &mut self.conn.get().expect("failed to get sqlite conn"),
            r.message.clone(),
            r.message.clone(),
        );
        let rep = CreateTodoResponse {
            todo: Option::from(Todo {
                id: "".to_string(),
                create_time: "".to_string(),
                update_time: "".to_string(),
                status: "".to_string(),
                title: "asdasd".to_string(),
                message: resp.message,
            }),
        };
        Ok(Response::new(rep))
    }

    async fn add_todo(
        &self,
        request: Request<CreateTodoRequest>,
    ) -> Result<Response<CreateTodoResponse>, Status> {
        todo!()
    }

    async fn update_todo(
        &self,
        request: Request<CreateTodoRequest>,
    ) -> Result<Response<CreateTodoResponse>, Status> {
        todo!()
    }

    async fn list_todo(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<ListTodoResponse>, Status> {
        todo!()
    }
    // async fn say_hello(
    //     &self,
    //     request: Request<HelloRequest>,
    // ) -> Result<Response<HelloReply>, Status> {
    //     println!("Got a request: {:?}", request);
    //
    //     let reply = hello_world::HelloReply {
    //         message: format!("Hello {}!", request.into_inner().name).into(),
    //     };
    //
    //     Ok(Response::new(reply))
    // }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let conn = establish_connection();
    let greeter = MyTodoService::new(conn);

    Server::builder()
        .add_service(TodoServiceServer::new(greeter))
        .serve(addr)
        .await?;
    Ok(())
}
