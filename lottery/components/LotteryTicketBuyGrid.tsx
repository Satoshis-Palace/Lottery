import React, { useState } from 'react';

const GridComponent: React.FC = () => {
    const [rowCount, setRowCount] = useState<number>(3);
    const handleInputChange = (event: React.ChangeEvent<HTMLInputElement>) => {
        const value = parseInt(event.target.value, 10);
        setRowCount(isNaN(value) ? 3 : value); // Default to 3 if NaN
    };

    const generateGrid = () => {
        const grid = [];
        for (let i = 0; i < rowCount; i++) {
            const row = [];
            for (let j = 0; j < 3; j++) {
                row.push(<div key={j}>Row {i + 1}, Col {j + 1}</div>);
            }
            grid.push(<div key={i} className="grid-row">{row}</div>);
        }
        return grid;
    };

    return (
        <div>
            <label>
                Number of Rows:
                <input type="number" value={rowCount} onChange={handleInputChange} />
            </label>

            <div className="scrollable-container">
                <div className="grid-container">
                    {generateGrid()}
                </div>
            </div>
        </div>
    );
}

export default GridComponent;