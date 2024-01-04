'use client'
import React from 'react';
import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri'
import { string } from 'zod';

export default function EverySection() {
    const [data, setData] = useState('');

    console.log(data);

    useEffect(() => {
        invoke<string>('select_report', { from: 'all', size: 9999 })
            .then(result => setData(result))
            .catch(console.error)
    }, []);

    const datamap = { "data": data }

    return <div>

        {
            datamap["data"].map((result) => (
                <div className='card bg-slate-400 p-2 m-4'>
                    <p>id: {result.id}</p>
                    <p>session: {result.session}</p>
                    <p>session_description: {result.session_description}</p>
                    <p>source_from: {result.source_from}</p>
                    <p>source_command: {result.source_command}</p>
                    <p>source_detail: {result.source_detail}</p>
                    <p>source_description: {result.source_description}</p>
                    <p>timestemp: {result.timestemp}</p>
                    <p>returned_status: {result.returned_status}</p>
                    <p>formatted_stdout: {result.formatted_stdout}</p>
                    <p>formatted_stderr: {result.formatted_stderr}</p>
                    <p>debug: {result.debug}</p>
                </div>
            ))
        }

    </div>;

}
