[21/02/2026]

- [x] Create a persist node id file to identify the node.
- [x] Setup a simple API server.
- [x] Add Gracefull shutdown.
- [x] add logging and good error handling.
- [x] then add config file to control dockless
- [x] add a supervisior to spawn and watch the project.
- [x] add Service state machine to manage the bin status.
- [x] add persist json registry of project config.
- [x] Design SupervisorManager layer to control the service state with API
- [x] extend service definition to input args, env, work dir, and restart limit
- [x] perist registry on restarts and add API route to create and delete service

[22/02/2026]

- [ ] Refactor project into module-based structure (separate runtime, registry, api layers)
- [ ] Create data/services/<id>/ directory structure on service creation
- [ ] Enforce service binary must live inside its own service directory
- [ ] Add working_dir support in Service + Supervisor spawn
- [ ] Add GET /registry endpoint to expose raw definitions
- [ ] Implement restart backoff (replace fixed 3s with exponential)
- [ ] Add Failed state handling visibility in /services output
- [ ] Add basic validation layer for ServiceDefinition (id format, path safety)
- [ ] Add structured per-service log prefix formatting
