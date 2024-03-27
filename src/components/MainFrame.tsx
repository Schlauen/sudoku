import Playfield from './Playfield'
import Sidebar from './Sidebar'
import Footer from './Footer';
import Header from './Header';

interface Props {
  setOpenLoadingModal: (open:boolean) => void;
  setOpenGenerateModal: (open:boolean) => void;
  setOpenSaveModal: (open:boolean) => void;
}

const MainFrame = ({
  setOpenLoadingModal: setOpenLoadingModal,
  setOpenGenerateModal: setOpenGenerateModal,
  setOpenSaveModal: setOpenSaveModal,
} : Props) => {
  
  return (
    <div id='main-frame'>
      <Header/>
      <Sidebar
        setOpenLoadingModal={setOpenLoadingModal}
        setOpenGenerateModal={setOpenGenerateModal}
        setOpenSaveModal={setOpenSaveModal}
      />
      <Playfield/>
      <Footer />
    </div>
  )
}

export default MainFrame