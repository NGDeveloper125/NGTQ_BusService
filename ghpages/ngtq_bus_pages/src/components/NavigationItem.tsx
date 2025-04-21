import React from "react";
import NavigationBar from "./NavigationBar";
import "../style/NavigationItem.css";

type NavigationItemProps = {
    pageName: string
}

function NavigationItem(props: NavigationItemProps) {
    return (
    <div className="navigation-item">
        <p>{props.pageName}</p>
    </div>
    );
}

export default NavigationItem;