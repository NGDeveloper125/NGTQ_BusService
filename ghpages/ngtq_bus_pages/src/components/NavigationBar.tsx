import React from "react";
import '../style/NavigationBar.css';

type BarItemProps = {
    address: string, 
    title: string
}

const BarItem = (props: BarItemProps) => {
    return (
        <div className="bar-item"><a href={props.address}>{props.title}</a></div>
    );
}

const NavigationBar = () => {
    return (
        <nav className ="navigation-bar">
            <div className="navigation-bar-left">
                <h1>NGTaskQueue</h1>
            </div>
            <div className="navigation-bar-center">
                <BarItem address="/about" title="About" />
                <BarItem address="/ngtq" title="NGTQ" />
                <BarItem address="/busserviecs" title="Bus Services" />
                <BarItem address="/project" title="Project" />
                <BarItem address="/community" title="Community" />
            </div>
        </nav>
    );
};

export default NavigationBar
