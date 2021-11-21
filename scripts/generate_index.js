const fs = require('fs');
const path = require('path');

function generate() {
    const basePath = path.join(__dirname, '..');
    const readmePath = path.join(basePath, 'README.md');
    const readmeContents = fs.readFileSync(readmePath, { encoding: 'utf-8' });
    const contents = [];
    const years = fs.readdirSync(basePath).filter((name) => /^\d+$/.test(name));
    years.sort((a, b) => +b - +a);
    for (const year of years) {
        contents.push(`<details><summary>${year}</summary>\n<p>\n`);
        const days = fs
            .readdirSync(path.join(basePath, year))
            .filter((name) => /^day\d+$/.test(name))
            .map((dayFolder) => ({ dayFolder, day: parseInt(dayFolder.slice(3)) }));
        days.sort((a, b) => a.day - b.day);
        for (const { dayFolder, day } of days) {
            contents.push(` - [Day ${day}](./${year}/${dayFolder}/src/main.rs)`);
        }
        contents.push(`\n</p>\n</details>`);
    }
    const newContents = readmeContents.replace(
        /<!-- INDEX-START -->.*?<!-- INDEX-END -->/s,
        `<!-- INDEX-START -->\n${contents.join('\n')}\n<!-- INDEX-END -->`
    );
    fs.writeFileSync(readmePath, newContents);
}

generate();
