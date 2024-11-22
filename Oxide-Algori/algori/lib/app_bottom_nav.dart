import 'package:flutter/material.dart';

class AppBottomNav extends StatelessWidget {
  final int currentIndex;
  final Function(int) onItemTapped;
  final ThemeMode currentTheme;

  AppBottomNav(
      {required this.currentIndex,
      required this.onItemTapped,
      required this.currentTheme});

  @override
  Widget build(BuildContext context) {
    return BottomNavigationBar(
      currentIndex: currentIndex,
      onTap: onItemTapped,
      items: [
        const BottomNavigationBarItem(
            icon: Icon(Icons.settings), label: 'Settings'),
        const BottomNavigationBarItem(icon: Icon(Icons.home), label: 'Home'),
        BottomNavigationBarItem(
            icon: currentTheme == ThemeMode.light
                ? Icon(Icons.light_mode)
                : Icon(Icons.dark_mode),
            label: 'Theme'),
      ],
    );
  }
}
