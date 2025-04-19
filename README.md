# NGTQ_BusService

A fast and lightweight message bus service implemented in Rust, powered by NGTaskQueue for efficient task queue management.

## Overview

NGTQ_BusService is a high-performance message bus service that facilitates communication between different services using IPC (Inter-Process Communication) sockets. Built with Rust, it provides a robust and efficient solution for message queuing and service communication.

### Core Components

- **NGTaskQueue**: A core task queue management system (to be extracted as a separate crate)
- **Receiver Layer**: Abstraction layer handling queue operations through different protocols
- **IPC Socket Implementation**: Primary communication protocol for service interaction

### Task Queue Types

The service supports two distinct types of task queues:

1. **ID-Based Queue**
   - Single unified queue for all ID-based tasks
   - Each task has a unique identifier
   - Tasks can be pulled specifically by their ID
   - Ideal for targeted task processing

2. **Category-Based Queues**
   - Dynamic queue creation based on task categories
   - Separate queue maintained for each category
   - First-In-First-Out (FIFO) processing within each category
   - Automatic new queue creation for new categories
   - Perfect for topic-based processing and workload separation

## Current Features

- Task queue management with push/pull operations
- Support for ID-based and category-based task queues
- IPC socket-based communication
- Local and containerized deployment options
- Native Rust service support

## Roadmap

### Short-term Goals
- Complete core NGTaskQueue implementation
- Implement basic IPC socket receiver
- Establish basic service functionality
- Create comprehensive test suite

### Long-term Goals
- Extract NGTaskQueue as a standalone Rust crate
- Implement additional receiver protocols
- Create language-specific facades for:
  - Python
  - Node.js
  - Java
  - .NET
- Enhanced monitoring and management tools
- Performance optimization and scaling capabilities

## Usage

*Detailed usage instructions will be added as the project develops*

### Basic Example (Coming Soon)
```rust