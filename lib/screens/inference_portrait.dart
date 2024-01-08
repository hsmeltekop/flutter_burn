import 'dart:typed_data';

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_styled_toast/flutter_styled_toast.dart';
import 'package:mnist_burn/exports.dart';

class InferencePortrait extends StatelessWidget {
  const InferencePortrait({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
          elevation: 8,
          shadowColor: Colors.black,
          flexibleSpace: Container(
            decoration: const BoxDecoration(
              gradient: LinearGradient(
                colors: [
                  Color.fromARGB(255, 169, 169, 169),
                  Color.fromARGB(255, 211, 211, 211)
                ],
              ),
            ),
          ),
          title: const Text('Rust BURN With Flutter: MNIST',
              style: TextStyle(color: Colors.white)),
          leading: Image.asset(
            "assets/icon.png",
          )),
      body: Container(
        decoration: const BoxDecoration(
          gradient: LinearGradient(
            begin: Alignment.topRight,
            end: Alignment.bottomLeft,
            stops: [0.5, 1.0],
            colors: [
              Color.fromARGB(255, 169, 169, 169),
              Color.fromARGB(255, 211, 211, 211)
            ],
          ),
        ),
        child: Center(
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            crossAxisAlignment: CrossAxisAlignment.center,
            children: [
              const SizedBox(
                height: 20,
              ),
              Center(child: Consumer(builder: (context, ref, child) {
                final position = ref.watch(positionProvider);
                return GestureDetector(
                  onPanStart: (DragStartDetails details) {
                    ref
                        .read(positionProvider.notifier)
                        .update((state) => [...state, []]);
                  },
                  onPanUpdate: (DragUpdateDetails details) {
                    logger.i(
                        "${details.globalPosition.toString()} | ${details.localPosition.toString()}");
                    if (details.localPosition.dx >= 0 &&
                        details.localPosition.dy >= 0) {
                      ref.read(positionProvider.notifier).update((state) {
                        state.last.add(details.localPosition);
                        return [...state];
                      });
                    }
                  },
                  child: Card(
                    elevation: 20,
                    shadowColor: const Color(0xFF2F4F4F),
                    child: ClipRRect(
                      borderRadius:
                          const BorderRadius.all(Radius.circular(12.0)),
                      child: CustomPaint(
                        size: const Size(280.0, 280.0),
                        painter: NumberPainter(position),
                      ),
                    ),
                  ),
                );
              })),
              const SizedBox(
                height: 20,
              ),
              Consumer(builder: (context, ref, child) {
                final prediction = ref.watch(predictionProvider);
                final Uint8List? image = ref.watch(imageProvider);
                return Row(
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: [
                    ElevatedButton(
                        onPressed: () async {
                          if (ref
                              .read(positionProvider.notifier)
                              .state
                              .isEmpty) {
                            showToast("Draw some number first!",
                                context: context);
                            return;
                          }

                          var image = await NumberPainter(
                                  ref.read(positionProvider.notifier).state)
                              .saveAsImage();
                          ref
                              .read(imageProvider.notifier)
                              .update((state) => image);
                          var prediction = await predict();
                          ref
                              .read(predictionProvider.notifier)
                              .update((state) => prediction);
                        },
                        child: const Text(
                          "Predict",
                          style: TextStyle(color: Colors.black),
                        )),
                    Padding(
                        padding: const EdgeInsets.all(8.0),
                        child: ClipRRect(
                            borderRadius:
                                const BorderRadius.all(Radius.circular(4.0)),
                            child: image == null
                                ? Container(
                                    color: Colors.black, width: 28, height: 28)
                                : Image.memory(
                                    image,
                                    width: 28,
                                    height: 28,
                                  ))),
                    const Text(":", style: TextStyle(fontSize: 24)),
                    Padding(
                      padding: const EdgeInsets.all(8.0),
                      child: Text(
                          style: const TextStyle(fontSize: 24),
                          prediction == null ? "--" : prediction.toString()),
                    ),
                    ElevatedButton(
                        onPressed: () async {
                          ref
                              .read(positionProvider.notifier)
                              .update((state) => []);
                          ref
                              .read(predictionProvider.notifier)
                              .update((state) => null);
                          ref
                              .read(imageProvider.notifier)
                              .update((state) => null);
                        },
                        child: const Text(
                          "Clear",
                          style: TextStyle(color: Colors.black),
                        )),
                  ],
                );
              })
            ],
          ),
        ),
      ),
    );
  }
}
