<?php
  $errmsg = "unbekannt";
  if(array_key_exists('missingvar', $_REQUEST)) $errmsg = "Formularelement <b>'" . $_REQUEST['missingvar'] . "'</b> nicht ausgef&uuml;llt";
  if(array_key_exists('backfile', $_REQUEST)) $backfile = $_REQUEST['backfile'];
?>
<html>
  <head>
    <title>Formularfehler</title>
  </head>
  <body>
    <h1>Formularfehler</h1>
    <p>Es ist folgender Fehler aufgetreten: <br/>
    <?php
      echo $errmsg . '<br />';
      if(isset($backfile)) echo '<a href="' . $backfile . '">Zur&uuml;ck</a>';
      echo "\n";
    ?>
    </p>
  <body>
</html>
