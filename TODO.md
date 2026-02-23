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

- [x] Refactor project into module-based structure (separate runtime, registry, api layers)
- [x] Create data/services/<id>/ directory structure on service creation
- [x] Enforce service binary must live inside its own service directory
- [x] Add working_dir support in Service + Supervisor spawn
- [x] Add GET /registry endpoint to expose raw definitions
- [x] Implement restart backoff (replace fixed 3s with exponential)
- [x] Add basic validation layer for ServiceDefinition (id format, path safety)
- [x] Implement artifact/upload endpoint (versioned)
- [x] Implement GitHub release downloader (basic)
- [x] Create portal/ project using SvelteKit
- [x] Configure static adapter (SSG output)
- [x] Build production and verify static output

[23/02/2026]

- [ ] Fix UI bugs and design it for better looks
- [ ] Add streaming logs view in portal
- [ ] Add system usage service specific
