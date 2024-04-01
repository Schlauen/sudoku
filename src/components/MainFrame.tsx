import Playfield from './Playfield'
import SolveSidebar from './SolveSidebar'
import Footer from './Footer';
import Header from './Header';
import { AppState, useStore } from '../store';
import EditorSidebar from './EditorSidebar';
import StartSidebar from './StartSidebar';

const renderSidebar = (appState:number) => {
  {
    switch (appState) {
      case AppState.Start:
        return <StartSidebar/>
      case AppState.Solved:
      case AppState.Solving:
        return <SolveSidebar/>
      case AppState.Editing:
        return <EditorSidebar/>
    }   
  }
}

const renderPlayfield = (appState:number) => {
  {
    switch (appState) {
      case AppState.Solving:
      case AppState.Editing:
        return <Playfield/>
    }   
  }
}

const MainFrame = () => {
  const appState = useStore(state => state.appState);

  return (
    <div id='main-frame'>
      <Header/>
      {
        renderSidebar(appState)
      }
      <Playfield/>
      <Footer />
    </div>
  )
}

export default MainFrame