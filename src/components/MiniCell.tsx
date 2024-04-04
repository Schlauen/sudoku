import { forwardRef, useImperativeHandle, useState } from "react"

interface Props {
    digit:number;
}

const MiniCell = forwardRef(({ digit }: Props, ref) => {
    const [shown, setShown] = useState(false);
    useImperativeHandle(ref, () => ({setShown}));
    return (
        <div className='mini-cell' key={digit}>
            {shown ? digit : ''}
        </div>
    )
});

export default MiniCell;
