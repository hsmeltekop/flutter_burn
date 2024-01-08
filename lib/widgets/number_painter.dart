import 'dart:io';
import 'dart:typed_data';
import 'dart:ui' as ui;

import 'package:flutter/material.dart';
import 'package:path_provider/path_provider.dart';

class NumberPainter extends CustomPainter {
  final List<List<Offset>> positions;
  bool mnistSize;

  NumberPainter(this.positions, [this.mnistSize = false]);

  @override
  void paint(Canvas canvas, Size size) {
    canvas.save();

    canvas.drawRect(Rect.fromLTWH(0, 0, size.width, size.height),
        Paint()..color = Colors.black);

    final paint = Paint()
      ..color = Colors.white
      ..strokeWidth = mnistSize ? 2.0 : 10.0;

    double d = mnistSize ? 10.0 : 1.0;

    for (var p in positions) {
      if (p.length >= 2) {
        var ix = 0;
        while (ix < p.length - 1) {
          canvas.drawLine(p[ix] / d, p[ix + 1] / d, paint);
          ix += 1;
        }
      }
    }
    canvas.restore();
  }

  @override
  bool shouldRepaint(covariant CustomPainter oldDelegate) {
    return true;
  }

  Future<Uint8List> saveAsImage() async {
    mnistSize = true;

    ui.PictureRecorder recorder = ui.PictureRecorder();
    Canvas canvas = Canvas(recorder);

    var size = const Size(28.0, 28.0);

    paint(canvas, size);

    ui.Image renderedImage = await recorder.endRecording().toImage(28, 28);
    var pngBytes =
        await renderedImage.toByteData(format: ui.ImageByteFormat.png);

    Directory saveDir = await getApplicationDocumentsDirectory();
    File saveFile = File('${saveDir.path}/number.png');

    if (!saveFile.existsSync()) {
      saveFile.createSync(recursive: true);
    }
    saveFile.writeAsBytesSync(pngBytes!.buffer.asUint8List(), flush: true);

    mnistSize = false;

    return Future.value(pngBytes.buffer.asUint8List());
  }
}
