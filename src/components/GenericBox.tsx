import { ReactNode } from "react";

interface Props {
    parentRow: number;
    parentCol: number;
    childFactory: (row:number, col:number) => ReactNode;
    keyPrefix: string;
}

const GenericBox = ({parentRow, parentCol, childFactory, keyPrefix}: Props) => {
    return (
        <div 
            className='box'
            id={keyPrefix + parentRow + "," + parentCol}
            key={keyPrefix + parentRow + "," + parentCol}
        >
            {Array(9).fill(undefined).map((_, i) => {
                let row = 3 * parentRow + Math.floor(i / 3);
                let col = 3 * parentCol + (i % 3);
                return childFactory(row, col);
            })}
        </div>
    )
}

export default GenericBox
