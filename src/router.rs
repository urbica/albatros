use iron::Chain;
use mount::Mount;
use iron_router::Router;
use project::project_controller;

fn api_v1() -> Router {
    let mut router = Router::new();

    router.get("/projects", project_controller::index, "projects index");
    router.post("/projects", project_controller::create, "create project");
    router.get("/projects/:id", project_controller::get, "get project");

    router
}

pub fn create() -> Chain {
    let mut mount = Mount::new();
    mount.mount("/api/v1/", api_v1());

    Chain::new(mount)
}
