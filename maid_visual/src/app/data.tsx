'use client'
import React from 'react';
import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri'

export default function Data() {
    const [data, setData] = useState('');

    console.log(data);

    useEffect(() => {
        invoke<string>('select_report', { from: 'scanner', size: 100 })
            .then(result => setData(result))
            .catch(console.error)
    }, [])

    // Necessary because we will have to use Greet as a component later.
    return <div>{`Object: ${JSON.stringify(data)}`}</div>;
}

// invoke('select_report', { from: 'all', size: 100 }).then((data) => exec_write_table(data, "logsTable"));
// invoke('select_report', { from: 'antivirus', size: 100 }).then((data) => exec_write_table(data, "logsAv"));
// invoke('select_report', { from: 'attack', size: 100 }).then((data) => exec_write_table(data, "logsAttack"));
// invoke('select_report', { from: 'botnet', size: 100 }).then((data) => exec_write_table(data, "logsBotnet"));
// invoke('select_report', { from: 'bcurl', size: 100 }).then((data) => exec_write_table(data, "logsBcurl"));
// invoke('select_report', { from: 'lookup', size: 100 }).then((data) => exec_write_table(data, "logsLookup"));
// invoke('select_report', { from: 'utils', size: 100 }).then((data) => exec_write_table(data, "logsAv"));
// invoke('select_report', { from: 'pods', size: 100 }).then((data) => exec_write_table(data, "logsPods"));
// invoke('select_report', { from: 'rootkit', size: 100 }).then((data) => exec_write_table(data, "logsRookit"));
// invoke('select_report', { from: 'scanner', size: 100 }).then((data) => exec_write_table(data, "logsScanner"));
// invoke('select_report', { from: 'osint', size: 100 }).then((data) => exec_write_table(data, "logsOsint"));