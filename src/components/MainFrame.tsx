import { useRef } from 'react'
import Playfield from './Playfield'
import Sidebar from './Sidebar'
import Footer from './Footer';
import Header from './Header';

const MainFrame = () => {
  const playfield = useRef<any>(null);
  const headerRef = useRef<any>(null);

  const update = () => playfield.current.update();
  const setMessage = (msg:string) => headerRef.current.setMessage(msg);
  const setGameState = (newState:number) => headerRef.current.setGameState(newState);

  return (
    <div id='main-frame'>
      <Header ref={headerRef}/>
      <Sidebar updatePlayfield={update} onError={setMessage} />
      <Playfield ref={playfield} onError={setMessage} setGameState={setGameState}/>
      <Footer />
    </div>
  )
}

export default MainFrame