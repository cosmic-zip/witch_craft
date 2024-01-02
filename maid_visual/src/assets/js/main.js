function toggle(id) {

    const boxes = document.getElementsByClassName('toggle');

    for (const box of boxes) {
        box.style.display = 'none';
    }

    var x = document.getElementById(id);
    if (x.style.display === "none") {
        x.style.display = "block";
    } else {
        x.style.display = "none";
    }
}

const invoke = window.__TAURI__.invoke

function insertDataIntoTable(data, tableId, selectedIndices) {
    const table = document.getElementById(tableId);

    table.innerHTML = `
        <thead>
            <tr>
                <th>id</th>
                <th>session</th>
                <th>source</th>
                <th>command</th>
                <th>timestemp</th>
                <th>status</th>
                <th>output</th>
                <th>error</th>
                <th>debug</th>
            </tr>
        </thead>
        <tbody>
        </tbody>
    `;

    if (!table) {
        console.error(`Table with ID "${tableId}" not found.`);
        return;
    }

    // Clear the existing table rows except for the header row.
    while (table.rows.length > 1) {
        table.deleteRow(1);
    }

    // Iterate through the data and insert rows into the table.
    for (const rowData of data) {
        const row = table.insertRow();
        for (const index of selectedIndices) {
            const cell = row.insertCell();
            ctn = rowData[index] || '' || [];
            cell.textContent = ctn;
        }
    }
}

function exec_write_table(data, tag_id) {
    console.log(data, tag_id);
    const selectedIndices = [0, 1, 3, 4, 7, 8, 9, 10, 11];
    // const selectedIndices = [0, 1, 3, 4, 7, 8];
    insertDataIntoTable(data, tag_id, selectedIndices);
}

invoke('select_report', { from: 'all', size: 100 }).then((data) => exec_write_table(data, "logsTable"));
invoke('select_report', { from: 'attack', size: 100 }).then((data) => exec_write_table(data, "logsAttack"));
invoke('select_report', { from: 'bootnet', size: 100 }).then((data) => exec_write_table(data, "logsBotnet"));
invoke('select_report', { from: 'bcurl', size: 100 }).then((data) => exec_write_table(data, "logsBcurl"));
invoke('select_report', { from: 'lookup', size: 100 }).then((data) => exec_write_table(data, "logsLookup"));
invoke('select_report', { from: 'utils', size: 100 }).then((data) => exec_write_table(data, "logsAv"));
invoke('select_report', { from: 'maid_ce', size: 100 }).then((data) => exec_write_table(data, "logsCe"));
invoke('select_report', { from: 'rootkit', size: 100 }).then((data) => exec_write_table(data, "logsRookit"));
invoke('select_report', { from: 'scanner', size: 100 }).then((data) => exec_write_table(data, "logsScanner"));
invoke('select_report', { from: 'osint', size: 100 }).then((data) => exec_write_table(data, "logsOsint"));

