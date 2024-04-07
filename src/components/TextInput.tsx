import { forwardRef, useImperativeHandle, useState } from "react";


interface Props {
    name: string,
}

const TextInput = forwardRef(({name: name} : Props, ref) => {
    const [value, setValue] = useState(new Date().toLocaleDateString());
    
    const getValue = () => value;
    useImperativeHandle(ref, () => ({
        getValue,
    }));

    return (
        <div className='menu-element range-container'>
            <label>{name}:</label>
            <input 
                type="text"
                className='text-input'
                value={value}
                onChange={(val) => setValue(val.target.value)}
            >
            </input>
        </div>
    )
});

export default TextInput
