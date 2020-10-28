const file = document.querySelector('#file');
let fileSwitch = 0;
const fileSub = document.querySelector('#fileSub')
const newOpen = document.querySelector('#new');
const newBox = document.querySelector('#newBox');
const newYes = document.querySelector('#newYes');
const newNo = document.querySelector('#newNo');
const save = document.querySelector("#save");
const saveBox = document.querySelector('#saveBox');
const saveName = document.querySelector('#saveName');
const saveButton = document.querySelector('#saveButton');
const saveClose = document.querySelector('#saveClose');
const about = document.querySelector('#about');
const aboutBox = document.querySelector('#aboutBox');
const aboutClose = document.querySelector('#aboutClose p');
const textArea = document.querySelector('#textArea');

//SAVE FUNCTION \/
function download(filename, text) {
    var pom = document.createElement('a');
    pom.setAttribute('href', 'data:text/plain;charset=utf-8,' + encodeURIComponent(text));
    pom.setAttribute('download', filename);

    if (document.createEvent) {
        var event = document.createEvent('MouseEvents');
        event.initEvent('click', true, true);
        pom.dispatchEvent(event);
    }
    else {
        pom.click();
    }
}

//SAVE FUNCTION /\

file.addEventListener('click', function() {
// OLD VERSION, DOUBLE CLICK BUG
//   if (fileSub.style.display == 'none') {
//     fileSub.style.display = 'block';
//   }  else {
//     fileSub.style.display = 'none';
//   }
   if (fileSwitch == 0) {
     fileSub.style.display = 'block';
     fileSwitch++;
   } else {
     fileSub.style.display = 'none';
     fileSwitch--;
   }

 
});


newOpen.addEventListener('click', function() {
  newBox.style.display = 'block';
  fileSub.style.display = 'none';
   
 })

newYes.addEventListener('click', function() {
  textArea.value = '';
  newBox.style.display = 'none';
  fileSub.style.display = 'none';
})
 
newNo.addEventListener('click', function() {
  newBox.style.display = 'none';
  fileSub.style.display = 'none';
})

save.addEventListener('click', function () {
  saveBox.style.display = 'block';
  fileSub.style.display = 'none';
})

saveButton.addEventListener('click', function() {
  if (saveName.value === '') {
    saveName.value += 'text.txt';
  } else if (!saveName.value.includes('.txt')) {
    saveName.value += '.txt';
  }  
  download(saveName.value, textArea.value)
})

saveClose.addEventListener('click', function() {
  saveBox.style.display = 'none';
  fileSub.style.display = 'none';
})

about.addEventListener('click', function() {
  aboutBox.style.display = 'block';
  fileSub.style.display = 'none';
});

aboutClose.addEventListener('click', function() {
    aboutBox.style.display = 'none';
    fileSub.style.display = 'none';
});


