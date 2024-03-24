import { useRef } from 'react'
import Playfield from './Playfield'
import Sidebar from './Sidebar'
import Footer from './Footer';
import Header from './Header';

const MainFrame = () => {
  const playfield = useRef<any>(null);
  const update = () => playfield.current.update();
  
  return (
    <div id='main-frame'>
      <Header/>
      <Sidebar updatePlayfield={update} />
      <Playfield ref={playfield} />
      <Footer />
    </div>
  )
}

export default MainFrame