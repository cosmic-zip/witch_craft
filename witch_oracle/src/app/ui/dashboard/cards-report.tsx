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

export default function CardsReport({ from = ''}) {
    const [data, setData] = useState(Array<CoreResult>);

    console.log(data);

    useEffect(() => {
        invoke<Array<CoreResult>>('select_report', { from: from, size: 9999 })
            .then(result => setData(result))
            .catch(console.error)
    }, []);

    return <div>

        {
            data.map((result) => (
                <>

                    <header className='rounded-t-xl p-4 bg-[#d4d409] text-black'>
                        <h1 className='uppercase font-bold'>⚫ id: {result.id}</h1>
                    </header>
                    <div className='card backdrop-blur-xl mb-4 p-4 rounded-b-xl'>

                        <div className='p-2 mb-2 mx-2'>
                            <h2> <b>session</b>: {result.session}</h2>
                            <h2> <b>session description</b>: {result.session_description}</h2>
                            <h2> <b>source from</b>: {result.source_from}</h2>
                            <h2> <b>source command</b>: {result.source_command}</h2>
                            <h2> <b>source detail</b>: {result.source_detail}</h2>
                            <h2> <b>source description</b>: {result.source_description}</h2>
                            <h2> <b>timestemp</b>: {result.timestemp}</h2>
                            <h2> <b>returned_status</b>: {result.returned_status}</h2>
                            <h2> 
                                <b>debug</b>: {
                                    result.debug === 'true' ? (
                                        <span className='text-green-500 font-bold'>{result.debug}</span>
                                    ) : (
                                        <span className='text-red-500 font-bold'>{result.debug}</span>
                                    )
                                }
                            </h2>
                        </div>

                        <div className='m-2'>

                            <div className='font-mono p-2 m-2 bg-gradient-to-r from-slate-900 to-gray-700 text-indigo-100 rounded-xl'>
                                <h1 className='font-bold'>formatted stdout: </h1>
                                <div>
                                    {
                                        result.formatted_stdout.split('\n').map((item, index) => (
                                            <p key={index}>{item}</p>
                                        ))
                                    }
                                </div>
                            </div>

                            <div className='font-mono p-2 m-2 bg-gradient-to-r from-slate-900 to-gray-700 text-indigo-100 rounded-xl'>
                                <h1 className='font-bold'>formatted stderr: </h1>
                                <div>
                                    {
                                        result.formatted_stderr.split('\n').map((item, index) => (
                                            <p key={index}>{item}</p>
                                        ))
                                    }
                                </div>
                                <br />
                            </div>
                        </div>


                    </div>

                </>
            ))
        }

    </div>;

}
