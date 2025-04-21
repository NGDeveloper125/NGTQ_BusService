import React from "react";
import NavigationBar from "./NavigationBar"

type HeadingProps = {
    currentPage: string
}

function Heading(props: HeadingProps) {
    return (
    <div>
        <h1>NGTaskQueue Bus Services</h1>
        <NavigationBar currentPage={props.currentPage} />
    </div>
    );
}

export default Heading;