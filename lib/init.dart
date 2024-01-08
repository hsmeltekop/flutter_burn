import 'dart:ffi';
import 'dart:io';
import 'package:flutter/services.dart';
import 'package:logger/logger.dart';
import 'package:mnist_burn/gen_api.dart';
import 'package:path/path.dart';
import 'package:path_provider/path_provider.dart';

late final Logger logger;

late apiClassImpl native;

Future<bool> init() async {
  /* load logger */
  logger = Logger(
    printer: PrettyPrinter(
        methodCount: 2, // Number of method calls to be displayed
        errorMethodCount: 8, // Number of method calls if stacktrace is provided
        lineLength: 120, // Width of the output
        colors: true, // Colorful log messages
        printEmojis: true, // Print an emoji for each log message
        printTime: false // Should each log print contain a timestamp
        ),
  );

  /* copy MNIST model from assets- to document-directory so rustcode can load model */
  Directory directory = await getApplicationDocumentsDirectory();
  var dbPath = join(directory.path, "model.mpk.gz");
  logger.i(dbPath);
  ByteData data = await rootBundle.load("assets/model.mpk.gz");
  List<int> bytes =
      data.buffer.asUint8List(data.offsetInBytes, data.lengthInBytes);
  await File(dbPath).writeAsBytes(bytes);

  native = apiClassImpl(DynamicLibrary.open("libnative.so"));

  await native.initPredictor(modelPath: directory.path);

  logger.i(native.toString());

  return Future.value(true);
}
