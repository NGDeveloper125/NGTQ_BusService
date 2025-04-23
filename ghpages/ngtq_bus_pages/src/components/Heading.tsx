import React from "react";
import NavigationBar from "./NavigationBar"

type HeadingProps = {
    currentPage: string
}

function Heading(props: HeadingProps) {
    return (
    <div>
        <NavigationBar />
    </div>
    );
}

export default Heading;