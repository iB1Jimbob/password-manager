import { writeFile, FsTextFileOption, readTextFile } from "@tauri-apps/api/fs";
import { message } from "@tauri-apps/api/dialog";

document.querySelector('button').addEventListener('click', () => {
    const file = readTextFile('test.txt');
    console.log(file);
});