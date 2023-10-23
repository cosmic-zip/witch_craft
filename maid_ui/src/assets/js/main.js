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
            ctn = rowData[index] || '';
            cell.textContent = ctn;

        }
    }
}

function exec_write_table(data, tag_id) {
    console.log(data, tag_id);
    const selectedIndices = [0, 1, 3, 4, 7, 8, 10, 11];
    insertDataIntoTable(data, tag_id, selectedIndices);
}

invoke('select_report', { from: 'all' }).then((data) => exec_write_table(data, "logsTable"));

