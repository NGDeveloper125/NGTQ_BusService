import React from "react";
import "../style/PageBody.css";

type PageBodyProp = {
    currentPage: string
}

function PageBody(props: PageBodyProp) {
    return (
    <div className="page-body">
        <div className="about-container">
            <h2>NGTQ Bus Services</h2>

            <div className="intro-section">
                <p className="lead">
                    The NGTQ Bus Services project represents a new approach to implementing message bus services in Rust, built around a flexible and extensible task queue abstraction.<br/>At its core, this project introduces the NGTQ (Next Generation Task Queue) trait, which serves as a foundation for creating interchangeable task queue implementations.
                </p>
            </div>

            <div className="goals-section">
                <h3>Project Goals</h3>
                <p>
                    Our primary goal is to provide developers with a collection of bus services that can adapt to different use cases while maintaining a consistent interface.<br/> The project's architecture separates concerns into three distinct layers: the NGTQ abstraction layer, concrete task queue implementations, and bus services that utilize them.
                </p>
            </div>

            <div className="architecture-section">
                <h3>Architecture</h3>
                <p>
                    The NGTQ abstraction layer, defined in the ngtq crate, establishes a standard interface that any task queue implementation must follow.<br/> This design choice ensures that developers can swap different queue implementations without modifying their bus service code, promoting flexibility and maintainability.
                </p>
            </div>

            <div className="current-status-section">
                <h3>Current Implementation</h3>
                <p>
                    Currently, we offer the ngtask_queue as our reference implementation of the NGTQ trait, demonstrating how to create a concrete task queue that adheres to the abstraction.<br/> The ngtq_bus_service, our first bus service implementation, showcases how to build a message bus using Unix domain sockets for inter-process communication while leveraging the NGTQ abstraction.
                </p>
            </div>

            <div className="future-section">
                <h3>Future Development</h3>
                <p>
                    Future releases will introduce additional bus service implementations, each designed for specific use cases and environments.<br/> We'll provide comprehensive documentation, including installation guides, performance benchmarks, and detailed comparisons to help developers choose the right implementation for their needs.
                </p>
            </div>

            <div className="community-section">
                <h3>Community Involvement</h3>
                <p>
                    As an open-source project, we welcome community contributions and feedback.<br/> Whether you're interested in creating new task queue implementations, developing bus services for different protocols, or improving our documentation, there are many ways to get involved.
                </p>
            </div>

            <div className="documentation-section">
                <h3>Documentation</h3>
                <p>
                    Our documentation will continue to expand, covering topics such as implementation guides, API references, performance optimizations, and best practices.<br/> We're committed to maintaining clear, practical documentation that helps developers understand and effectively use our components.
                </p>
            </div>

            <div className="closing-section">
                <p>
                    The project is actively maintained and growing.<br/> We're currently working on new bus service implementations, comprehensive benchmarking suites, and expanded documentation.<br/> Stay connected through our GitHub repository to keep up with the latest developments and contribute to the project's evolution.
                </p>
            </div>
        </div>
    </div>
    );
}

export default PageBody;