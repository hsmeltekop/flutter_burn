import 'dart:io';

import 'dart:typed_data';
import 'package:image/image.dart' as img;
import 'package:mnist_burn/exports.dart';

import 'package:path_provider/path_provider.dart';

Future<int> predict() async {
  Directory loadDir = await getApplicationDocumentsDirectory();
  File loadFile = File('${loadDir.path}/number.png');

  final image = img.decodePng(await loadFile.readAsBytes())!;

  List<double> rawBytesOut =
      image.map((pixel) => pixel.r == 0 ? 0.0 : 255.0).toList();

  var dummyRawBytesOut = rawBytesOut.map((b) => b == 0 ? " " : '▇').toList();

  for (var row in Iterable.generate(28)) {
    var line = List.generate(28, (index) => dummyRawBytesOut[row * 28 + index]);
    // ignore: avoid_print
    print(line);
  }

  Float32List bytes = Float32List.fromList(rawBytesOut);

  var prediction = await native.predict(bytes: bytes);

  return Future.value(prediction.toInt());
}
