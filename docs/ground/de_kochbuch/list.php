<!doctype html>
<?php
  echo "<?xml version=\"1.0\" encoding=\"ISO-8859-1\" ?>\n";
  $folders = scandir('entries/');
  function fdn2cat($fdn) {
    return ucfirst(substr($fdn, 3));
  }
?>
<html>
  <head>
    <title>Kochbucheintr&auml;ge</title>
  </head>
  <body>
    <a href="index.html">Zum Rezeptformular</a>
    <h1>Kochbucheintr&auml;ge</h1>
    <table>
      <thead>
        <tr>
          <th>Kategorie</th>
          <th>Eintr&auml;ge</th>
        </tr>
      </thead>
      <tbody>
<?php
  foreach($folders as $folder) {
    // skip hidden folders
    if($folder == '.' || $folder == '..') continue;
    echo "        <tr>\n";
    echo "          <td>" . fdn2cat($folder) . "</td>\n";
    echo "          <td>\n";
    $entries = scandir('entries/' . $folder . '/');
    foreach($entries as $entry) {
      if($entry == '.' || $entry == '..') continue;
      $name = ucfirst(str_replace('.tex', '', $entry));
      $transform = array(
        '_AE_' => '&Auml;',
        '_OE_' => '&Ouml;',
        '_UE_' => '&Uuml;',
        '_ae_' => '&auml;',
        '_oe_' => '&ouml;',
        '_ue_' => '&uuml;',
        '_sz_' => '&szlig;',
      );
      echo "            <a href='entries/" . $folder . '/' . $entry . "'>" . strtr($name, $transform) . "</a><br />";
    }
    echo "          </td>\n";
    echo "        </tr>\n";
  }
?>
      </tbody>
    </table>
  </body>
</html>
