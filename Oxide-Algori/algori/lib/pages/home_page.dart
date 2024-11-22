import 'package:flutter/material.dart';
import 'package:flutter_svg/flutter_svg.dart'; // Svg images

class Tuple<T1, T2> {
  final T1 item1;
  final T2 item2;

  Tuple(this.item1, this.item2);
}

class HomePage extends StatelessWidget {
  final List<Tuple<String, String>> algorithms = [
    Tuple("Array", "assets/images/array.svg"),
    Tuple("Tree", "assets/images/tree.svg"),
    Tuple("Linked List", "assets/images/linked_list.svg"),
    Tuple("Hash Table", "assets/images/hash_table.svg"),
    Tuple("Bit Manipulation", "assets/images/bitwise.svg"),
    Tuple("Math", "assets/images/math.svg"),
    Tuple("Stack", "assets/images/stack.svg"),
    Tuple("Queue", "assets/images/queue.svg"),
    Tuple("Heap", "assets/images/heap.svg"),
    Tuple("Trie", "assets/images/trie.svg"),
    Tuple("Dijkstra", "assets/images/tree.svg"),
  ];

  @override
  Widget build(BuildContext context) {
    double screenWidth = MediaQuery.of(context).size.width;
    int columns = screenWidth < 600 ? 2 : 4;

    return Scaffold(
      appBar: AppBar(title: Text('Home Page')),
      body: Padding(
        padding: const EdgeInsets.all(8.0),
        child: GridView.builder(
          gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
            crossAxisCount: columns,
            crossAxisSpacing: 16.0,
            mainAxisSpacing: 16.0,
            childAspectRatio:
                1, // Ajusta el aspecto para hacer los elementos más pequeños
          ),
          itemCount: algorithms.length,
          itemBuilder: (context, index) {
            final algorithm = algorithms[index];
            return GestureDetector(
              onTap: () {
                Navigator.push(
                  context,
                  MaterialPageRoute(
                    builder: (context) => AlgorithmDetailPage(
                      name: algorithm.item1,
                      image: algorithm.item2,
                    ),
                  ),
                );
              },
              child: Card(
                shape: RoundedRectangleBorder(
                  borderRadius: BorderRadius.circular(16.0),
                ),
                elevation: 4,
                child: Column(
                  children: [
                    Expanded(
                      child: ClipRRect(
                        borderRadius: BorderRadius.circular(16.0),
                        child: SvgPicture.asset(
                          algorithm.item2,
                          fit: BoxFit.contain,
                          width: double.infinity,
                        ),
                      ),
                    ),
                    Padding(
                      padding: const EdgeInsets.all(8.0),
                      child: Text(
                        algorithm.item1,
                        style: TextStyle(
                            fontSize: 14, fontWeight: FontWeight.bold),
                        textAlign: TextAlign.center,
                      ),
                    ),
                  ],
                ),
              ),
            );
          },
        ),
      ),
    );
  }
}

class AlgorithmDetailPage extends StatelessWidget {
  final String name;
  final String image;

  AlgorithmDetailPage({required this.name, required this.image});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: Text(name)),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            SvgPicture.asset(image, width: 200, height: 200),
            SizedBox(height: 16),
            Text(
              "Detalles sobre $name",
              style: TextStyle(fontSize: 20),
            ),
          ],
        ),
      ),
    );
  }
}
