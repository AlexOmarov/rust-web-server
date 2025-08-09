# News Service 🗞️

A service that runs an analytical Telegram channel based on current news.

---

## Introduction 👨‍💻

The service is built on **[Ktor](https://ktor.io/)**, a powerful framework for building web applications in Kotlin.

The main features of the service include:

- Collecting relevant news from various global sources 🌍
- Analyzing the collected news using Large Language Models (LLM) 🤖.
- Generating and sending messages to a Telegram channel 💬.

The repository contains the source code and a `docker-compose.yml` file for local deployment of the service.

A complete list of used libraries can be found in the `libs.versions.toml` file (which is pulled from a shared repository).

All the documentation is available in the [docs](docs) folder and published on related pages site.

### Third-Party Systems 🛠️:

- **[Ollama](https://ollama.com/)**: A platform for large language models (LLM).
- **[OpenTelemetry Collector Contrib](https://github.com/open-telemetry/opentelemetry-collector-contrib/)**: A community-maintained implementation of otel collector.
- **[Postgres](https://www.postgresql.org/)**: A powerful, open-source relational database system.
- **[Telegram](https://telegram.org/)**: A cloud-based messaging service used for the channel integration.

## Requirements ✅
To build and run the project, you need:

 - [Docker](https://www.docker.com/) installed on your local machine.
 - Several env variables being set:
     - **CI_SERVER_URL**: set gitlab host here
     - **GITLAB_PRIVATE_TOKEN**: issue your personal access token from gitlab's profile page
     - **PROJECT_BOT**: get your bot token from [BotFather](https://core.telegram.org/bots/tutorial) in a telegram
     - **PROJECT_CHAT**: create your chat and get its id, then add your bot to that chat

You can then import the project directly using the following menu navigation:
`File -> New -> Project from Existing Sources -> Gradle`

## Launch 🚀

The easiest way to launch the service and all its dependencies is by using Docker.

```bash
docker-compose up -d
```
Docker Compose includes the following services, all of which can be accessed via `localhost`:

1. **Postgres** - `localhost:5432`
2. **Service** - `localhost:8080`
3. **Ollama** - `localhost:11434`

The service can also be deployed outside Docker, using the compiled source code instead of the Docker image.

To do this:

1. Begin by deploying the services using the `docker-compose.yml` file.
2. Once the services are running, stop the container for the service (`service`).
3. Build the project and run it using the built-in capabilities of your IDE.

## Usage 📡

The `Healthcheck` APIs are available at the following endpoints:

- `/liveness`
- `/readiness`

Additionally, you can manually access the service APIs at `localhost:8080`.  
The APIs use **JSON** data format.

## Testing 📊

Testing and coverage measurement are automatically performed when the `build` task is executed.  
Coverage is measured using the **Kover** tool, which relies on the **JaCoCo** engine.

Coverage reports are generated in the following formats:
- **XML** format for Sonar tools: `./build/app/reports/kover/report.xml`
- **HTML** format for local development: `./build/app/reports/kover/html/index.html`

You can also manually trigger testing and coverage measurement using the following command:
```bash  
.\gradlew test koverPrintCoverage
```  
You can also view the coverage percentage directly in the IDE. To do so:

1. Right-click on the **test** folder.
2. Select **Run with Coverage**.

**Important!** The tests will not pass if Docker Compose is running at the same time!

## Quality 🚦

When the project is built using the `build` task in Gradle, **Detekt** and **ktlint** checks are run automatically.  
Any issues detected by **Detekt** will be automatically fixed, simply rebuild the project after the task fails.

Detekt reports can be found at the following paths:
- `./build/app/reports/detekt`
- `./build/api/reports/detekt`

You can also run the checks manually with the following command:

```bash  
.\gradlew detekt
```

The Quality Gate process is implemented using the Gradle plugin SonarQube. 
To trigger the quality gate, run the following commands:
```bash  
.\gradlew build
.\gradlew sonar \
  -D"sonar.host.url"="<SONAR_HOST>" \
  -D"sonar.token"="YOUR_TOKEN" \
  -D"sonar.projectKey"="KEY" \
  -D"sonar.organization"="ORG"
```
When running SonarQube through the Gradle task, the generated Detekt and Kover reports will be included in the analysis.

## Publishing 📦

For testing purposes, it can be useful to publish packages locally to **mavenLocal**.  
To do this, use the following command:

```bash  
.\gradlew publishToMavenLocal
```  
After executing this command, the artifact `<SERVICE_NAME>-api` will appear in the `.m2` directory of the user.
This artifact will contain all the necessary interfaces required to interact with the service.