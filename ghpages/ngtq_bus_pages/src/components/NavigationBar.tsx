import React from "react";
import NavigationItem from "./NavigationItem";
import '../style/NavigationBar.css';


type NavigationBarProp = {
    currentPage: string
}

function NavigationBar(props: NavigationBarProp) {
    return (
    <div className="navigation-bar">
        <NavigationItem pageName="About" />
        <NavigationItem pageName="NGTQ" />
        <NavigationItem pageName="Bus-Services" />
        <NavigationItem pageName="Forms" />
        <NavigationItem pageName="FAQ" />
        <NavigationItem pageName="Project" />
    </div>
    );
}

export default NavigationBar
