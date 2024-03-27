import { forwardRef, useImperativeHandle, useState } from "react";


interface Props {
    name: string,
}

const NumberInput = forwardRef(({name: name} : Props, ref) => {
    const [value, setValue] = useState(new Date().getTime());
    
    const getValue = () => value;
    useImperativeHandle(ref, () => ({
        getValue,
    }));

    return (
        <div className='menu-element range-container'>
            <label>{name}:</label>
            <input 
                type="number"
                value={value}
                min={0}
                max={2**64-1}
                className='text-input'
                onChange={(val) => {
                    setValue(Number(val.target.value));
                }}
            >
            </input>
        </div>
    )
});

export default NumberInput
