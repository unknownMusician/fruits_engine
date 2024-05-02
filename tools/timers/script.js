var timerInfos;

window.onload = (event) => {
    let scale = document.getElementById("scale");

    scale.oninput = e => {
        let width = Math.pow(10, -scale.value);
        document.getElementById("moments-content").style.width = `${width}px`
    }

    document.getElementById("file-picker").onchange = e => {
        let file = e.target.files[0];
        
        var reader = new FileReader();
        reader.readAsText(file,'UTF-8');

        reader.onload = readerEvent => {
            var content = readerEvent.target.result;

            let json = JSON.parse(content)

            timerInfos = json.timers;

            timerInfos.sort(function(a, b){ 
                return a.duration - b.duration;
            })

            let lineIndices = [];

            for (let i = 0; i < timerInfos.length; i++) {
                const timerInfo = timerInfos[i];

                let lineIndex = 0;

                for (let j = 0; j < i; j++) {
                    const definedTimerInfo = timerInfos[j];
                    
                    if (lineIndices[j] != lineIndex) {
                        continue;
                    }

                    if (intersect(definedTimerInfo, timerInfo)) {
                        lineIndex += 1;
                        j = -1;
                    }
                }

                lineIndices.push(lineIndex);
            }

            let momentsContent = document.getElementById("moments-content");
            momentsContent.innerHTML = "";

            let moments = []

            for (let i = 0; i < timerInfos.length; i++) {
                const timerInfo = timerInfos[i];
                
                moments.push(getMoment(timerInfo.startTime, timerInfo.endTime, timerInfo.name, lineIndices[i], i));
            }

            momentsContent.innerHTML += moments.join("");

            let momentElements = document.getElementsByClassName("moment");

            for (let i = 0; i < momentElements.length; i++) {
                const momentElement = momentElements[i];
                
                momentElement.onmouseenter = e => {
                    let id = momentElement.dataset.momentId;
                    document.getElementById("selected-name").innerHTML = `Name: ${timerInfos[id].name}`;
                    document.getElementById("selected-duration").innerHTML = `Duration: ${timerInfos[id].durationNs} ns`;
                };
                momentElement.onmouseleave = e => {
                    let id = momentElement.dataset.momentId;
                    document.getElementById("selected-name").innerHTML = `Name:`;
                    document.getElementById("selected-duration").innerHTML = `Duration:`;
                };
            }
        }      
    }
};

function intersect(timeInfo1, timeInfo2) {
    let start1 = Math.min(timeInfo1.startTime, timeInfo1.endTime);
    let end1 = Math.max(timeInfo1.startTime, timeInfo1.endTime);
    let start2 = Math.min(timeInfo2.startTime, timeInfo2.endTime);
    let end2 = Math.max(timeInfo2.startTime, timeInfo2.endTime);

    return !(end2 < start1 || end1 < start2);
}

function getMoment(startTime, endTime, name, lineIndex, momentId) {
    let divisor = 1_000;

    startTime /= divisor;
    endTime /= divisor;

    return `
        <div class="moment" data-moment-id="${momentId}" style="left: ${startTime}%; width: ${endTime - startTime}%; top: ${lineIndex * 20}px;">${name}</div>
    `
}

// let timerInfo = {
//     startTime: 0,
//     startOrder: 0,
//     endTime: 0,
//     endOrder: 0,
//     name: "",
//     durationNs: 0
// }
