import { forwardRef, useImperativeHandle, useState } from 'react'

interface Props {
    name: string;
    onStateToggle: () => void; 
}

const Checkbox = forwardRef(({name, onStateToggle} : Props, ref) => {
    const [active, setActive] = useState(false);
    useImperativeHandle(ref, () => ({setActive}));

    return (
        <div className='menu-element checkbox-container'>
            <label>{name}</label>
            <div 
                className='custom-checkbox'
                onClick={onStateToggle}
            >
                {active ? <div className='tick'></div> : undefined}
            </div>
        </div>
    )
});

export default Checkbox
