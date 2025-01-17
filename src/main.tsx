import React from "react";
import ReactDOM from "react-dom/client";
import { createBrowserRouter, RouterProvider } from "react-router-dom";
import Layout from "./layout";
import ErrorPage from "./error-page";
import Settings from "./routes/settings";
import { TauriProvider } from "./context/TauriProvider";
import "./styles.css";
import { SettingsProvider } from "./context/SettingsProvider";
import Standings from "./routes/standings.tsx";
import Schedule from "./routes/schedule.tsx";

const router = createBrowserRouter([
    {
        path: "/",
        element: <Layout />,
        errorElement: <ErrorPage />,
        children: [
            {
                index: true,
                element: <Standings />,
            },
            {
                path: "/schedule",
                element: <Schedule />
            },
            {
                path: "/settings",
                element: <Settings />,
            },
        ],
    },
]);

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <React.StrictMode>
        <TauriProvider>
            <SettingsProvider>
                <RouterProvider router={router} />
            </SettingsProvider>
        </TauriProvider>
    </React.StrictMode>
);