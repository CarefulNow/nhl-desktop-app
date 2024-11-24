import {useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/core";
import {Paper} from "@mui/material";

export default function Standings() {
    const [schedule, setSchedule] = useState<Team[]>([]);

    type Standings = {
        standings: Team[]
    }
    type Team = {
        teamName: String
        points: number
    }

    useEffect(
        () => {
            invoke<Standings>("get_schedule", )
                .then((data) => setSchedule(data.standings))
        },
    )

    return (
        <Paper sx={{ height: '100%', width: '100%' }}>

        </Paper>
    );
}
