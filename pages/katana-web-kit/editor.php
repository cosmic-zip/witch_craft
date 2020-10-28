<!DOCTYPE html>
<html>
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width">
  <link rel="stylesheet" type="text/css" href="main.css">
  <title>shell</title>
</head>
<body>
   <header>
     <div id="menu">
       <ul>
         <li id="file">File</li>
          <ul id="fileSub">
            <li id='new'>New</li>
            <li id='save'>Save</li>
            <li id='load'>Load</li>
          </ul>
         <li id="about">About</li>
           <div id="aboutBox">
             <div id="aboutClose">
               <p>X</p>
             </div>
             <h2>Old School Text Editor</h2>
             <p>Version: 1.0</p>
             <p>Made by Lucas Hoffmann</p>
             <p>October, 2017</p>
             <p>Uptaded by Xx_zeref_xX</p>
             <p>June, 2020</p>
           </div>
         <li id="title">linux shell v 2.3 <a href="index.php">Exit</a></li>
       </ul>
     </div>
     
   </header>
   
   <div id="newBox">
     <h2>Are you sure?</h2>
     <p id="newYes">Yes</p>
     <p id="newNo">No</p>
   </div>
  
  <div id="saveBox">
     <div id="saveTitle"><p>Save your file</p></div>
     <input id="saveName" type="text" placeholder = "FILENAME.TXT">
     <input id="saveButton" type="submit" value = "Save">
     <input id="saveClose" type="submit" value = "Close">
     </input>
   </div>
  
  <section class="txt">
    <textarea id="textarea" class='textarea' type='text' placeholder='TYPE HERE' rows='25'>      
    </textarea>
  </section>

  <footer class="footer">
      [exit]      [:set/ to set]      [:w write]     [:q quit]
  </footer>
  <script type="text/javascript" src="scripts/main.js"></script>
</body>
</html>
