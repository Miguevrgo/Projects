import 'package:algori/pages/settings_page.dart';
import 'package:flutter/material.dart';

// Views
import 'pages/home_page.dart';
// Navigation bar
import 'app_bottom_nav.dart';

void main() {
  runApp(MyApp());
}

ValueNotifier<ThemeMode> themeNotifier = ValueNotifier(ThemeMode.light);

class MyApp extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return ValueListenableBuilder<ThemeMode>(
      valueListenable: themeNotifier,
      builder: (context, currentTheme, _) {
        return MaterialApp(
          debugShowCheckedModeBanner: false,
          title: 'Algorithm Visualizer',
          theme: ThemeData.light(),
          darkTheme: ThemeData.dark(),
          themeMode: currentTheme,
          home: MainApp(),
        );
      },
    );
  }
}

class MainApp extends StatefulWidget {
  @override
  _MainAppState createState() => _MainAppState();
}

class _MainAppState extends State<MainApp> {
  int _selectedIndex = 0;

  // List of pages
  final List<Widget> _pages = [
    SettingsPage(),
    HomePage(),
    SizedBox.shrink(),
  ];

  // Change current page
  void _onItemTapped(int index) {
    if (index == 2) {
      setState(() {
        themeNotifier.value = themeNotifier.value == ThemeMode.light
            ? ThemeMode.dark
            : ThemeMode.light;
      });
    } else {
      setState(() {
        _selectedIndex = index;
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
        body: _pages[_selectedIndex],
        bottomNavigationBar: AppBottomNav(
          currentIndex: _selectedIndex,
          onItemTapped: _onItemTapped,
        ));
  }
}
