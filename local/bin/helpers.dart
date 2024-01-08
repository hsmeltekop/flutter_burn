import 'dart:io';
import 'package:image/image.dart' as img;

Future<List<double>> imageToRawBytes() async {
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

  List<double> rawBytesOut = [];

  for (final pixel in image) {
    rawBytesOut.add(pixel.r == 0 ? 0 : 255);
  }

  return Future.value(rawBytesOut);
}
