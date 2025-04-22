import React from "react";
import "../style/PageBody.css";
import "../pages/AboutPage"
import AboutPage from "../pages/AboutPage";

type PageBodyProp = {
    currentPage: string
}

function PageBody(props: PageBodyProp) {
    return (
    <div className="page-body">
        {GetPageContent(props.currentPage)}
    </div>
    );
}

function GetPageContent(currentPage: string) {
    return (
        (() => {
            switch (currentPage) {
            case "About":
                return <AboutPage />;
            // Add more cases here for other pages
            default:
                return null;
            }
        })()
    );
}

export default PageBody;