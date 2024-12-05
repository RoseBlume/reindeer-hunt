import { createSignal, Show, Index, Switch, Match  } from "solid-js";
import { open, save } from '@tauri-apps/plugin-dialog';
import { invoke } from "@tauri-apps/api/core";
// Components
//import StudentList from './StudentList';

// Style
import "./App.css";
const [name1, setName1] = createSignal("");
const [surname1, setSurname1] = createSignal("");
const [homeroom1, setHomeroom1] = createSignal("");
const [win, setWin] = createSignal("")
const [name2, setName2] = createSignal("");
const [surname2, setSurname2] = createSignal("");
const [homeroom2, setHomeroom2] = createSignal("");
const [notes, setNotes] = createSignal("");
const [pendingMatch, setPendingMatch] = createSignal(true);
const [path, setPath] = createSignal("");
const [path1, setPath1] = createSignal("");
const [path2, setPath2] = createSignal("");
const [path3, setPath3] = createSignal("");
const [test, setTest] = createSignal(false);
const [editTimes, setEditTimes] = createSignal(false);
const [coinMessage, setCoinMessage] = createSignal("");
const [saver, setSaver] = createSignal("");
const [selected, setSelected] = createSignal("");
// Can be values main, PlayerEdit, TimesEdit, PlayerAdd
const [viewMode, setViewMode] = createSignal("main");
const [moddedName, setModdedName] = createSignal("");
const [moddedSurname, setModdedSurname] = createSignal("");
const [moddedRoom, setModdedRoom] = createSignal("");
const [undecidedList, setUndecidedList] = createSignal();
const [huntTimes, setHuntTimes] = createSignal([
  {
    start: "",
    end: ""
  },
  {
    start: "",
    end: ""
  },
  {
    start: "",
    end: ""
  }
]);
const [buttonState, setButtonState] = createSignal(1);

function App() {
  // Student Variables
  const [students, setStudents] = createSignal([
    {
      // Students name
      firstName: "",
      lastName: "",
      // The home room number or spare
      room: "",
      rand: 0,
      win: "",
      notes: "",
      pending: true,
      // The name of the opposing student
      pairFirst: "",
      pairLast: "",
      pairRoom: ""
    }
  ]
  );


  const [undecidedList, setUndecidedList] = createSignal();
  const handleStudentClick = (deer) => {
    setSurname1(deer().lastName);
    setName1(deer().firstName);
    setHomeroom1(deer().room);
    setSurname2(deer().pairLast);
    setName2(deer().pairFirst);
    setHomeroom2(deer().pairRoom);
    setPendingMatch(deer().pending);
    setWin(deer().win);
    if (win() === "win") {
      setButtonState(2);
    } else if (win() === "loss") {
      setButtonState(3);
    } else {
      setButtonState(1);
    }
  };
  // Show Variables
  const [coin, setCoin] = createSignal(true);
  function incrementButtonState() {
    setButtonState(prev => {
      const newState = prev + 1;
      return newState > 4 ? 1 : newState;
    });
  }
  async function openTimes() {
    setHuntTimes(await invoke("open_times"));
  }
  openTimes();
  async function saveTimes() {
    setHuntTimes(await invoke("save_times", { contents: huntTimes() }));
  }
  async function addStudent() {
    setStudents(await invoke("add_student", {
      firstName: name1(),
      lastName: surname1(),
      homeRoom: homeroom1(),
      notes: notes(),
      contents: students()
    }));
  }
  async function saveStudent() {
    setStudents(await invoke("update_notes", {
      name: name1(),
      lastName: surname1(),
      homeRoom: homeroom1(),
      newNotes: notes(),
      contents: students()
    }));
    if (buttonState() == 1) {
      resetStatus();
    }
    else if (buttonState() == 2) {
      studentWin();

    }
    else {
      studentLoss();
    }
  }
  async function openDia() {
    const res = await open({
      multiple: false,
      directory: false,
      filters: [{ name: "CSV Files", extensions: ["csv"] }]
    });
    if (res) {
      setPath(res)
    }
  }
  async function removeLost() {
    setStudents(await invoke("remove_lost", {contents: students()}));
  }
  async function studentWin() {
    setStudents(await invoke("win", {
      name: name1(),
      lastName: surname1(),
      homeRoom: homeroom1(),
      content: students()
    }));
  }

  async function studentLoss() {
    setStudents(await invoke("loss", {
      name: name1(),
      lastName: surname1(),
      homeRoom: homeroom1(),
      content: students()
    }));
  }
  async function resetStatus() {
    setStudents(await invoke("reset_status", {
      name: name1(),
      lastName: surname1(),
      homeRoom: homeroom1(),
      contents: students()
    }));
  }
  async function importDia() {
    const res = await open({
      multiple: false,
      directory: false,
      filters: [{ name: "CSV Files", extensions: ["csv"] }]
    });
    if (res) {
      setPath1(res)
    }

    const res2 = "something";
    setStudents(await invoke("import", {
      path: path1()
    }));
  }
  async function sortStudents() {
    setStudents(await invoke("sort_students", {contents: students()}))
  }

  async function removeStudent() {
    setStudents(await invoke("remove_student", {
      name: name1(),
      lastName: surname1(),
      homeRoom: homeroom1(),
      contents: students()
    }));
  }
  async function OpenJSON() {
    const res = await open({
      multiple: false,
      directory: false,
      filters: [{ name: "JSON Files", extensions: ["json"] }]
    });
    if (res) {
      setPath(res)
    }
    setStudents(await invoke('open', {path: path()}))
  }


  async function savehunts() {
    const pathos = await save({
      filters: [
        {
          name: 'JSON Files',
          extensions: ['json'],
        },
      ],
    });
    if (pathos) {
      setPath3(pathos)
    }
    if (path3() != "") {
      await invoke('save', {path: path3(), dataInfo: students()});
    }
  }
  async function nextRound() {
    randomize();
    setStudents(await invoke("remove_lost", {contents: students()}));
    pair();
  }
  async function pair() {
    setStudents(await invoke('pair_students', {contents: students()}));
    sortStudents();
  }
  async function singleToss() {
    setStudents(await invoke('single_toss', {
      contents: students(),
      name1: name1(),
      lastName1: surname1(),
      homeRoom1: homeroom1()
    }));
  }
  async function updateNotes() {
    setStudents(await invoke("update_notes", {
      name: name1(),
      lastName: surname1(),
      homeRoom: homeroom1(),
      newNotes: notes(),
      contents: students()
    }));
  }
  async function randomize() {
    setStudents(await invoke("coin_toss", { contents: students() }));
  }

  async function genpermits() {
    const pathos = await save({
      filters: [
        {
          name: 'PDF Files',
          extensions: ['pdf'],
        },
      ],
    });
    if (pathos) {
      setPath3(pathos)
    }
    if (path3() != "") {
      await invoke("generate_permits", { contents: students(), path: path3(), times: huntTimes() });
    }
  }

  return (
    <main class="container">
      <header>
        <ul id="Top">
          <li class="top">
            <div class="dropdown">
              <button class="dropbtn">Manage</button>
              <div class="dropdown-content">
                <ul class="dropmenu">
                  <li class="droplist" onClick={importDia}>Import Players</li>
                  <li class="droplist" onClick={OpenJSON}>Open</li>
                  <li class="droplist" onClick={() => setViewMode("PlayerAdd")}>Add Player</li>
                  <li class="droplist" onClick={nextRound}>Next Round</li>
                  <li class="droplist" onClick={genpermits}>Create Permits</li>
                  <li class="droplist" onClick={savehunts}>Save Hunts</li>
                  <li class="droplist" onClick={pair}>Pair Students</li>
                  <li class="droplist">Quit</li>
                </ul>
              </div>
            </div>
          </li>
          <li class="top">
            <div class="dropdown">
              <button class="dropbtn">Player</button>
              <div class="dropdown-content">
                <ul class="dropmenu">
                  <li class="droplist" onClick={() => {
                    if (name1() != "") {
                      setViewMode("PlayerEdit")
                    }
                  }}>Edit Player</li>
                  
                  <li class="droplist" onClick={removeStudent}>Remove Player</li>
                </ul>
              </div>
            </div>
          </li>
          <li class="top">
            <div class="dropdown">
              <button class="dropbtn">Tools</button>
              <div class="dropdown-content">
                <ul class="dropmenu">
                  <li class="droplist" onClick={() => setViewMode("TimesEdit")}>Edit Hunt Times</li>
                  <li class="droplist" onClick={randomize}>Randomize Pending Matches</li>
                </ul>
              </div>
            </div>
          </li>
        </ul>
      </header>
      <button id="menu" onClick={openDia}>Open File</button>
      <ul id="Men">
        <li id="studentlist" class="MenArea">
      <table>
        <tbody>
          <tr>
            <td>First Name</td>
            <td>Last Name</td>
            <td>Homeroom</td>
            <td>Status</td>
          </tr>
        <Index each={students().filter(deer => deer.win === "undecided")}>{(deer, i) =>
          <tr class="students" onClick={
            () => {
              setNotes(deer().notes)
              setSurname1(deer().lastName)
              setName1(deer().firstName)
              setHomeroom1(deer().room)
              setSurname2(deer().pairLast)
              setName2(deer().pairFirst)
              setHomeroom2(deer().pairRoom)
              setPendingMatch(deer().pending)
              setWin(deer().win)
              setButtonState(1)
            }
          }>
            <td>{deer().firstName}</td>
            <td>{deer().lastName}</td>
            <td>{deer().room}</td>
            <td>Pending</td>
          </tr>
        }</Index>
        <Index each={students().filter(deer => deer.win === "win")}>{(deer, i) =>
          <tr class="students" onClick={
            () => {
              setNotes(deer().notes)
              setSurname1(deer().lastName)
              setName1(deer().firstName)
              setHomeroom1(deer().room)
              setSurname2(deer().pairLast)
              setName2(deer().pairFirst)
              setHomeroom2(deer().pairRoom)
              setPendingMatch(deer().pending)
              setWin(deer().win)
              setButtonState(2)
            }
          }>
            <td>{deer().firstName}</td>
            <td>{deer().lastName}</td>
            <td>{deer().room}</td>
            <td>Won</td>
          </tr>
        }</Index>
        <Index each={students().filter(deer => deer.win === "loss")}>{(deer, i) =>
          <tr class="students" onClick={
            () => {
              setNotes(deer().notes)
              setSurname1(deer().lastName)
              setName1(deer().firstName)
              setHomeroom1(deer().room)
              setSurname2(deer().pairLast)
              setName2(deer().pairFirst)
              setHomeroom2(deer().pairRoom)
              setPendingMatch(deer().pending)
              setWin(deer().win)
              setButtonState(3)
            }
          }>
            <td>{deer().firstName}</td>
            <td>{deer().lastName}</td>
            <td>{deer().room}</td>
            <td>Lost</td>
          </tr>
        }</Index>
        </tbody>
      </table>
        </li>
         <li id="right" class="MenArea">
          <Switch>
            <Match when={viewMode() === "main"}>
            <h2>Player Info</h2>
          <h3>Name: {name1()} {surname1()}</h3>
          <h3>Homeroom: {homeroom1()}</h3>
          <h3>
          <Switch fallback={<button onClick={incrementButtonState()}>Unknown</button>}>
            <Match when={buttonState() === 1}>
              <div onClick={() => { incrementButtonState(); }}>Result: <div class="save" >Undecided</div></div>
            </Match>
            <Match when={buttonState() === 2}>
              <div onClick={() => { incrementButtonState(); }}>Result: <div class="save" >Won</div></div>
            </Match>
            <Match when={buttonState() === 3}>
              <div onClick={() => { incrementButtonState(); }}>Result: <div class="save" >Lost</div></div>
            </Match>
            </Switch>
            </h3>
          <h2>Playing Against:</h2>
            
          <h3>Name: {name2()} {surname2()}</h3>
          <h3>Homeroom: {homeroom2()}</h3>
          <br />
          <h3>Pending Match: 
            <Show when={pendingMatch()}>True</Show>
            <Show when={!pendingMatch()}>False</Show>
          </h3>
          <h3 class="save" onClick={singleToss}>Coin Toss</h3>
          <h3 class="save" onClick={saveStudent}>Save Player Changes</h3>
            </Match>


          <Match when={viewMode() === "TimesEdit"}>
            <div>
              <h2>Edit Hunt Times</h2>
              <form onSubmit={(e) => {
                e.preventDefault();
                setHuntTimes([
                  { start: e.target.start1.value, end: e.target.end1.value },
                  { start: e.target.start2.value, end: e.target.end2.value },
                  { start: e.target.start3.value, end: e.target.end3.value }
                ]);
                saveTimes();
                setViewMode("main")
              }}>
                <div>
                  <label>Morning:</label>
                  <input type="time" name="start1" value={huntTimes()[0].start} onInput={(e) => setHuntTimes(prev => [{ ...prev[0], start: e.target.value }, prev[1], prev[2]])} />
                  <input type="time" name="end1" value={huntTimes()[0].end} onInput={(e) => setHuntTimes(prev => [{ ...prev[0], end: e.target.value }, prev[1], prev[2]])} />
                </div>
                <div>
                  <label>Lunch:</label>
                  <input type="time" name="start2" value={huntTimes()[1].start} onInput={(e) => setHuntTimes(prev => [prev[0], { ...prev[1], start: e.target.value }, prev[2]])} />
                  <input type="time" name="end2" value={huntTimes()[1].end} onInput={(e) => setHuntTimes(prev => [prev[0], { ...prev[1], end: e.target.value }, prev[2]])} />
                </div>
                <div>
                  <label>Evening:</label>
                  <input type="time" name="start3" value={huntTimes()[2].start} onInput={(e) => setHuntTimes(prev => [prev[0], prev[1], { ...prev[2], start: e.target.value }])} />
                  <input type="time" name="end3" value={huntTimes()[2].end} onInput={(e) => setHuntTimes(prev => [prev[0], prev[1], { ...prev[2], end: e.target.value }])} />
                </div>
                <button type="submit">Save Hunt Times</button>
              </form>
              <button onClick={() => setViewMode("main")}>Cancel</button>
            </div>
          </Match>
          <Match when={viewMode() === "PlayerEdit"}>
            <h2>Edit Player</h2>
          
          <div>
            <form onSubmit={(e) => {
              e.preventDefault();
              const updatedStudents = students().map(student => {
                if (student.firstName === name1() && student.lastName === surname1() && student.room === homeroom1()) {
                  return {
                    ...student,
                    firstName: moddedName() || student.firstName,
                    lastName: moddedSurname() || student.lastName,
                    room: moddedRoom() || student.room
                  };
                }
                return student;
              });
              setStudents(updatedStudents);
              setViewMode("main");
            }}>
              <div>
                <label>First Name:</label>
                <input type="text" value={moddedName()} onInput={(e) => setModdedName(e.target.value)} />
              </div>
              <div>
                <label>Last Name:</label>
                <input type="text" value={moddedSurname()} onInput={(e) => setModdedSurname(e.target.value)} />
              </div>
              <div>
                <label>Homeroom:</label>
                <input type="text" value={moddedRoom()} onInput={(e) => setModdedRoom(e.target.value)} />
              </div>
              <button type="submit">Save Changes</button>
            </form>
            <button onClick={() => setViewMode("main")}>Cancel</button>
          </div>
          </Match>
          <Match when={viewMode() === "PlayerAdd"}>
            <h2>Add New Player</h2>
            <div>
              <form onSubmit={(e) => {
                e.preventDefault();
                addStudent();
                setViewMode("main");
              }}>
                <div>
                  <label>First Name:</label>
                  <input type="text" value={name1()} onInput={(e) => setName1(e.target.value)} />
                </div>
                <div>
                  <label>Last Name:</label>
                  <input type="text" value={surname1()} onInput={(e) => setSurname1(e.target.value)} />
                </div>
                <div>
                  <label>Homeroom:</label>
                  <input type="text" value={homeroom1()} onInput={(e) => setHomeroom1(e.target.value)} />
                </div>
                <div>
                  <label>Notes:</label>
                  <textarea value={notes()} onInput={(e) => setNotes(e.target.value)} />
                </div>
                <button type="submit">Add Player</button>
              </form>
              <button onClick={() => setViewMode("main")}>Cancel</button>
            </div>
          </Match>
          </Switch>
          
          </li>
          <li id="bott" class="MenArea">
        <textarea
          value={notes()}
          onInput={(e) => setNotes(e.target.value)}
          placeholder="Enter notes here..."
          id="noteArea"
        />
      </li>
      </ul>
    </main>
  );
}

export default App;
