<?php
$fp = fsockopen('127.0.0.1', 1337, $errno, $errstr, 30);
if (!$fp) {
  throw new \Exception($errstr, $errno);
}

function send(string $command): string {
  global $fp;
  fwrite($fp, $command);
  return fgets($fp, 256);
}

function has(string $item): bool {
  $resp = send("HAS $item");
  if ($resp === "Yes.\n") {
    return true;
  }
  if ($resp === "No.\n") {
    return false;
  }
  throw new \Exception('Server responded with '.$resp);
}

function add(string $item): void {
  $resp = send("ADD $item");
  if ($resp !== "OK.\n") {
    throw new \Exception('Server responded with '.$resp);
  }
}

var_dump(has('a'));
add('a');
var_dump(has('a'));

fclose($fp);
