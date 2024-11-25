import {useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/core";
import {Paper} from "@mui/material";

export default function Schedule() {
    const [schedule, setSchedule] = useState<Team[]>([]);

    type Schedule = {
        standings: Team[]
    }
    type Team = {
        teamName: String
        points: number
    }

    useEffect(
        () => {
            invoke<Schedule>("get_schedule", )
                .then((data) => setSchedule(data.standings))
        },
    )

    return (
        <Paper sx={{ height: '100%', width: '100%' }}>

        </Paper>
    );
}
