import {useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/core";
import { DataGrid, GridColDef } from '@mui/x-data-grid';
import {Box, FormControl, InputLabel, NativeSelect, Paper} from "@mui/material";

const columns: GridColDef[] = [
    {
        field: 'leaguePosition',
        headerName: 'Position',
        type: 'number',
        minWidth: 10,
    },
    {
        field: 'teamName',
        headerName: 'Team',
        minWidth: 175,
    },
    {
        field: 'points',
        headerName: 'Points',
        type: 'number',
        minWidth: 8,
    },
];

export default function Standings() {
    const [standings, setStandings] = useState<Team[]>([]);

    type Standings = {
        standings: Team[]
    }
    type Team = {
        teamName: String
        points: number
    }

    useEffect(
        () => {
            invoke<Standings>("get_standings")
                .then((data) => setStandings(data.standings))
        },
    )

    return (
        <Paper sx={{ height: '100%', width: '100%' }}>
            <Box>
                <FormControl className="object-center">
                    <InputLabel variant="standard" htmlFor="uncontrolled-native">
                        Standings
                    </InputLabel>
                    <NativeSelect
                        defaultValue={"League"}
                        inputProps={{
                            name: 'standings',
                            id: 'uncontrolled-native',
                        }}
                    >
                        <option value={"League"}>League</option>
                        <option value={"Conference"}>Conference</option>
                        <option value={"Division"}>Division</option>
                    </NativeSelect>
                </FormControl>
            </Box>
            <DataGrid
                className={"table-auto"}
                rows={standings}
                getRowId={(row) => row.leaguePosition}
                columns={columns}
                hideFooter={true}
                autosizeOptions={{
                    columns: ['Position', 'Team', 'Points'],
                    includeOutliers: true,
                    includeHeaders: true,
                }}
                sx={{ border: 0 }}
            />
        </Paper>
    );
}
