'use client'
import React from 'react';
import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri'


interface CoreResult {
    id: string;
    session: string;
    session_description: string;
    source_from: string;
    source_command: string;
    source_detail: string;
    source_description: string;
    timestemp: string;
    returned_status: string;
    formatted_stdout: string;
    formatted_stderr: string;
    debug: string;
};

export default function EverySection() {
    const [data, setData] = useState(Array<CoreResult>);

    console.log(data);

    useEffect(() => {
        invoke<Array<CoreResult>>('select_report', { from: 'all', size: 9999 })
            .then(result => setData(result))
            .catch(console.error)
    }, []);

    return <div>

        {
            data.map((result) => (
                <div className='card bg-slate-600 p-2 m-4 rounded-xl'>
                    <p>id: {result.id}</p>
                    <p>session: {result.session}</p>
                    <p>session_description: {result.session_description}</p>
                    <p>source_from: {result.source_from}</p>
                    <p>source_command: {result.source_command}</p>
                    <p>source_detail: {result.source_detail}</p>
                    <p>source_description: {result.source_description}</p>
                    <p>timestemp: {result.timestemp}</p>
                    <p>returned_status: {result.returned_status}</p>

                    <div className='p2 bg-gray-900 text-indigo-100'>
                        <h1>formatted_stdout: </h1>
                        <div>
                            {
                                result.formatted_stdout.split('\n').map((item, index) => (
                                    <p>{item}</p>
                                ))
                            }
                        </div>


                    </div>


                    <p>formatted_stderr: {result.formatted_stderr.split('\n')}</p>
                    <p>debug: {result.debug}</p>
                </div>
            ))
        }

    </div>;

}
