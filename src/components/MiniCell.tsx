import { forwardRef, useImperativeHandle, useState } from "react"

interface Props {
    digit:number;
}

const MiniCell = forwardRef(({ digit }: Props, ref) => {
    const [shown, setShown] = useState(false);

    const toggle = () => setShown(!shown);

    useImperativeHandle(ref, () => ({toggle}));
    return (
        <div className='mini-cell' key={digit}>
            {shown ? digit : ''}
        </div>
    )
});

export default MiniCell;
