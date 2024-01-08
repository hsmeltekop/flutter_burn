import 'dart:core';

import 'dart:io';

import 'package:image/image.dart' as img;

void main() async {
  var paths = [
    "./number.png",
    "../../number.png",
    "../number.png",
    "./local/number.png"
  ];
  File? file;
  for (var path in paths) {
    file = File(path);
    if (await file.exists()) {
      break;
    }
  }

  var sw = Stopwatch();
  sw.start();
  final image = img.decodePng(await file!.readAsBytes())!;

  var rawBytesOut = [];

  for (final pixel in image) {
    rawBytesOut.add(pixel.r == 0 ? " " : '▇');
  }

  print(rawBytesOut.length);

  for (var row in Iterable.generate(28)) {
    var r = List.generate(28, (index) => rawBytesOut[row * 28 + index]);
    print(r);
  }
}
