<?php
  header('Content-type: application/x-tex; charset=ISO-8859-1');
  $keys = array(
    'categ',
    'reztitle',
    'anzpers',
    'zutaten',
    'zubereitung',
  );
  foreach($keys as $key) {
    if(!array_key_exists($key, $_REQUEST)) {
      $host  = $_SERVER['HTTP_HOST'];
      $uri   = rtrim(dirname($_SERVER['PHP_SELF']), '/\\'); // '
      header("Location: http://$host$uri/errmsg.php?backfile=index.html&missingvar=$key");
      exit;
    }
    ${$key} = str_replace("\r", '', $_REQUEST[$key]);
  }

  function transform_utf8($str, $transform) {
    return utf8_decode(
      strtr(utf8_encode($str), $transform)
    );
  }

  $transform = array(
    ' ' => '_', '.' => '_', '/' => '_'
  );

  $dirname = 'entries/' . transform_utf8($categ, $transform);

  $transform = array(
    ' ' => '_',    '.' => '_',    '/' => '_',
    'Ä' => '_AE_', 'Ö' => '_OE_', 'Ü' => '_UE_',
    'ä' => '_ae_', 'ö' => '_oe_', 'ü' => '_ue_',
    'ß' => '_sz_',
  );
  $filename = transform_utf8($reztitle, $transform) . '.tex';
  unset($transform);

  header('Content-Disposition: attachment; filename="' . $filename . '"');

  $anzpers = (int) $_REQUEST['anzpers'];

  $zutaten = explode("\n", $zutaten);
  $zutatentxt = "";
  foreach($zutaten as $zutat)
    $zutatentxt .= $zutat . "\\\\\n";
  $zutaten = $zutatentxt;
  unset($zutatentxt);

  $zubereitung = explode("\n", $zubereitung);
  $zubereitungtxt = "";
  foreach($zubereitung as $line)
    $zubereitungtxt .= wordwrap($line) . "\n\n";
  $zubereitung = $zubereitungtxt;
  unset($zubereitungtxt);

  $bemerkungen = "";
  if(array_key_exists('bemerkungen', $_REQUEST) && !empty($_REQUEST['bemerkungen']))
    $bemerkungen = "\n\\bigskip\n\\noindent\nBemerkungen:\n\n" . $_REQUEST['bemerkungen'] . "\n";

  $quellen = array();
  if(array_key_exists('quellen', $_REQUEST)) {
    $quellen = explode("\n", $_REQUEST['quellen']);
  }
?>
\documentclass[a5paper,fontsize=9pt,]{scrartcl}
%\usepackage{bookman}
\usepackage[ngerman]{babel}  %deutsche Silbentrennung und Kapitelueberschriften

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%Kodierung der Zeichensaetze auf 8bit                       %
%damit koennen nun auch Umlaute direkt verwendet werden     %
                                                            %
\usepackage[latin1]{inputenc}% bei win evtl. [ansinew]      %
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%stellt auf 8 bitige fonts um, so dass Umlaute nicht mehr   %
%wie in der cm-Schrift aus zwei zeichen zusammengesetzt     %
%werden muessen. Dies ist bei der Silbentrennung von Vorteil%
                                                            %
\usepackage[T1]{fontenc}                                    %
                                                            %
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

%%%%%%%%%%%%%%%%%%%%%% KOMA-Script Modifikationen %%%%%%%%%%%%%%%%%%%%%%%%%%%%
%
%Titel und Ueberschriften werden in der selben Schriftart wie der Text gesetzt:
\setkomafont{sectioning}{\normalfont}

%Die zweite Zeile einer Fusznote wird nicht eingezogen:
%\deffootnote[Markenbreite]{Einzug}{Absatzeinzug}{Markendefinition}
\deffootnote[1em]{0em}{1em}{\textsuperscript{\thefootnotemark}}

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%


%%%%%%%%%%%%%%%%%%%%%% Formatierungsvorgaben %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

% Der Zeilenabstand wird auf das 1,5 fache des Normalabstandes vergroeszert:

%\renewcommand{\baselinestretch}{1,5}

%der als Standart voreingestellte Durchschuss wird mit dem angegebenen Faktor multipliziert
%ist ein durchschuss von 1,2 voreingestellt so ergibt sich im vorliegenden Fall 1,5
%\linespread{1.25}\selectfont %hat den Nachteil, dass auch Fusznoten 1,5 Zeilig gesetzt werden.
\usepackage{multicol}
\usepackage{setspace}
\onehalfspacing
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

\typearea[current]{calc} %berechtnet den Satzspiegel nach ggf. erfolgten AEnderungen in der Praeambel nocheinmal.

%%%%%%%%%%%%%%%%%%%%%%%%Das Dokument beginnt%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
<?php ob_start(); ?>
\begin{document}
\section{<?php echo $reztitle; ?>}
Zutaten <?php echo utf8_decode("für") . ' ' . $anzpers; ?> Personen:


\begin{multicols}{2}
\raggedright
<?php echo $zutaten; ?>
\end{multicols}

\noindent
Zubereitung:

<?php echo $zubereitung . $bemerkungen; ?>


\end{document}
%%% Local Variables:
%%% mode: latex
%%% TeX-master: t
%%% End: 
<?php
  $oldumask = umask(0);
  if(!is_dir($dirname)) mkdir($dirname);
  umask($oldumask);
  echo "%% on server: " . $dirname . '/' . $filename . "\n";
  echo "%% Quellen:\n";
  foreach($quellen as $quelle) echo "%% " . $quelle . "\n";
  file_put_contents($dirname . '/' . $filename, ob_get_contents());
  ob_end_flush();
?>
