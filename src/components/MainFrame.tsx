import { forwardRef, useImperativeHandle, useRef } from 'react'
import Playfield from './Playfield'
import Sidebar from './Sidebar'
import Footer from './Footer';
import Header from './Header';

interface Props {
  setOpenModal: (open:boolean) => void;
}

const MainFrame = forwardRef(({setOpenModal} : Props, ref) => {
  const playfield = useRef<any>(null);
  const update = () => playfield.current.update();
  
  useImperativeHandle(ref, () => ({update}));

  return (
    <div id='main-frame'>
      <Header/>
      <Sidebar 
        updatePlayfield={update}
        setOpenModal={setOpenModal}
      />
      <Playfield ref={playfield} />
      <Footer />
    </div>
  )
})

export default MainFrame