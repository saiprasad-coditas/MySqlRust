use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

pub struct AppState {
    pub managers: Arc<Mutex<Vec<Manager>>>,
    pub employees: Arc<Mutex<Vec<Employee>>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Manager {
    pub id: Option<u32>,
    pub name: String,
    pub role: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Employee {
    pub id: Option<u32>,
    pub name: String,
    pub role: String,
    pub manager_id: Option<u32>,
}

// Manager API Handlers
pub async fn add_manager(manager: web::Json<Manager>, state: web::Data<AppState>) -> impl Responder {
    let mut manager = manager.into_inner();
    let mut managers = state.managers.lock().unwrap();

    // Simulate database insertion
    manager.id = managers.last().map(|last_manager| last_manager.id.unwrap_or(0) + 1);
    managers.push(manager.clone());

    HttpResponse::Created().json(manager)
}

pub async fn get_managers(state: web::Data<AppState>) -> impl Responder {
    let managers = state.managers.lock().unwrap();
    HttpResponse::Ok().json(&*managers)
}

pub async fn update_manager(
    path: web::Path<u32>,
    manager: web::Json<Manager>,
    state: web::Data<AppState>,
) -> impl Responder {
    let manager_id = path.into_inner();
    let mut managers = state.managers.lock().unwrap();

    if let Some(existing_manager) = managers.iter_mut().find(|m| m.id == Some(manager_id)) {
        *existing_manager = manager.into_inner();
        HttpResponse::Ok().json(existing_manager.clone())
    } else {
        HttpResponse::NotFound().body("Manager not found")
    }
}

pub async fn delete_manager(path: web::Path<u32>, state: web::Data<AppState>) -> impl Responder {
    let manager_id = path.into_inner();
    let mut managers = state.managers.lock().unwrap();

    if let Some(index) = managers.iter().position(|m| m.id == Some(manager_id)) {
        let deleted_manager = managers.remove(index);
        HttpResponse::Ok().json(deleted_manager)
    } else {
        HttpResponse::NotFound().body("Manager not found")
    }
}

// Employee API Handlers
pub async fn add_employee(employee: web::Json<Employee>, state: web::Data<AppState>) -> impl Responder {
    let mut employee = employee.into_inner();
    let mut employees = state.employees.lock().unwrap();

    // Simulate database insertion
    employee.id = employees.last().map(|last_employee| last_employee.id.unwrap_or(0) + 1);
    employees.push(employee.clone());

    HttpResponse::Created().json(employee)
}

pub async fn get_employees(state: web::Data<AppState>) -> impl Responder {
    let employees = state.employees.lock().unwrap();
    HttpResponse::Ok().json(&*employees)
}

pub async fn update_employee(
    path: web::Path<u32>,
    employee: web::Json<Employee>,
    state: web::Data<AppState>,
) -> impl Responder {
    let employee_id = path.into_inner();
    let mut employees = state.employees.lock().unwrap();

    if let Some(existing_employee) = employees.iter_mut().find(|e| e.id == Some(employee_id)) {
        *existing_employee = employee.into_inner();
        HttpResponse::Ok().json(existing_employee.clone())
    } else {
        HttpResponse::NotFound().body("Employee not found")
    }
}

pub async fn delete_employee(path: web::Path<u32>, state: web::Data<AppState>) -> impl Responder {
    let employee_id = path.into_inner();
    let mut employees = state.employees.lock().unwrap();

    if let Some(index) = employees.iter().position(|e| e.id == Some(employee_id)) {
        let deleted_employee = employees.remove(index);
        HttpResponse::Ok().json(deleted_employee)
    } else {
        HttpResponse::NotFound().body("Employee not found")
    }
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/manager")
            .route(web::post().to(add_manager))
            .route(web::get().to(get_managers)),
    )
    .service(
        web::resource("/manager/{id}")
            .route(web::put().to(update_manager))
            .route(web::delete().to(delete_manager)),
    )
    .service(
        web::resource("/employee")
            .route(web::post().to(add_employee))
            .route(web::get().to(get_employees)),
    )
    .service(
        web::resource("/employee/{id}")
            .route(web::put().to(update_employee))
            .route(web::delete().to(delete_employee)),
    );
}