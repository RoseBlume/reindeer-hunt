import { Index } from 'solid-js';

function StudentList() { ({ students, winStatus, statusLabel, onClick }) => {
  return (
    <Index each={students().filter(deer => deer.win === winStatus)}>
      {(deer, i) => (
        <tr class="students" onClick={() => onClick(deer)}>
          <td>{deer().firstName}</td>
          <td>{deer().lastName}</td>
          <td>{deer().room}</td>
          <td>{statusLabel}</td>
        </tr>
      )}
    </Index>
  );
}};

export default StudentList;