'use client'
import React from 'react';
import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri'

export default function EverySection(){
    const [data, setData] = useState('');

    // console.log(data);

    useEffect(() => {
        invoke<string>('select_report', { from: 'scanner', size: 9999 })
            .then(result => setData(result))
            .catch(console.error)
    }, [])

    return <div>{`Object: ${JSON.stringify(data)}`}</div>;

}
