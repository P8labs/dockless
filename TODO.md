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

- [x] Fix UI bugs and design it for better looks
- [x] Fixed backend service state management bug

[24/02/2026]

- [x] Add A way to add service specific configs
- [x] Add port forwarding management
- [x] Service creation in parts.
- [x] Add streaming logs view in portal
- [x] Add system usage service specific
- [x] fixed some UI bugs and add more details appbar
- [x] offload the logs from memory to disk saving RAM
- [x] Fixed service config template and real values bug
- [x] deallocating and killing the service from port.
- [x] addes settings page to edit the service definations
- [x] add option to refresh and clean logs

[25/02/2026]

- [x] add landing page website and docs.
- [x] added github actions and dockerfile to make a release.

[26/02/2026]

- [x] Make a beautiful landing page
- [x] Write docs about how to setup dockless
- [x] Sanitize the SEO and metadata
- [ ] Release first version and test everything works
