# Rust AI Code Builder 🤖

> **App in Rust to build web apps automatically using AI**

Part of the **Coursera Advanced Rust Programming and AutoGPT Development** course series.

## 📖 Overview

Rust AI Code Builder is an advanced AutoGPT-style system that uses multiple AI agents to automatically design, plan, and build web applications based on natural language descriptions. The system leverages OpenAI's GPT-4 to power intelligent agents that collaborate to transform user requirements into fully-functional web applications.

## 🏗️ Architecture

The project implements a **multi-agent architecture** with specialized AI agents that work collaboratively:

### Agent Hierarchy

```
Managing Agent
    ├── Solution Architect Agent
    │   └── Analyzes requirements and designs system architecture
    ├── Backend Developer Agent
    │   └── Implements backend code and API endpoints
    └── [Extensible for additional agents]
```

### Core Components

- **Agent System**: Modular agent framework with basic traits and specialized functions
- **AI Functions**: Integration with OpenAI's function calling capabilities
- **Command Line Interface**: Interactive colored terminal output for agent communication
- **FactSheet**: Central data structure tracking project state and decisions

## ✨ Features

- 🎯 **Intelligent Project Scoping**: Automatically analyzes requirements and determines:
  - CRUD functionality needs
  - User authentication requirements
  - External API integration requirements

- 🔄 **Multi-Agent Collaboration**: Specialized agents work together with defined roles and responsibilities

- 🎨 **Colorized Terminal Output**: Visual feedback with color-coded agent messages:
  - 🟢 Green: Agent identification
  - 🔵 Cyan: AI API calls
  - 🟣 Magenta: Unit tests
  - 🔴 Red: Issues/errors

- ⚡ **Async/Await Architecture**: Built on Tokio for efficient async operations

- 🔌 **OpenAI GPT-4 Integration**: Leverages state-of-the-art language models for intelligent decision-making

## 🛠️ Technology Stack

- **Language**: Rust
- **Async Runtime**: Tokio
- **HTTP Client**: Reqwest
- **Serialization**: Serde & Serde JSON
- **Terminal UI**: Crossterm
- **AI Integration**: ai_functions crate
- **API**: OpenAI GPT-4

## 📋 Prerequisites

- Rust 1.70+ (with 2024 edition support)
- OpenAI API key
- Internet connection for API calls

## 🚀 Installation

1. **Clone the repository**:
   ```bash
   git clone https://github.com/<your-username>/rust-ai-code-builder.git
   cd rust-ai-code-builder
   ```

2. **Set up environment variables**:
   Create a `.env` file in the project root:
   ```env
   OPEN_AI_KEY=your_openai_api_key_here
   ```

3. **Build the project**:
   ```bash
   cargo build --release
   ```

## 💻 Usage

Run the application:

```bash
cargo run
```

The system will prompt you with:
```
What webserver are we building today?
```

Provide a natural language description of your desired web application, for example:
- "I need a full stack website that accepts users and gets stock price data"
- "Build a TODO app with user authentication"
- "Create a blog platform with CRUD operations"

The AI agents will then:
1. Analyze your requirements
2. Design the system architecture
3. Plan API endpoints
4. Generate backend code
5. Create necessary schemas

## 📁 Project Structure

```
rust-ai-code-builder/
├── src/
│   ├── main.rs                 # Entry point
│   ├── ai_functions/           # AI function definitions
│   │   ├── aifunc_architect.rs # Architecture planning functions
│   │   ├── aifunc_backend.rs   # Backend development functions
│   │   └── aifunc_managing.rs  # Management functions
│   ├── apis/
│   │   └── call_request.rs     # OpenAI API integration
│   ├── helpers/
│   │   ├── command_line.rs     # CLI utilities
│   │   └── general.rs          # General helpers
│   └── models/
│       ├── agent_basic/        # Base agent traits and structures
│       ├── agents/             # Specialized agent implementations
│       ├── agents_manager/     # Managing agent
│       └── general/            # Shared models (LLM structures)
├── schemas/
│   └── api_schema.json         # API schema definitions
├── Cargo.toml                  # Dependencies
└── .env                        # Environment variables (create this)
```

## 🎓 Learning Objectives

This project demonstrates advanced Rust concepts including:

- **Async/Await Programming**: Efficient concurrent operations with Tokio
- **Trait-Based Design**: Flexible agent system using traits
- **Error Handling**: Robust error propagation with `Result` types
- **Type Safety**: Leveraging Rust's type system for reliable code
- **Module Organization**: Clean code structure and separation of concerns
- **API Integration**: Working with external REST APIs
- **Macro Usage**: Custom macros for code generation
- **Testing**: Unit tests for critical components

## 🔧 Key Dependencies

```toml
tokio = { version = "1.28.0", features = ["full"] }
reqwest = { version = "0.11.17", features = ["json"] }
serde = { version = "1.0.160", features = ["derive"] }
ai_functions = "0.1.1"
async-trait = "0.1.68"
crossterm = "0.26.1"
```

## 🧪 Testing

Run the test suite:

```bash
cargo test
```

Run a specific test:

```bash
cargo test tests_call_to_openai
```

## 🔐 Security Notes

- Never commit your `.env` file or expose your OpenAI API key
- The `.env` file should be added to `.gitignore`
- API calls consume OpenAI credits - monitor your usage

## 📝 Development Notes

### Agent State Machine

Agents progress through defined states:
- `Discovery`: Initial information gathering
- `Working`: Active development
- `UnitTesting`: Testing phase
- `Finished`: Task completion

### Extending the System

To add a new agent:
1. Create a new struct implementing `SpecialFunctions` trait
2. Define the agent's specific logic in the `execute` method
3. Register the agent with the Managing Agent

## 🤝 Contributing

This is an educational project from the Coursera course series. Feel free to:
- Fork the repository
- Experiment with new agent types
- Enhance AI function definitions
- Improve error handling

## 📚 Related Courses

This project is part of the **Advanced Rust Programming and AutoGPT Development** course series on Coursera, covering:
- Advanced Rust programming patterns
- AI agent architectures
- GPT integration and function calling
- Async programming in Rust
- Building intelligent automation systems

## ⚠️ Limitations

- Requires active internet connection
- OpenAI API costs apply
- GPT-4 response times vary
- Currently optimized for web application generation

## 📄 License

[Specify your license here]

## 🙏 Acknowledgments

- Coursera Advanced Rust Programming course
- OpenAI for GPT-4 API
- Rust community for excellent tooling

## 📮 Contact

[Your contact information or course forum link]

---

**Note**: This is an educational project designed to teach advanced Rust programming concepts and AI agent development. Use responsibly and be mindful of API usage costs.

